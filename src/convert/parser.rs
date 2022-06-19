// Copyright 2022 Ethan Kinnear
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Parse a value of a configuration into a vector of parts.

use std::ops::RangeInclusive;
use std::process::exit;

/// Represents part of the value of a shell script.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValuePartKind {
    Literal,       // A literal value.
    ShellVariable, // A shell variable. Represented in the toml file as $val or ${val}
    ShellCommand,  // A shell command. Represented in the toml file as $(command)
    Home,          // A home directory. Represented as ~ normally, but sometimes as ~name
}

/// A part of a environment variable value with a specific function,
/// as determined by its kind (ValuePartKind).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValuePart {
    pub kind: ValuePartKind, // The function of this specific part.
    pub value: String,       // The contents of the part.
}

impl ValuePart {
    fn new(kind: ValuePartKind) -> Self {
        Self {
            kind,
            value: String::new(),
        }
    }

    /// Add a char to the value.
    fn push(&mut self, char: char) {
        self.value.push(char);
    }

    // Add self to a vector (res).
    fn push_self_to(self, res: &mut Vec<Self>) {
        // Display the current section if logging is trace level.
        trace!("{:?}", self);
        // Don't add ValueParts with nothing added to them.
        // The exception is `ValuePartKind::Home`, which can be valid with no additional value.
        if !self.value.is_empty() || self.kind == ValuePartKind::Home {
            res.push(self);
        }
    }

    /// Add self to a vector (res), and then return a new ValuePart with the specified kind.
    fn push_self_and_new(self, res: &mut Vec<Self>, kind: ValuePartKind) -> Self {
        self.push_self_to(res);
        Self::new(kind)
    }
}

/// The current state that the `parse_value` parser is in.
/// Used to behave differently based on previous special characters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValueParsingState {
    Normal,              // Behave normally.
    Backslash,           // The previous character was a backslash (\).
    BeginShellStatement, // The previous characters was a dollar sign ($).
}

