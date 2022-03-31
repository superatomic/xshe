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

//! # Xshe
//! Set **Sh**ell **E**nvironment variables across multiple shells.
//! View the README (`README.md`) for more information.
//!
//! ## License
//!
//! Licensed under either of
//!
//! * Apache License, Version 2.0 (`LICENSE-APACHE.txt` or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license (`LICENSE-MIT.txt` or <http://opensource.org/licenses/MIT>)
//!
//! at your option.


mod cli;
mod config_file;

use std::{path::PathBuf, process::exit, string::String};
use clap::Parser;
use shellexpand;
use human_panic::setup_panic;

use crate::cli::{Cli, Shell};
use crate::config_file::{ConfigFile, FileResult, EnvValue, EnvironmentVariables};


fn main() {
    //! Main function.

    // Macro that gives user friendly panic reports.
    // Uses crate `human_panic`
    setup_panic!();

    // Parse the commandline options.
    let cli_options = Cli::parse();

    // Get the target TOML file with the environment variables.
    // If they are not manually set, use the XDG Base Directory Specification defaults.
    let raw_file: Option<PathBuf> = cli_options.file;
    let file = raw_file.clone().unwrap_or(get_file_path_default());

    // Load file data from the TOML file.
    let file_data = match ConfigFile::load(&file) {
        FileResult::Success(res) => { res }

        // The file doesn't exist!
        FileResult::NotFound => {

            // Select an informative help message.
            let help_msg = match raw_file {
                None => "Try setting `--file` to the correct location, or create the file.",
                Some(_) => "Is `--file` set correctly?",
            };

            // Display the error and exit.
            eprintln!(
                "The file {} does not exist or is a directory. {}",
                file.display(),
                help_msg,
            );
            exit(exitcode::NOINPUT)

        }

        // The file isn't a valid TOML format!
        FileResult::Invalid => {

            // Display the error and exit.
            eprintln!(
                "The file {} is not in a valid TOML format or is not in the form Xshe is expecting.",
                file.display(),
            );
            exit(exitcode::CONFIG)

        }

    };

    // Output the file data converted to the correct shell format to the standard output.
    let output = to_shell_source(file_data.vars, &cli_options.shell);
    print!("{}", output);

}



fn to_shell_source(vars: &EnvironmentVariables, shell: &Shell) -> String {
    //! Converts the hash table of `vars` into a script for the given `shell`.

    let mut output = String::new();
    for (name, raw_value) in vars {

        // Convert an array to a string, but log if it was an array.
        // Any array are treated
        let (value, is_path) = match raw_value.clone() {
            EnvValue::String(s) => (s, false),
            EnvValue::Array(v) => (v.join(":"), true),
        };

        // Replace TOML escape codes with the literal representation so that they are correctly used.
        // We need to do this for every code listed here: https://toml.io/en/v1.0.0#string
        let value = value
            .replace("\\", r"\\")  // Backslash - Must be first!
            .replace("\x08", r"\b")  // Backspace
            .replace("\t", r"\t")  // Tab
            .replace("\n", r"\n")  // Newline
            .replace("\x0C", r"\f")  // Form Feed
            .replace("\r", r"\r")  // Carriage Return
            .replace("\"", "\\\"");  // Double Quote

        match shell {
            Shell::Bash | Shell::Zsh => {
                output += &*format!("export {}=\"{}\";\n", name, value);
            }
            Shell::Fish => {
                // Add `--path` to the variable if the variable was represented as a list in the TOML.
                let path = match is_path { true => " --path", false => "" };
                output += &*format!("set -gx{path} {} \"{}\";\n", name, value, path = path);
            }
        };
    };
    output
}


fn get_file_path_default() -> PathBuf {
    //! Gets the default file path for `xshe.toml` if the `-f`/`--file` option is not set.

    // Get the directory where the file is located.
    // If `$XDG_CONFIG_HOME` is set, use that,
    // otherwise use the default location determined by the XDG Base Directory Specification.
    // Spec: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    let xdg_config_home: PathBuf = shellexpand::env("$XDG_CONFIG_HOME")
        .unwrap_or(shellexpand::tilde("~/.config"))
        .into_owned()
        .into();

    xdg_config_home.join("xshe.toml")

}
