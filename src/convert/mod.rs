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

use clap::ArgEnum;
use indexmap::IndexMap;
use std::string::String;

use crate::cli::Shell;
use crate::config_file::{EnvVariableOption, EnvVariableValue, EnvironmentVariables};
use crate::convert::parser::ValuePartKind;

/// Converts the hash table of `vars` into a script for the given `shell`.
pub(crate) fn to_shell_source(vars: &EnvironmentVariables, shell: &Shell) -> String {
    let mut output = String::new();
    for (name, variable_option) in vars {
        // Check whether the current item is a single environment var or a table of specific shells.
        let raw_value = match variable_option {
            EnvVariableOption::General(v) => v,

            // If it is a shell specific choice, get the correct value for `shell`, and then...
            EnvVariableOption::Specific(map) => match value_for_specific(shell, map) {
                Some(v) => v,     // ...extract the `EnvVariableValue` if it exists
                None => continue, // ...and skip the value if it does not.
            },
        };

        // Convert an array to a string, but log if it was an array.
        // Any arrays are treated as a path.
        let (value, is_path) = &match raw_value {
            // If the value of the environment variable is `false`,
            // then add the "unset" script line to the String and skip the rest of this function.
            EnvVariableValue::Set(false) => {
                add_script_line::unset_variable(&mut output, shell, name);
                continue;
            }

            EnvVariableValue::String(string) => (expand_value(string.as_str(), shell), false),
            EnvVariableValue::Set(true) => (String::from("1"), false),
            EnvVariableValue::Array(array) => {
                let v_expanded: Vec<String> = array
                    .iter()
                    .map(|value| expand_value(value, shell))
                    .collect();
                (v_expanded.join(":"), true)
            }
        };

        add_script_line::set_variable(&mut output, shell, name, value, is_path);
    }
    output
}

// Module for adding a line to the script that will be sourced by the shell.
// Defines methods for adding the different types of lines.
mod add_script_line {
    use super::Shell;

    pub fn set_variable(
        output: &mut String,
        shell: &Shell,
        name: &str,
        value: &str,
        is_path: &bool,
    ) {
        // Log each processed variable
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "[Set]",
                false => "'Set'",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };

        // Select the correct form for the chosen shell.
        *output += &match shell {
            Shell::Bash | Shell::Zsh => {
                format!("export {}=", name)
            }
            Shell::Fish => {
                // Add `--path` to the variable if the variable is represented as a list.
                let path = match is_path {
                    true => " --path",
                    false => "",
                };
                format!("set -gx{path} {} ", name, path = path)
            }
        };

        *output += value;
        *output += ";\n";
    }

    pub fn unset_variable(output: &mut String, shell: &Shell, name: &str) {
        // Log each processed variable
        trace!("Unset: {}", name);

        // Select the correct form for the chosen shell.
        *output += &match shell {
            Shell::Bash | Shell::Zsh => {
                format!("unset {}", name)
            }
            Shell::Fish => {
                format!("set -ge {}", name)
            }
        };

        *output += ";\n";
    }
}

/// Given a `shell` and a `map` of all specific shell options, get the correct shell `EnvVariableValue`.
/// Used by `to_shell_source` to filter the right `EnvVariableOption::Specific` for the current shell.
fn value_for_specific<'a>(
    shell: &Shell,
    map: &'a IndexMap<String, EnvVariableValue>,
) -> Option<&'a EnvVariableValue> {
    let shell_name = shell.to_possible_value()?.get_name();
    map.get(shell_name).or_else(|| map.get("_"))
}

/// Expand the literal representation of a string in the toml to a value that can be parsed by the
/// given shell.
fn expand_value(value: &str, shell: &Shell) -> String {
    use Shell::*;
    use ValuePartKind::*;

    let value_parts = parser::parse_value(value);

    let mut expanded_value = String::new();

    // Handle each part for the specified shell, and concatenate each part together.
    for part in value_parts {
        let parser::ValuePart { value, kind } = part;

        expanded_value += &match (kind, shell) {
            (Literal, _) => single_quote(value, shell),

            (ShellVariable, Bash | Zsh) => format!("${{{}}}", value),
            (ShellVariable, Fish) => format!("{{${}}}", value),

            (ShellCommand, Bash | Zsh) => format!("$(eval {})", single_quote(value, shell)),
            (ShellCommand, Fish) => format!("(eval {})", single_quote(value, shell)),

            (Home, Bash | Zsh) => {
                format!("$(eval echo \"~{}\")", value)
            }
            (Home, Fish) => format!("(eval echo \"~{}\")", value),
        }
    }
    expanded_value
}