/// Parses a value line in the configuration into a vector of `ValuePart`s.
/// This allows for the different parts of a value to be converted into a usable form for any shell.
///
/// May exit the cli app early if a non-recoverable parse occurs.
pub fn parse_value(raw_value: &str) -> Vec<ValuePart> {
    use ValueParsingState::*;
    use ValuePartKind::*;

    // If the raw_value is an empty string, simply return vector with a single empty `ValuePart`.
    // This removes the need to work around panics that are caused by an empty `raw_value`,
    // and also makes it so that the result is a vector with one empty element instead of an empty
    // vector. Because most shells expect some value, it is important to return at least one
    // element, even if the element itself has an empty value.
    if raw_value.is_empty() {
        return Vec::from([ValuePart::new(Literal)]);
    }

    // Result buffer. This is the vector that is returned if the `raw_value` is not empty.
    let mut res = Vec::new();

    // Parsing state for giving context for how to handle different characters.
    let mut parsing_mode = Normal;

    // There are some cases where `ValuePart` is not terminated by a specific character or beginning
    // of another part, but instead ends by itself when a separation character is encountered.
    //
    // For example, `raw_values` like "${XDG_DATA_HOME}/cargo" have a terminating character for the
    // `ValuePartKind::ShellVariable`, but "$XDG_DATA_HOME/cargo" simply ends once it reaches the
    // forward-slash character without consuming the character itself.
    //
    // Note that the "}" character in the first example will not be included as a character in any
    // string, while the "/" character will. This is the distinction that this variable tracks.
    //
    // What counts as a valid character for a non-bracketed shell variable (eg. $CARGO_HOME) or
    // username expansion (eg. ~superatomic) is defined in the `is_valid_shell_variable` function.
    //
    // When this variable is true, before matching the current character, the current part will be
    // ended if the current character is not a valid character to continue the part.
    //
    // This allows for characters like "/" to begin an arbitrarily new part (of an unknown variant)
    // while also being parsed as a character themselves.
    let mut parse_until_shell_separator = false;

    // The current part that is being added.
    // Defaults to `ValuePartKind::Literal`, even if something else immediately changes it.
    let mut current_part = ValuePart::new(Literal);

    // The index that we started a part at.
    // Used only for displaying error messages and nothing else.
    let mut shell_statement_starting_index = 0;

    // Loop through each character and react accordingly.
    //
    // This code enumerates through the index so it can be used to display error messages.
    // Besides showing error messages, the index is not used.
    for (index, symbol) in raw_value.chars().enumerate() {
        // If the parser continues a part until it reaches a non-valid character to end the part on,
        // as we do for non-bracketed shell variables (eg. $VAR) and home tildes (eg. ~username),
        // we need to end the state *before* we match it, so we don't skip the character that ended
        // the loop.
        //
        // For example, if the entire value was "$CARGO_HOME/bin", a naive parser that just matched
        // the next non-valid character (which is determined by `fn is_valid_shell_variable(char)`)
        // would do the following:
        //
        //   - See '$' and move change from ValuePartKind::Literal to ValuePartKind::ShellVariable
        //   - See "CARGO_HOME" one character at a time and add them to the current_part.
        //   - See '/', detect that it isn't valid, and change the ValuePartKind to Literal
        //
        // However, while we can immediately add the '/' character to the new `current_part`,
        // there is no way of knowing if the ValuePartKind should be immediately replaced with a
        // different variant.
        //
        // So this example would function because the following segment to `$CARGO_HOME` is `/bin`,
        // which is a ValuePartKind::Literal. However, something like "$CARGO_HOME$(uname -s)" would
        // consume the '$' and fail to match the new state (from `ValueParsingState::Normal` to
        // `ValueParsingState::BeginShellStatement`), and would register it as if the '$' didn't
        // exist.
        //
        // So this just checks for the `parse_until_shell_separator` state and if the current
        // character would end the part. If it would, we end the part here so we can still match it.
        if parse_until_shell_separator && !is_valid_shell_variable(symbol) {
            // Finish this part and create a new part.
            // The `ValuePartKind::Literal` variant might immediately be changed,
            // but this just sets it back to the initial default state.
            //
            // In other words, if it's actually a `Literal`, it'll work,
            // and if it's not it'll automatically be changed because that's the normal behavior
            // of the following match statement anyway.
            current_part = current_part.push_self_and_new(&mut res, Literal);
            parse_until_shell_separator = false; // Make sure that we've ended the state!
        }

        // Main logic for determining what characters to add to the current part,
        // what state to change, and when to change the current part.
        match (&parsing_mode, symbol, &current_part.kind) {
            // =========================== //
            // NORMAL VALUE PARSING STATES //
            // =========================== //

            // Backslash escapes are not allowed inside of shell variables.
            //
            // Display an error message stating the problem and exit.
            (Normal, '\\', ShellVariable) => {
                print_parse_error(
                    log::Level::Error,
                    raw_value,
                    index..=index,
                    "Fatal: Shell variables cannot contain backslashes",
                );
                exit(exitcode::DATAERR);
            }

            // A backslash in normal conditions begins a backslash escape sequence
            (Normal, '\\', _) => parsing_mode = Backslash,

            // A '$' character puts the parser in the state where it looks for the following
            // character and behaves accordingly.
            //
            // An opening parenthesis will change to a ShellCommand, and a opening curly bracket or
            // valid shell variable character will change the mode to a ShellVariable.
            //
            // For more, see the `(BeginShellStatement, char, Literal)` match.
            (Normal, '$', Literal) => {
                parsing_mode = BeginShellStatement;

                // Also set the index for the beginning of the shell statement, so that an error
                // message can be displayed if a closing bracket or parenthesis is never used.
                shell_statement_starting_index = index;
            }

            // If there is a tilde at the start of the string, we switch to a home tilde mode.
            // Otherwise, the tilde character behaves normally.
            (Normal, '~', Literal) if res.is_empty() && current_part.value.is_empty() => {
                // End the current part (which is guaranteed to be empty) and begin a new `Home` one
                current_part = ValuePart::new(Home);

                // There is no character that changes the mode from `ValuePartKind::Home` back to
                // `ValuePartKind::Literal`. Instead, it changes back once a non-valid character for
                // it is encountered.
                //
                // For more context, see the `if` statement directly above this `match` statement.
                parse_until_shell_separator = true;
            }

            // Note that this is the union of two different match conditions!
            //
            // Matches an ending bracket/parenthesis, and returns the mode to `Literal`.
            // Does not trigger if `parse_until_shell_separator` is `true`, because that means the
            // current `ShellVariable` mode was not started by a opening '{' and as such cannot be
            // ended by one.
            //
            // Instead, it will automatically end once it reaches a character that isn't valid.
            // For more context, see the `if` statement directly above this `match` statement.
            (Normal, ')', ShellCommand) | (Normal, '}', ShellVariable)
                if !parse_until_shell_separator =>
            {
                current_part = current_part.push_self_and_new(&mut res, Literal);
            }

            // Make sure to disallow character that are not allowed as shell variables.
            //
            // This disallows for things like the following:
            //
            //   - ${VAR!}
            //   - ${"VAR"}
            //   - ${;VAR}
            //   - etc.
            //
            // All of these characters are not allowed as the names of shell environment variables,
            // so it's better to catch them here rather then have the resulting shell script that is
            // generated be broken.
            //
            // This does not match backslashes, which are handled in a previous match.
            //
            // This branch will only match if a bracketed shell variable syntax is used (eg. syntax
            // like `${VAR}`), as opposed to non-bracketed shell variables, which will simply end
            // their part.
            //
            // Display an error message stating the problem and exit.
            (Normal, char, ShellVariable) if !is_valid_shell_variable(char) => {
                print_parse_error(
                    log::Level::Error,
                    raw_value,
                    index..=index,
                    "Fatal: Shell variables cannot contain this character",
                );
                exit(exitcode::DATAERR);
            }

            // Otherwise, if none of the special conditions above were met in the `Normal` state,
            // just append the character to the `current_part`.
            (Normal, char, _) => current_part.push(char),

            // ========================== //
            // OTHER VALUE PARSING STATES //
            // ========================== //

            // If a character is escaped in a backslash, the result is handled here.
            // This mode bypasses the special handling of the `ValueParsingState::Normal` state.
            (Backslash, char, part) => {
                // Is the character a valid escape?
                match (part, char) {
                    // If it is a valid escape sequence, add its literal value.
                    (_, '\\') | (Literal, '$') | (ShellCommand, '(' | ')') => {
                        current_part.push(char)
                    }

                    // Otherwise, display an error message, and add the literal value along with the
                    // preceding backslash.
                    _ => {
                        // Be more specific about why the character cannot be escaped, if possible.
                        let problem = if "$()".contains(char) {
                            "You don't need to escape this value here"
                        } else {
                            "Not a valid escape character"
                        };

                        // Display an error message explaining that the backslash was not valid.
                        print_parse_error(
                            log::Level::Warn,
                            raw_value,
                            (index - 1)..=index,
                            problem,
                        );

                        // Add the literal value.
                        current_part.push('\\');
                        current_part.push(char);
                    }
                }
                // Return to normal parsing since the backslash-escaped character has been escaped.
                parsing_mode = Normal;
            }

            // Handle what happens when a non-escaped '$' character was the previous character.
            (BeginShellStatement, char, Literal) => {
                match char {
                    // Shell Command. $(...)
                    '(' => {
                        current_part = current_part.push_self_and_new(&mut res, ShellCommand);
                    }

                    // Bracket wrapped Shell Variable ${...}
                    '{' => {
                        current_part = current_part.push_self_and_new(&mut res, ShellVariable);
                    }

                    // Non-wrapped Shell Variable $...
                    char if is_valid_shell_variable(char) => {
                        current_part = current_part.push_self_and_new(&mut res, ShellVariable);
                        current_part.push(char);
                        // Automatically end this part once a character that is not valid in a shell
                        // variable occurs.
                        parse_until_shell_separator = true;
                    }

                    // Invalid subsequent character after the '$'.
                    _ => {
                        print_parse_error(
                            log::Level::Warn,
                            raw_value,
                            (index - 1)..=index,
                            "Inline shell variables cannot begin with this character",
                        );

                        // Add the literal value instead of starting a new part.
                        current_part.push('$');
                        current_part.push(char);
                    }
                }

                parsing_mode = Normal;
            }

            // A `ValueParsingState::BeginShellStatement` shouldn't be able to occur unless the
            // `current_part.kind` is `ValuePartKind::Literal`, so this code is unreachable.
            (BeginShellStatement, ..) => unreachable!(),
        }
    }

    // Index for the end of the `raw_value` string to pass to `print_parse_error` if the ending
    // resulted in an invalid state.
    //
    // This function will exit early if `raw_value` is empty, so this has no risk of panicking.
    let end_index = (raw_value.len() - 1)..=(raw_value.len() - 1);
    // Display an error message if an unescaped backslash or dollar-sign was the final character.
    // If it was, the program doesn't have to exit. Instead, it will just add the literal value and
    // send a warning message to fix the problem immediately.
    //
    // In either case, when an invalid ending does occur, an error message will be displayed, and
    // the problematic character will be treated as if it was escaped.
    match parsing_mode {
        // `raw_value` ended with an unescaped backslash
        Backslash => {
            print_parse_error(
                log::Level::Warn,
                raw_value,
                end_index,
                "Backslash was not escaped",
            );
            current_part.push('\\');
        }
        // `raw_value` ended with an unescaped dollar sign
        BeginShellStatement => {
            print_parse_error(
                log::Level::Warn,
                raw_value,
                end_index,
                "Unused final \"$\". Use \"\\$\" instead",
            );
            current_part.push('$');
        }

        // Otherwise, it's fine.
        Normal => {}
    }

    // Display an error message if a shell statement that must be closed is not closed.
    // For example, these statements will cause the following error message to occur:
    //
    //   $(which micro
    //   ${PATH
    //
    // To fix these, all that would be needed would be to add the closing ) or } to the end.
    // However, syntax like $PATH or ~user is still okay, so we only display this error if the
    // current_part.kind isn't a Literal AND if we aren't in the `parse_until_shell_separator` mode.
    if current_part.kind != Literal && !parse_until_shell_separator {
        print_parse_error(
            log::Level::Error,
            raw_value,
            shell_statement_starting_index..=(raw_value.len() - 1),
            "Fatal: Unclosed shell statement!",
        );

        // There is no easy way to recover from this problem, so exit the app with a non-zero exit
        // code after displaying an error message.
        exit(exitcode::DATAERR);
    }

    // Add the final part to the return value
    current_part.push_self_to(&mut res);

    res
}

