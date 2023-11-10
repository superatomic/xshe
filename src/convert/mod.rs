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

//! Convert a mapping representation of toml-formatted data into an `eval`able shell script.

// Ignore ptr_arg Clippy warnings, as they are false positives.
// This will be fixed in 1.61.
// https://github.com/rust-lang/rust-clippy/pull/8271
#![allow(clippy::ptr_arg)]

mod parser;
use clap::ValueEnum;
use indexmap::IndexMap;
use std::string::String;

use crate::cli::Shell::{self, *};
use crate::convert::parser::ValuePartKind;
use crate::structure::{EnvVariableOption, EnvVariableValue, EnvironmentVariables};

/// Converts the hash table of `vars` into a script for the given `shell`.
pub(crate) fn to_shell_source(vars: &EnvironmentVariables, shell: &Shell) -> String {
    let outputs: Vec<String> = vars
        .iter()
        .filter_map(|(name, variable_option)| {
            // Check whether the current item is a single environment var or a table of
            // specific shells.
            match variable_option {
                EnvVariableOption::General(v) => Some(v),
                // If it is a shell specific choice, get the correct value for `shell`,
                // and then extract the `EnvVariableValue` if it exists and skip the value
                // if it does not
                EnvVariableOption::Specific(map) => value_for_specific(shell, map),
            }
            .map(|raw_value| process_variable(shell, name, raw_value))
        })
        .collect();
    outputs.join("\n") + "\n"
}

fn process_variable(shell: &Shell, name: &str, raw_value: &EnvVariableValue) -> String {
    // If the value of the environment variable is `false`,
    // then add the "unset" script line to the String and skip the rest of this function.
    let script_line = match raw_value {
        EnvVariableValue::Set(false) => add_script_line::unset_variable(shell, name),
        EnvVariableValue::Set(true) => add_script_line::set_variable(shell, name, "1", false),
        EnvVariableValue::String(string) => {
            let expanded_value = expand_value(string, shell);
            add_script_line::set_variable(shell, name, &expanded_value, false)
        }
        EnvVariableValue::Array(array_of_arrays) => {
            let flattened_array = array_of_arrays
                .iter()
                .flat_map(|array| array.iter())
                .map(|value| expand_value(value, shell))
                .collect::<Vec<String>>()
                .join(":");
            add_script_line::set_variable(shell, name, &flattened_array, false)
        }
        EnvVariableValue::Path(path) => {
            let path_string = path
                .iter()
                .map(|value| expand_value(value, shell))
                .collect::<Vec<String>>()
                .join(":");
            add_script_line::set_variable(shell, name, &path_string, true)
        }
    };
    script_line
}

// Module for adding a line to the script that will be sourced by the shell.
// Defines methods for adding the different types of lines.
mod add_script_line {
    use crate::cli::Shell::{self, *};

    pub fn set_variable(shell: &Shell, name: &str, value: &str, is_path: bool) -> String {
        // Log each processed variable
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "[Set]",
                false => "'Set'",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };

        // Select the correct form for the chosen shell.
        match shell {
            Bash | Zsh => format!("export {}={}", name, value),
            Fish => {
                let path_option = if is_path { " --path" } else { "" };
                format!("set -gx{} {} {}", path_option, name, value)
            }
        }
    }

    pub fn unset_variable(shell: &Shell, name: &str) -> String {
        trace!("Unset: {}", name);

        // Select the correct form for the chosen shell.
        match shell {
            Bash | Zsh => format!("unset {}", name),
            Fish => format!("set -ge {}", name),
        }
    }
}

/// Given a `shell` and a `map` of all specific shell options, get the correct shell `EnvVariableValue`.
/// Used by `to_shell_source` to filter the right `EnvVariableOption::Specific` for the current shell.
fn value_for_specific<'a>(
    shell: &Shell,
    map: &'a IndexMap<String, EnvVariableValue>,
) -> Option<&'a EnvVariableValue> {
    let binding = shell.to_possible_value()?;
    let shell_name = binding.get_name();
    map.get(shell_name).or_else(|| map.get("_"))
}

