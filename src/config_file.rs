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

pub(crate) type EnvironmentVariables = IndexMap<String, EnvValue>;

/// The TOML file to load environment variables from.
#[derive(Deserialize, Debug)]
pub(crate) struct ConfigFile {
    #[serde(flatten)]
    pub(crate) vars: EnvironmentVariables,

    pub(crate) shell: HashMap<String, EnvironmentVariables>,
}

impl ConfigFile {
    pub(crate) fn load(toml_string: String) -> Result<ConfigFile, toml::de::Error> {
        toml::from_str(toml_string.as_str())
    }
}

/// Enum of possible environment variable value types.
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum EnvValue {
    String(String),
    Array(Vec<String>),
}