/// A function that determines what characters are allowed inside of a shell variable.
fn is_valid_shell_variable(char: char) -> bool {
    char.is_alphanumeric() || char == '_'
}

/// Display an error message for an invalid string pointing at the problematic section.
///
/// This function is smart about where it puts the error message in relation to the arrows that
/// point at the error.
fn print_parse_error(
    log_level: log::Level,
    line: &str,
    index: RangeInclusive<usize>,
    error_description: &str,
) {
    if log::log_enabled!(log_level) {
        // The arrows that point at the section that is a problem.
        let error_arrows = &index.clone().map(|_| '^').collect::<String>();

        // The lengths of various sections.
        let error_msg_len = error_description.len();
        let entire_line_len = line.len();
        let pad_len = *index.start();
        let error_len = index.count();

        // Get the full error message to display, which consists of the error message and the arrows
        // that point at the problematic section of the line.
        let full_error_message = if pad_len > error_msg_len {
            // SOME STRING THAT PARSED INCORRECTLY HERE
            //                        SOME MESSAGE ^^^^

            // The padding used here is different. In this rendering of the error message, there is
            // an extra space between the error message and the arrows. To make sure that the arrows
            // point at the correct location, the whitespace padding needs to be one less.
            let pad_inline = &(error_msg_len..pad_len - 1)
                .map(|_| ' ')
                .collect::<String>();
            [pad_inline, error_description, " ", error_arrows].concat()
        } else if pad_len + error_len + error_msg_len < entire_line_len {
            // SOME STRING THAT PARSED INCORRECTLY HERE
            //             ^^^^ SOME MESSAGE

            let pad_inline = &(0..pad_len).map(|_| ' ').collect::<String>();
            [pad_inline, error_arrows, " ", error_description].concat()
        } else if pad_len + error_msg_len <= entire_line_len {
            // SOME STRING THAT PARSED INCORRECTLY HERE
            //                  ^^^^^^^^^^^^^^^^^^
            //                  SOME MESSAGE

            let pad = &(0..pad_len).map(|_| ' ').collect::<String>();
            [pad, error_arrows, "\n", pad, error_description].concat()
        } else {
            // SOME STRING THAT PARSED INCORRECTLY HERE
            //                  ^^^^^^^^^^^^^^^^^^
            //                        SOME MESSAGE

            let arrow_pad = &(0..pad_len).map(|_| ' ').collect::<String>();
            let msg_pad = &(error_msg_len..pad_len + error_len)
                .map(|_| ' ')
                .collect::<String>();
            [arrow_pad, error_arrows, "\n", msg_pad, error_description].concat()
        };

        // Use color in error messages.
        use colored::Colorize;

        // Finally, display the error message.
        log!(log_level, "{}\n{}\n", line, full_error_message.red());
    }
}

