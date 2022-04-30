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

use clap::{AppSettings, ArgEnum, ArgGroup, Parser, ValueHint};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::path::PathBuf;

// CLI Parser.
#[derive(Parser, Debug)]
#[clap(group = ArgGroup::new("mode").multiple(false))]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(version, arg_required_else_help = true)]
#[clap(after_help = "GitHub: https://github.com/superatomic/xshe")]
/// Cross-Shell Environment Variable Manager
///
/// Xshe sets shell environment variables across multiple shells with a single configuration file.
///
/// For more, go to https://github.com/superatomic/xshe#readme
pub struct Cli {
    /// The shell to generate a script for
    ///
    /// Outputs a runnable shell script for the specified shell.
    ///
    /// You can directly source these files in your shell.
    /// Read https://github.com/superatomic/xshe#sourcing-the-xshetoml-file for info.
    #[clap(arg_enum, index = 1)]
    pub shell: Shell,

    #[clap(group = "mode")]
    #[clap(short, long, parse(from_os_str), value_name = "FILE", value_hint = ValueHint::FilePath)]
    /// Specifies a custom location to read from
    ///
    /// This defaults to $XDG_CONFIG_HOME, or ~/.config if not set.
    ///
    /// Use --pipe or --file=- to pipe from stdin.
    ///
    /// The file must be in TOML format (https://toml.io/en/).")
    pub file: Option<PathBuf>,

    #[clap(group = "mode")]
    #[clap(short, long, value_name = "TOML", visible_alias = "toml", value_hint = ValueHint::Other)]
    /// Directly specify TOML to parse
    ///
    /// The passed string must be in TOML format (https://toml.io/en/).
    pub text: Option<String>,

    #[clap(group = "mode")]
    #[clap(short, long, value_name = "PIPE", alias = "stdin")]
    #[clap(verbatim_doc_comment)]
    /// Get TOML-formatted data from the standard input
    ///
    /// This is normally used to pass a configuration in from a pipe, like so:
    ///
    ///     cat xshe.toml | xshe bash
    ///
    /// The passed string must be in TOML format (https://toml.io/en/).
    #[clap(takes_value = false)]
    pub pipe: bool,

    #[clap(flatten)]
    pub verbose: Verbosity<WarnLevel>,
}

#[derive(ArgEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
}
