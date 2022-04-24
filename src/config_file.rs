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

//! Defines the structure of the TOML configuration file.

use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;
use std::string::String;

pub(crate) type EnvironmentVariables = IndexMap<String, EnvVariableOption>;

/// The TOML file to load environment variables from.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub(crate) struct ConfigFile {
    #[serde(flatten)]
    pub(crate) vars: EnvironmentVariables,

    // Deprecated
    pub(crate) shell: Option<HashMap<String, IndexMap<String, EnvVariableValue>>>,
}

impl ConfigFile {
    pub(crate) fn load(toml_string: String) -> Result<ConfigFile, toml::de::Error> {
        toml::from_str(&*toml_string)
    }
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub(crate) enum EnvVariableOption {
    Specific(IndexMap<String, EnvVariableValue>),
    General(EnvVariableValue),
}

/// Enum of possible environment variable value types.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub(crate) enum EnvVariableValue {
    String(String),
    Array(Vec<String>),
    Set(bool),
}

#[cfg(test)]
mod tests {
    use indexmap::indexmap;
    use indoc::indoc;
    use pretty_assertions::assert_eq;

    use super::*;
    use EnvVariableOption::*;

    /// A function to get a ConfigFile from a str more easily.
    fn to_config(toml_str: &str) -> ConfigFile {
        toml::from_str(toml_str).unwrap()
    }

    /// Used to compare TOML to its expected representation in all other tests.
    fn assert_config_value(toml_str: &str, map: EnvironmentVariables) {
        assert_eq!(to_config(toml_str).vars, map);
    }

    #[test]
    fn test_config_file_load_string() {
        assert_config_value(
            indoc! {r#"
                FOO = "Bar"
            "#},
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("Bar".into())),
            },
        )
    }

    #[test]
    fn test_config_file_load_path() {
        assert_config_value(
            indoc! {r#"
                PATH = ["/usr/local/bin", "/usr/bin", "/bin", "/usr/sbin", "/sbin"]
            "#},
            indexmap! {
                "PATH".into() => General(EnvVariableValue::Array(vec![
                    "/usr/local/bin".into(),
                    "/usr/bin".into(),
                    "/bin".into(),
                    "/usr/sbin".into(),
                    "/sbin".into(),
                ])),
            },
        )
    }

    #[test]
    fn test_config_file_load_specific() {
        assert_config_value(
            indoc! {r#"
                ONLY_FOR_BASH.bash = "Do people read test cases?"
            "#},
            indexmap! {
                "ONLY_FOR_BASH".into() => Specific(indexmap! {
                    "bash".into() => EnvVariableValue::String("Do people read test cases?".into()),
                }),
            },
        )
    }

    #[test]
    fn test_config_file_load_specific_other() {
        assert_config_value(
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
        )
    }

    #[test]
    fn test_config_file_load_specific_other_alt() {
        assert_config_value(
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
        )
    }

    #[test]
    fn test_config_file_everything() {
        assert_config_value(
            indoc! {r#"
                FOO = 'bar'
                BAZ.zsh = 'zž'
                BAZ.fish = ['gone', 'fishing']
                BAZ._ = 'other'
            "#},
            indexmap! {
                "FOO".into() => General(EnvVariableValue::String("bar".into())),
                "BAZ".into() => Specific(indexmap! {
                    "zsh".into() => EnvVariableValue::String("zž".into()),
                    "fish".into() => EnvVariableValue::Array(vec!["gone".into(), "fishing".into()]),
                    "_".into() => EnvVariableValue::String("other".into()),
                })
            },
        )
    }
}