#[cfg(test)]
mod test_parsing {
    use super::*;
    use pretty_assertions::assert_eq;

    use ValuePartKind::*;

    /// Function providing shorthand to test the mapping of strings to their parsed representations.
    fn assert_parses(value: &str, res_rep: Vec<(ValuePartKind, &str)>) {
        // Map a simplified tuple representation of Vec<ValuePart> into a Vec<ValuePart>.
        // This allows us to keep the actual tests a lot clearer-looking,
        // while still converting it to the correct type.
        let res: Vec<ValuePart> = res_rep
            .iter()
            .map(|(kind, value)| ValuePart {
                kind: *kind,
                value: value.to_string(),
            })
            .collect();
        // Compute the value that we get.
        let parsed_value = parse_value(value);

        // Now check to see if it's what we expected.
        assert_eq!(parsed_value, res, "Check how {} parses", value);
    }

    #[test]
    fn test_literal() {
        assert_parses("Hello World!", vec![(Literal, "Hello World!")])
    }

    #[test]
    fn test_variable() {
        assert_parses("${PATH}", vec![(ShellVariable, "PATH")])
    }

    #[test]
    fn test_command() {
        assert_parses("$(tty)", vec![(ShellCommand, "tty")])
    }

    #[test]
    fn test_tilde_blank() {
        assert_parses(
            "~/.config/xshe.toml",
            vec![(Home, ""), (Literal, "/.config/xshe.toml")],
        )
    }