/// Expand the literal representation of a string in the toml to a value that can be parsed by the
/// given shell.
fn expand_value(value: &str, shell: &Shell) -> String {
    use ValuePartKind::*;

    let value_parts = parser::parse_value(value);

    // Pre-allocate space for the string
    let mut expanded_value = String::with_capacity(value.len() * 2);

    // Handle each part for the specified shell, and concatenate each part together.
    let shell_format = |kind: &ValuePartKind, value: &str| -> String {
        match (kind, shell) {
            (Literal, _) => single_quote(value, shell),

            (ShellVariable, Bash | Zsh | Fish) => format!(r#""${}""#, value),

            (ShellCommand, Bash | Zsh) => format!("$(eval {})", single_quote(value, shell)),
            (ShellCommand, Fish) => format!("(eval {})", single_quote(value, shell)),

            (Home, Bash | Zsh) => {
                format!("$(eval echo \"~{}\")", value)
            }
            (Home, Fish) => format!("(eval echo \"~{}\")", value),
        }
    };

    for parser::ValuePart { value, kind } in value_parts {
        expanded_value.push_str(&shell_format(&kind, &value));
    }
    expanded_value
}

// Surround a string in single quotes in a way that is best suited for a specific shell.
// Specifically, Fish shell has a simpler way of escaping single quotes in a single quoted string,
// while Bash and Zsh have to do it another way.
fn single_quote(string: &str, shell: &Shell) -> String {
    match shell {
        Bash | Zsh => string
            // Bash and Zsh can't escape any single quotes within a single-quoted string,
            // so whenever we encounter one we need to get the current string, begin a
            // double-quoted string containing the single quote, and then start a new
            // single-quoted string.
            .split('\'')
            .map(|v| {
                // If there are no characters between single quotes, or if a single quote begins or
                // ends a string, don't add an additional two single quotes (`''`) to the output.
                if v.is_empty() {
                    String::new()
                } else {
                    // There will be something between the quotes, so add the quotes!
                    format!("'{}'", v)
                }
            })
            .collect::<Vec<String>>()
            .join(r#""'""#),
        Fish => format!("'{}'", string.replace('\\', "\\\\").replace('\'', "\\'")),
    }
}

#[cfg(test)]
mod test_conversion {
    use super::*;
    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::structure::*;

    use EnvVariableOption::*;

    use indexmap::indexmap;
    use indoc::indoc;
    use maplit::hashmap;
    use std::collections::HashMap;

    // Used to test whether a file in toml format can be converted ito a correct `IndexMap`,
    // and then whether it can be converted from an `IndexMap`
    // into an output string to be sourced, for each shell.
    // This checks both the functionality of `convert::to_shell_source` (in this file)
    // and whether `config_file::ConfigFile` parses correctly.
    fn assert_convert(
        toml_str: &str,
        map: EnvironmentVariables,
        shell_sources: HashMap<Shell, &str>,
    ) {
        // Verify that the toml converts to the correct representation.
        assert_eq!(
            toml::from_str::<ConfigFile>(toml_str)
                .expect("Invalid toml formatting")
                .vars,
            map,
            "Compare toml data to its `EnvironmentVariables` representation",
        );

        // Verify that the representation translates into the correct shell-script, for each shell.
        for (shell, shell_source) in shell_sources {
            assert_str_eq!(
                // Trim the trailing newline(s), if they exist.
                to_shell_source(&map, &shell).trim_end_matches('\n'),
                shell_source.trim_end_matches('\n'),
                "Check for correct {:?} syntax",
                &shell,
            );
        }
    }

    #[test]
    fn test_convert_string() {
        assert_convert(
            // language=TOML
            r#"FOO = "Bar""#,
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("Bar".into())),
            },
            // language=sh
            hashmap! {
                Bash => r#"export FOO='Bar'"#,
                Zsh => r#"export FOO='Bar'"#,
                Fish => r#"set -gx FOO 'Bar'"#,
            },
        )
    }

    #[test]
    fn test_convert_path() {
        assert_convert(
            // language=TOML
            r#"PATH = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"]"#,
            indexmap! {
                "PATH".into() => General(EnvVariableValue::Path(vec![
                    "/usr/local/bin".into(),
                    "/usr/bin".into(),
                    "/bin".into(),
                    "/usr/sbin".into(),
                    "/sbin".into(),
                ])),
            },
            // language=sh
            hashmap! {
                Bash => r#"export PATH='/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin'"#,
                Zsh => r#"export PATH='/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin'"#,
                Fish => r#"set -gx --path PATH '/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin'"#,
            },
        )
    }

    #[test]
    fn test_convert_array() {
        assert_convert(
            // language=TOML
            r#"ARRAY = [["array_item_1", "array_item_2", "array_item_3"]]"#,
            indexmap! {
                "ARRAY".into() => General(EnvVariableValue::Array(vec![vec![
                    "array_item_1".into(),
                    "array_item_2".into(),
                    "array_item_3".into(),
                ]])),
            },
            // language=sh
            hashmap! {
                Bash => r#"export ARRAY='array_item_1':'array_item_2':'array_item_3'"#,
                Zsh => r#"export ARRAY='array_item_1':'array_item_2':'array_item_3'"#,
                Fish => r#"set -gx ARRAY 'array_item_1':'array_item_2':'array_item_3'"#,
            },
        )
    }

    #[test]
    fn test_convert_set() {
        assert_convert(
            // language=TOML
            "HOMEBREW_NO_ANALYTICS = true",
            indexmap! {
                "HOMEBREW_NO_ANALYTICS".into() => General(EnvVariableValue::Set(true)),
            },
            // language=sh
            hashmap! {
                Bash => r#"export HOMEBREW_NO_ANALYTICS=1"#,
                Zsh => r#"export HOMEBREW_NO_ANALYTICS=1"#,
                Fish => r#"set -gx HOMEBREW_NO_ANALYTICS 1"#,
            },
        )
    }

    #[test]
    fn test_convert_unset() {
        assert_convert(
            // language=TOML
            "HOMEBREW_NO_ANALYTICS = false",
            indexmap! {
                "HOMEBREW_NO_ANALYTICS".into() => General(EnvVariableValue::Set(false)),
            },
            // language=sh
            hashmap! {
                Bash => r#"unset HOMEBREW_NO_ANALYTICS"#,
                Zsh => r#"unset HOMEBREW_NO_ANALYTICS"#,
                Fish => r#"set -ge HOMEBREW_NO_ANALYTICS"#,
            },
        )
    }

    #[test]
    fn test_convert_specific() {
        assert_convert(
            // language=TOML
            r#"ONLY_FOR_BASH.bash = "Do people read test cases?""#,
            indexmap! {
                "ONLY_FOR_BASH".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::String("Do people read test cases?".into()),
                }),
            },
            // language=sh
            hashmap! {
                Bash => r#"export ONLY_FOR_BASH='Do people read test cases?'"#,
                Zsh => "",
                Fish => "",
            },
        )
    }

    #[test]
    fn test_convert_specific_other() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                SOME_VARIABLE.fish = "you're pretty"
                SOME_VARIABLE._ = '[ACCESS DENIED]'
            "#},
            indexmap! {
                "SOME_VARIABLE".into() => Specific(indexmap! {
                    "fish".into() => EnvVariableValue::String("you're pretty".into()),
                    "_".into() => EnvVariableValue::String("[ACCESS DENIED]".into()),
                })
            },
            // language=sh
            hashmap! {
                Bash => r#"export SOME_VARIABLE='[ACCESS DENIED]'"#,
                Zsh => r#"export SOME_VARIABLE='[ACCESS DENIED]'"#,
                Fish => r#"set -gx SOME_VARIABLE 'you\'re pretty'"#,
            },
        )
    }

    #[test]
    fn test_convert_specific_other_alt() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                [SOME_VARIABLE]
                fish = "you're pretty"
                _ = '[ACCESS DENIED]'

                [ANOTHER_VARIABLE]
                zsh = 'Zzz'
            "#},
            indexmap! {
                "SOME_VARIABLE".into() => Specific(indexmap! {
                    "fish".into() => EnvVariableValue::String("you're pretty".into()),
                    "_".into() => EnvVariableValue::String("[ACCESS DENIED]".into()),
                }),
                "ANOTHER_VARIABLE".into() => Specific(indexmap! {
                    "zsh".into() => EnvVariableValue::String("Zzz".into()),
                }),
            },
            // language=sh
            hashmap! {
                Bash => r#"export SOME_VARIABLE='[ACCESS DENIED]'"#,
                Zsh => indoc! (r#"
                    export SOME_VARIABLE='[ACCESS DENIED]'
                    export ANOTHER_VARIABLE='Zzz'
                "#),
                Fish => r#"set -gx SOME_VARIABLE 'you\'re pretty'"#,
            },
        )
    }

    #[test]
    fn test_shell_variables_inline() {
        assert_convert(
            // language=TOML
            r#"WHERE_THE_HEART_IS = "$HOME""#,
            indexmap! {
                "WHERE_THE_HEART_IS".into() => General(EnvVariableValue::String("$HOME".into())),
            },
            // language=sh
            hashmap! {
                Bash => r#"export WHERE_THE_HEART_IS="$HOME""#,
                Zsh => r#"export WHERE_THE_HEART_IS="$HOME""#,
                Fish => r#"set -gx WHERE_THE_HEART_IS "$HOME""#,
            },
        )
    }

    #[test]
    fn test_shell_variables_block() {
        assert_convert(
            // language=TOML
            r#"AN_EXAMPLE = "${HOME}less""#,
            indexmap! {
                "AN_EXAMPLE".into() => General(EnvVariableValue::String("${HOME}less".into())),
            },
            // language=sh
            hashmap! {
                Bash => r#"export AN_EXAMPLE="$HOME"'less'"#,
                Zsh => r#"export AN_EXAMPLE="$HOME"'less'"#,
                Fish => r#"set -gx AN_EXAMPLE "$HOME"'less'"#,
            },
        )
    }

    #[test]
    fn test_shell_commands() {
        assert_convert(
            // language=TOML
            r#"EDITOR = "$(which micro)""#,
            indexmap! {
                "EDITOR".into() => General(EnvVariableValue::String("$(which micro)".into())),
            },
            // language=sh
            hashmap! {
                Bash => r#"export EDITOR=$(eval 'which micro')"#,
                Zsh => r#"export EDITOR=$(eval 'which micro')"#,
                Fish => r#"set -gx EDITOR (eval 'which micro')"#,
            },
        )
    }

    #[test]
    fn test_shell_home_tilde() {
        assert_convert(
            // language=TOML
            r#"HOME = "~superatomic""#,
            indexmap! {
                "HOME".into() => General(EnvVariableValue::String("~superatomic".into())),
            },
            // language=sh
            hashmap! {
                Bash => r#"export HOME=$(eval echo "~superatomic")"#,
                Zsh => r#"export HOME=$(eval echo "~superatomic")"#,
                Fish => r#"set -gx HOME (eval echo "~superatomic")"#,
            },
        )
    }

    #[test]
    fn test_convert_character_escapes() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                MESSAGE = '''\$() is literal, and $(echo '\)') is escaped.'''
                FAVORITE_CHARACTER = '\\'
            "#},
            indexmap! {
                "MESSAGE".into() => General(EnvVariableValue::String(
                    r"\$() is literal, and $(echo '\)') is escaped.".into()
                )),
                "FAVORITE_CHARACTER".into() => General(EnvVariableValue::String(r"\\".into())),
            },
            // language=sh
            hashmap! {
                Bash => indoc!(r#"
                    export MESSAGE='$() is literal, and '$(eval 'echo '"'"')'"'")' is escaped.'
                    export FAVORITE_CHARACTER='\'
                "#),
                Zsh => indoc!(r#"
                    export MESSAGE='$() is literal, and '$(eval 'echo '"'"')'"'")' is escaped.'
                    export FAVORITE_CHARACTER='\'
                "#),
                Fish => indoc!(r#"
                    set -gx MESSAGE '$() is literal, and '(eval 'echo \')\'')' is escaped.'
                    set -gx FAVORITE_CHARACTER '\\'
                "#),
            },
        )
    }

    #[test]
    fn test_convert_escaping_quotes() {
        assert_convert(
            // language=TOML
            "MESSAGE = '''I 'love' books'''",
            indexmap! {
                "MESSAGE".into() => General(EnvVariableValue::String(r"I 'love' books".into())),
            },
            // language=sh
            hashmap! {
                Bash => r#"export MESSAGE='I '"'"'love'"'"' books'"#,
                Zsh => r#"export MESSAGE='I '"'"'love'"'"' books'"#,
                Fish => r"set -gx MESSAGE 'I \'love\' books'",
            },
        )
    }

    #[test]
    fn test_convert_everything() {
        assert_convert(
            // language=TOML
            indoc! {r#"
                # A collection of random things for testing.
                FOO = 'bar'
                BAZ.zsh = 'zž'
                BAZ.fish = ['gone', '$fishing']
                BAZ._ = '~other'
                ARRAY_TEST = [['$home', 'alone']]
                NOTHING_CHANGES = ['$home', 'alone']
                TTY = '$(tty)'
                THE_ECHO = '$(echo "\)")'
                XSHE_IS_THE_BEST = true # look, idk.
                # Return to poluting the home directory in bash
                XDG_CONFIG_HOME.bash = false
            "#},
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("bar".into())),
                "BAZ".into() => Specific(indexmap! {
                    "zsh".into() => EnvVariableValue::String("zž".into()),
                    "fish".into() => EnvVariableValue::Path(vec!["gone".into(), "$fishing".into()]),
                    "_".into() => EnvVariableValue::String("~other".into()),
                }),
                "ARRAY_TEST".into() => General(EnvVariableValue::Array(vec![vec![
                    "$home".into(),
                    "alone".into(),
                ]])),
                "NOTHING_CHANGES".into() => General(EnvVariableValue::Path(vec![
                    "$home".into(),
                    "alone".into(),
                ])),
                "TTY".into() => General(EnvVariableValue::String("$(tty)".into())),
                "THE_ECHO".into() => General(EnvVariableValue::String(r#"$(echo "\)")"#.into())),
                "XSHE_IS_THE_BEST".into() => General(EnvVariableValue::Set(true)),
                "XDG_CONFIG_HOME".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::Set(false),
                }),
            },
            // language=sh
            hashmap! {
                Bash => indoc! (r#"
                    export FOO='bar'
                    export BAZ=$(eval echo "~other")
                    export ARRAY_TEST="$home":'alone'
                    export NOTHING_CHANGES="$home":'alone'
                    export TTY=$(eval 'tty')
                    export THE_ECHO=$(eval 'echo ")"')
                    export XSHE_IS_THE_BEST=1
                    unset XDG_CONFIG_HOME
                "#),
                Zsh => indoc! (r#"
                    export FOO='bar'
                    export BAZ='zž'
                    export ARRAY_TEST="$home":'alone'
                    export NOTHING_CHANGES="$home":'alone'
                    export TTY=$(eval 'tty')
                    export THE_ECHO=$(eval 'echo ")"')
                    export XSHE_IS_THE_BEST=1
                "#),
                Fish => indoc! (r#"
                    set -gx FOO 'bar'
                    set -gx --path BAZ 'gone':"$fishing"
                    set -gx ARRAY_TEST "$home":'alone'
                    set -gx --path NOTHING_CHANGES "$home":'alone'
                    set -gx TTY (eval 'tty')
                    set -gx THE_ECHO (eval 'echo ")"')
                    set -gx XSHE_IS_THE_BEST 1
                "#),
            },
        )
    }
}
