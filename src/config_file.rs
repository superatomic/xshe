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


use std::{path::PathBuf, string::String, fs};
use indexmap::IndexMap;
use serde::Deserialize;

pub(crate) type EnvironmentVariables = IndexMap<String, EnvValue>;

/// The TOML file to load environment variables from.
#[derive(Deserialize, Debug)]
pub(crate) struct ConfigFile {
    #[serde(flatten)]
    pub(crate) vars: EnvironmentVariables,
}

impl ConfigFile {
    pub(crate) fn load(path: &PathBuf) -> FileResult {
        let toml_string = match fs::read_to_string(path) {
            Ok(valid_file) => valid_file,
            Err(_) => return FileResult::NotFound,
        };
        let config = match toml::from_str(toml_string.as_str()) {
            Ok(config) => config,
            Err(_) => return FileResult::Invalid,
        };
        FileResult::Success(config)
    }
}

/// Enum of possible environment variable value types.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum EnvValue {
    String(String),
    Array(Vec<String>),
}

/// Enum of all potential outcomes of attempting to read the configuration file.
pub(crate) enum FileResult {
    Success(ConfigFile),

    // Errors
    NotFound,
    Invalid,
}