    #[test]
    fn test_tilde_named() {
        assert_parses(
            "~superatomic/.config/xshe.toml",
            vec![(Home, "superatomic"), (Literal, "/.config/xshe.toml")],
        )
    }

    #[test]
    fn test_multiple() {
        assert_parses(
            "The $(pwd) might be on the ${PATH}",
            vec![
                (Literal, "The "),
                (ShellCommand, "pwd"),
                (Literal, " might be on the "),
                (ShellVariable, "PATH"),
            ],
        )
    }

    #[test]
    fn test_escape_dollar_sign() {
        assert_parses(r"\$(tty)", vec![(Literal, r"$(tty)")])
    }

    #[test]
    fn test_escape_backslash() {
        assert_parses(r"$(echo '\\')", vec![(ShellCommand, r"echo '\'")])
    }

    #[test]
    fn test_escape_bracket() {
        assert_parses(r"$(echo '\)')", vec![(ShellCommand, r"echo ')'")])
    }

    #[test]
    fn test_escape_closing() {
        assert_parses(
            r"$(echo \)))",
            vec![(ShellCommand, "echo )"), (Literal, ")")],
        )
    }

    #[test]
    fn test_nesting() {
        // You shouldn't be able to nest special modes.
        assert_parses(r"$(echo ${$\(\)})", vec![(ShellCommand, "echo ${$()}")])
    }
}
