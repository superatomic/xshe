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

//! Defines the CLI interface for Xshe.

use clap::{ArgEnum, Parser};
use std::path::PathBuf;

/// CLI Parser.
#[derive(Parser)]
#[clap(version, about, long_about = None, arg_required_else_help = true)]
#[clap(after_help = "Documentation: https://lib.rs/crates/xshe\n\
GitHub: https://github.com/superatomic/xshe")]
pub(crate) struct Cli {
    #[clap(arg_enum)]
    #[clap(help = "The shell to generate a script")]
    pub shell: Shell,

    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    #[clap(help = "Specify a custom location to read from")]
    #[clap(long_help = "Specifies a custom location to read from. \n\
     This defaults to $XDG_CONFIG_HOME, or ~/.config if not set. \n\
     \n\
     The file must be in TOML format (https://toml.io/en/).")]
    pub file: Option<PathBuf>,

    #[clap(short, long, value_name = "TOML")]
    #[clap(help = "Directly specify TOML to parse", conflicts_with = "file")]
    #[clap(long_help = "Directly specify TOML to parse. \n\
     The passed string must be in TOML format (https://toml.io/en/).")]
    pub toml: Option<String>,
}

/// Enum of every supported shell.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub(crate) enum Shell {
    Bash,
    Zsh,
    Fish,
}
