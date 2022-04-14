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

#[macro_use]
extern crate log;

use clap::{ArgEnum, Parser};
use human_panic::setup_panic;
use std::io::{ErrorKind, Read};
use std::path::Path;
use std::{fs, path::PathBuf, process::exit, string::String};

use crate::cli::{Cli, Shell};
use crate::config_file::{ConfigFile, EnvValue, EnvironmentVariables};

fn main() {
    //! Main function.

    // Macro that gives user friendly panic reports.
    // Uses crate `human_panic`
    setup_panic!();

    // Parse the commandline options.
    let cli_options: Cli = Cli::parse();

    // Setup logging
    env_logger::Builder::new()
        .filter_level(cli_options.verbose.log_level_filter())
        .format_timestamp(None)
        .format_module_path(false)
        .format_target(false)
        .format_indent(Some(8)) // Aligns the first line with the other lines
        .init();

    // Pipe if `cli_options.pipe` is used or if `cli_options.file` is used and equal to "-".
    let pipe = cli_options.pipe
        || cli_options
            .file
            .as_ref()
            .map_or(false, |x| x.to_string_lossy() == "-");

    let (toml_string, file_name) = if pipe {
        // If --pipe was specified, use that as the direct toml.
        (read_stdin(), String::from("<STDIN>"))
    } else if let Some(toml) = cli_options.toml {
        // If --toml was specified, use that. Otherwise, get the file and read from it.
        (toml, String::from("<STRING>"))
    } else {
        // Otherwise, read from the chosen file.
        read_config_file(&cli_options)
    };
    info!("Reading file data from {}", file_name);

    // Load file data from the TOML file.
    let file_data = match ConfigFile::load(toml_string) {
        Ok(valid_toml) => valid_toml,

        // The file isn't a valid TOML format!
        Err(e) => {
            // Display the error and exit.
            error!(
                 "The file {} is not in a valid TOML format,\nor it is not in the form Xshe is expecting.",
                 file_name,
             );
            if let Some((line, column)) = e.line_col() {
                error!("Parse error at line {:}, column {:}", line + 1, column + 1);
            }
            exit(exitcode::CONFIG)
        }
    };

    let shell: Shell = cli_options.shell;

    // Deprecation warning
    if file_data.shell.is_some() {
        warn!(
            "Using [shell.SHELL] notation is deprecated\n\
            See https://github.com/superatomic/xshe/issues/30\n\
            To be removed in release v1.0.0"
        );
    }

    // Output the file data converted to the correct shell format to the standard output.
    let general_output = to_shell_source(&file_data.vars, &shell);
    print!("{}", general_output);

    // Output the any specific variables for the shell the same way, if they exist.
    // This behavior is deprecated.
    if let Some(specific_vars) = get_specific_shell(&shell, &file_data) {
        let shell_specific_output = to_shell_source(specific_vars, &shell);
        print!("{}", shell_specific_output);
    };
}

fn read_config_file(cli_options: &Cli) -> (String, String) {
    //! Read from the config file that should be selected based on the `--file` option.
    // Get the target TOML file with the environment variables.
    // If they are not manually set, use the XDG Base Directory Specification defaults.
    let raw_file: &Option<PathBuf> = &cli_options.file;
    let file = &raw_file.clone().unwrap_or_else(get_file_path_default);

    // Read the contents of the file.
    // Exit with an error message and exit code if an error occurs.
    let toml_string = match fs::read_to_string(file) {
        Ok(string) => string,
        Err(e) => exit(display_file_error(e.kind(), cli_options, file)),
    };
    (toml_string, file.display().to_string())
}

