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

use atty::Stream;
use indexmap::IndexMap;
use serde::Deserialize;
use std::collections::HashMap;
use std::io::Read;
use std::path::Path;
use std::{fs, string::String};

pub(crate) type EnvironmentVariables = IndexMap<String, EnvValue>;

/// The TOML file to load environment variables from.
#[derive(Deserialize, Debug)]
pub(crate) struct ConfigFile {
    #[serde(flatten)]
    pub(crate) vars: EnvironmentVariables,

    pub(crate) shell: HashMap<String, EnvironmentVariables>,
}

impl ConfigFile {
    pub(crate) fn load(path: &Path) -> FileResult {
        // If there isn't an input stream or the file path is specifically "-", read from stdin.
        let use_stdin = atty::isnt(Stream::Stdin) || path.to_string_lossy() == "-";
        let toml_string = match use_stdin {
            true => Self::read_stdin(),

            // Otherwise, read the specified file.
            false => match fs::read_to_string(path) {
                Ok(valid_file) => valid_file,
                Err(_) => return FileResult::NotFound,
            },
        };

        // Parse
        let config = match toml::from_str(toml_string.as_str()) {
            Ok(config) => config,
            Err(_) => return FileResult::Invalid,
        };
        FileResult::Success(config)
    }

    fn read_stdin() -> String {
        //! Read all text from stdin.
        let mut buffer = String::new();
        std::io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
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