// Surround a string in single quotes in a way that is best suited for a specific shell.
// Specifically, Fish shell has a simpler way of escaping single quotes in a single quoted string,
// while Bash and Zsh have to do it another way.
fn single_quote(string: String, shell: &Shell) -> String {
    use Shell::*;
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

    use crate::config_file::*;

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
                Shell::Bash => r#"export FOO='Bar';"#,
                Shell::Zsh => r#"export FOO='Bar';"#,
                Shell::Fish => r#"set -gx FOO 'Bar';"#,
            },
        )
    }

    #[test]
    fn test_convert_path() {
        assert_convert(
            // language=TOML
            r#"PATH = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"]"#,
            indexmap! {
                "PATH".into() => General(EnvVariableValue::Array(vec![
                    "/usr/local/bin".into(),
                    "/usr/bin".into(),
                    "/bin".into(),
                    "/usr/sbin".into(),
                    "/sbin".into(),
                ])),
            },
            // language=sh
            hashmap! {
                Shell::Bash => r#"export PATH='/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin';"#,
                Shell::Zsh => r#"export PATH='/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin';"#,
                Shell::Fish => r#"set -gx --path PATH '/usr/local/bin':'/usr/bin':'/bin':'/usr/sbin':'/sbin';"#,
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
                Shell::Bash => r#"export HOMEBREW_NO_ANALYTICS=1;"#,
                Shell::Zsh => r#"export HOMEBREW_NO_ANALYTICS=1;"#,
                Shell::Fish => r#"set -gx HOMEBREW_NO_ANALYTICS 1;"#,
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
                Shell::Bash => r#"unset HOMEBREW_NO_ANALYTICS;"#,
                Shell::Zsh => r#"unset HOMEBREW_NO_ANALYTICS;"#,
                Shell::Fish => r#"set -ge HOMEBREW_NO_ANALYTICS;"#,
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
                Shell::Bash => r#"export ONLY_FOR_BASH='Do people read test cases?';"#,
                Shell::Zsh => "",
                Shell::Fish => "",
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
                Shell::Bash => r#"export SOME_VARIABLE='[ACCESS DENIED]';"#,
                Shell::Zsh => r#"export SOME_VARIABLE='[ACCESS DENIED]';"#,
                Shell::Fish => r#"set -gx SOME_VARIABLE 'you\'re pretty';"#,
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
                Shell::Bash => r#"export SOME_VARIABLE='[ACCESS DENIED]';"#,
                Shell::Zsh => indoc! (r#"
                    export SOME_VARIABLE='[ACCESS DENIED]';
                    export ANOTHER_VARIABLE='Zzz';
                "#),
                Shell::Fish => r#"set -gx SOME_VARIABLE 'you\'re pretty';"#,
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
                Shell::Bash => r#"export WHERE_THE_HEART_IS=${HOME};"#,
                Shell::Zsh => r#"export WHERE_THE_HEART_IS=${HOME};"#,
                Shell::Fish => r#"set -gx WHERE_THE_HEART_IS {$HOME};"#,
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
                Shell::Bash => r#"export AN_EXAMPLE=${HOME}'less';"#,
                Shell::Zsh => r#"export AN_EXAMPLE=${HOME}'less';"#,
                Shell::Fish => r#"set -gx AN_EXAMPLE {$HOME}'less';"#,
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
                Shell::Bash => r#"export EDITOR=$(eval 'which micro');"#,
                Shell::Zsh => r#"export EDITOR=$(eval 'which micro');"#,
                Shell::Fish => r#"set -gx EDITOR (eval 'which micro');"#,
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
                Shell::Bash => r#"export HOME=$(eval echo "~superatomic");"#,
                Shell::Zsh => r#"export HOME=$(eval echo "~superatomic");"#,
                Shell::Fish => r#"set -gx HOME (eval echo "~superatomic");"#,
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
                Shell::Bash => indoc!(r#"
                    export MESSAGE='$() is literal, and '$(eval 'echo '"'"')'"'")' is escaped.';
                    export FAVORITE_CHARACTER='\';
                "#),
                Shell::Zsh => indoc!(r#"
                    export MESSAGE='$() is literal, and '$(eval 'echo '"'"')'"'")' is escaped.';
                    export FAVORITE_CHARACTER='\';
                "#),
                Shell::Fish => indoc!(r#"
                    set -gx MESSAGE '$() is literal, and '(eval 'echo \')\'')' is escaped.';
                    set -gx FAVORITE_CHARACTER '\\';
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
                Shell::Bash => r#"export MESSAGE='I '"'"'love'"'"' books';"#,
                Shell::Zsh => r#"export MESSAGE='I '"'"'love'"'"' books';"#,
                Shell::Fish => r#"set -gx MESSAGE 'I \'love\' books';"#,
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
                    "fish".into() => EnvVariableValue::Array(vec!["gone".into(), "$fishing".into()]),
                    "_".into() => EnvVariableValue::String("~other".into()),
                }),
                "TTY".into() => General(EnvVariableValue::String("$(tty)".into())),
                "THE_ECHO".into() => General(EnvVariableValue::String(r#"$(echo "\)")"#.into())),
                "XSHE_IS_THE_BEST".into() => General(EnvVariableValue::Set(true)),
                "XDG_CONFIG_HOME".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::Set(false),
                }),
            },
            // language=sh
            hashmap! {
                Shell::Bash => indoc! (r#"
                    export FOO='bar';
                    export BAZ=$(eval echo "~other");
                    export TTY=$(eval 'tty');
                    export THE_ECHO=$(eval 'echo ")"');
                    export XSHE_IS_THE_BEST=1;
                    unset XDG_CONFIG_HOME;
                "#),
                Shell::Zsh => indoc! (r#"
                    export FOO='bar';
                    export BAZ='zž';
                    export TTY=$(eval 'tty');
                    export THE_ECHO=$(eval 'echo ")"');
                    export XSHE_IS_THE_BEST=1;
                "#),
                Shell::Fish => indoc! (r#"
                    set -gx FOO 'bar';
                    set -gx --path BAZ 'gone':{$fishing};
                    set -gx TTY (eval 'tty');
                    set -gx THE_ECHO (eval 'echo ")"');
                    set -gx XSHE_IS_THE_BEST 1;
                "#),
            },
        )
    }
}