fn display_file_error(kind: ErrorKind, cli_options: &Cli, file: &Path) -> i32 {
    //! Displays a message and returns an specific error code for an general file read error.
    match kind {
        // The file doesn't exist!
        ErrorKind::NotFound => {
            // Select an informative help message.
            let help_msg = match cli_options.file {
                None => "Try setting `--file` to the correct location, or create the file.",
                Some(_) => "Is `--file` set correctly?",
            };

            error!("The file {:?} does not exist\n{}", file, help_msg);
            exitcode::NOINPUT
        }

        // Unstable API. Uncomment when it becomes stable.

        // ErrorKind::IsADirectory => {
        //     error!("{:?} is a directory", file);
        //     exitcode::DATAERR
        // }

        // Permission Error!
        ErrorKind::PermissionDenied => {
            error!("Can't access {:?}: Permission denied", file);
            exitcode::NOPERM
        }

        // Not UTF-8
        ErrorKind::InvalidData => {
            error!(
                "The file {:?} is not a UTF-8 text file\n\
                Did you specify a file with a different encoding by accident?",
                file,
            );
            exitcode::DATAERR
        }

        // Other. Just display the name, and exit.
        _ => {
            error!("{:?} Error while trying to access {:?}", kind, file);
            exitcode::UNAVAILABLE
        }
    }
}

fn get_specific_shell<'a>(
    shell: &Shell,
    file_data: &'a ConfigFile,
) -> Option<&'a EnvironmentVariables> {
    //! Gets the specific environment variables IndexMap for a specific shell.
    //!
    //! ie. This will return the map for `Shell::Zsh`, which looks like this in TOML:
    //! ```toml
    //! [shell.zsh]
    //! ...
    //! ```
    //! This function's output is meant to be passed into `to_shell_source(...)`.

    let field_name = shell.to_possible_value()?.get_name();
    file_data.shell.as_ref()?.get(field_name)
}

fn to_shell_source(vars: &EnvironmentVariables, shell: &Shell) -> String {
    //! Converts the hash table of `vars` into a script for the given `shell`.

    let mut output = String::new();
    for (name, raw_value) in vars {
        // Convert an array to a string, but log if it was an array.
        // Any arrays are treated as a path.
        let (value, is_path) = match raw_value.clone() {
            EnvValue::String(s) => (s, false),
            EnvValue::Array(v) => (v.join(":"), true),
        };

        // Replace TOML escape codes with the literal representation so that they are correctly used.
        // We need to do this for every code listed here: https://toml.io/en/v1.0.0#string
        let value = value
            .replace('\\', r"\\") // Backslash - Must be first!
            .replace('\x08', r"\b") // Backspace
            .replace('\t', r"\t") // Tab
            .replace('\n', r"\n") // Newline
            .replace('\x0C', r"\f") // Form Feed
            .replace('\r', r"\r") // Carriage Return
            .replace('\"', "\\\""); // Double Quote

        // Log each processed variable
        if log_enabled!(log::Level::Trace) {
            let variable_log_header = match is_path {
                true => "PATH EnvVar",
                false => "EnvVariable",
            };
            trace!("{}: {} -> {}", variable_log_header, name, value);
        };

        // Select the correct form for the chosen shell.
        match shell {
            Shell::Bash | Shell::Zsh => {
                output += &*format!("export {}=\"{}\";\n", name, value);
            }
            Shell::Fish => {
                // Add `--path` to the variable if the variable was represented as a list in the TOML.
                let path = match is_path {
                    true => " --path",
                    false => "",
                };
                output += &*format!("set -gx{path} {} \"{}\";\n", name, value, path = path);
            }
        };
    }
    output
}

fn get_file_path_default() -> PathBuf {
    //! Gets the default file path for `xshe.toml` if the `-f`/`--file` option is not set.

    // Get the directory where the file is located.
    // If `$XDG_CONFIG_HOME` is set, use that,
    // otherwise use the default location determined by the XDG Base Directory Specification.
    // Spec: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    let xdg_config_home: PathBuf = shellexpand::env("$XDG_CONFIG_HOME")
        .unwrap_or_else(|_| shellexpand::tilde("~/.config"))
        .into_owned()
        .into();

    info!(
        "Using default xshe.toml location: {}",
        xdg_config_home.to_string_lossy()
    );

    xdg_config_home.join("xshe.toml")
}

fn read_stdin() -> String {
    //! Read all text from stdin.
    let mut buffer = String::new();
    std::io::stdin().lock().read_to_string(&mut buffer).unwrap();
    debug!("The following input was read from stdin:\n{}", &buffer);
    buffer
}
