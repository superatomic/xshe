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

use clap::{ArgGroup, Parser, ValueEnum, ValueHint};
use clap_verbosity_flag::{Verbosity, WarnLevel};
use std::path::PathBuf;

// CLI Parser.
#[derive(Parser, Debug)]
#[command(about, long_about, author, version)]
#[command(arg_required_else_help = true, group = ArgGroup::new("mode").multiple(false))]
/// Cross-Shell Environment Variable Manager
///
/// Xshe sets shell environment variables across multiple shells with a single configuration file.
///
/// Full documentation can be found at https://xshe.superatomic.dev
/// or in the `docs/` source directory.
///
/// Source Repository: https://github.com/superatomic/xshe
pub struct Cli {
    /// The shell to generate a script for
    ///
    /// Outputs a runnable shell script for the specified shell.
    ///
    /// You can directly source these files in your shell.
    /// See <https://xshe.superatomic.dev/#/cli> for a detailed explanation.
    #[arg(value_enum, index = 1)]
    pub shell: Shell,

    #[arg(short, long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    #[arg(env = "XSHE_CONFIG", group = "mode")]
    /// Specifies a custom location to read from
    ///
    /// This defaults to $XDG_CONFIG_HOME, or ~/.config if not set.
    ///
    /// Use --pipe or --file=- to pipe from stdin.
    ///
    /// The file must be in TOML format (https://toml.io/en/).")
    pub file: Option<PathBuf>,

    #[arg(short, long, value_name = "TOML", value_hint = ValueHint::Other)]
    #[arg(visible_alias = "toml", group = "mode")]
    /// Directly specify TOML to parse
    ///
    /// The passed string must be in TOML format (https://toml.io/en/).
    pub text: Option<String>,

    #[arg(short, long, value_name = "PIPE", verbatim_doc_comment)]
    #[arg(alias = "stdin", group = "mode", action)]
    /// Get TOML-formatted data from the standard input
    ///
    /// This is normally used to pass a configuration in from a pipe, like so:
    ///
    ///     cat xshe.toml | xshe bash
    ///
    /// The passed string must be in TOML format (https://toml.io/en/).
    pub pipe: bool,

    #[clap(flatten)]
    pub verbose: Verbosity<WarnLevel>,
}

#[derive(ValueEnum, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Shell {
    Bash,
    Fish,
    Zsh,
}
