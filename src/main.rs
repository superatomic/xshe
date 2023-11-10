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
//! * Apache License, Version 2.0 (`LICENSE-APACHE` or <http://www.apache.org/licenses/LICENSE-2.0>)
//! * MIT license (`LICENSE-MIT` or <http://opensource.org/licenses/MIT>)
//!
//! at your option.

#![forbid(unsafe_code)]

mod cli;
mod convert;
mod structure;

#[macro_use]
extern crate log;

use clap::{Parser, ValueEnum};
use human_panic::setup_panic;
use indexmap::IndexMap;
use std::io::{stdin, ErrorKind, Read};
use std::{env, fs, path::PathBuf, process::exit, string::String};

use crate::cli::{Cli, Shell};
use crate::structure::{ConfigFile, EnvVariableOption, EnvVariableValue};

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
    } else if let Some(text) = cli_options.text {
        // If --text was specified, use that. Otherwise, get the file and read from it.
        (text, String::from("<STRING>"))
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
                "The file {} is not in a valid TOML format,\n\
                 or it is not in the form xshe is expecting.",
                file_name
            );
            error!("{}", e);
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
    let output = convert::to_shell_source(&file_data.vars, &shell);
    print!("{}", output);

    // Retain compatibility with deprecated https://github.com/superatomic/xshe/issues/30
    deprecated_to_specific_shell_source(&file_data, &shell);
}

fn deprecated_to_specific_shell_source(file_data: &ConfigFile, shell: &Shell) {
    // Output the any specific variables for the shell the same way, if they exist.
    // This behavior is deprecated.
    if let Some(specific_vars) = get_specific_shell(shell, file_data) {
        // wrap `specific_vars` to be compatible with the changed `to_shell_source` function.
        // This is a hack, but it's only to preserve deprecated behavior until it is removed.
        let wrap_specific_vars = specific_vars
            .into_iter()
            .map(|(key, value)| (key.to_owned(), EnvVariableOption::General(value.to_owned())))
            .collect();

        let shell_specific_output = convert::to_shell_source(&wrap_specific_vars, shell);

        print!("{:?}", shell_specific_output);
    };
}

fn read_config_file(cli_options: &Cli) -> (String, String) {
    //! Read from the config file that should be selected based on the `--file` option.
    // Get the target TOML file with the environment variables.
    // If they are not manually set, use the XDG Base Directory Specification defaults.
    let raw_file = &cli_options.file;
    let file = &raw_file.clone().unwrap_or_else(get_file_path_default);

    // Read the contents of the file.
    // Exit with an error message and exit code if an error occurs.
    let toml_string = match fs::read_to_string(file) {
        Ok(string) => string,
        Err(e) => exit_with_file_error(e.kind(), &file.to_string_lossy(), raw_file.is_some()),
    };
    (toml_string, file.display().to_string())
}

fn exit_with_file_error(kind: ErrorKind, file_name: &str, file_option_set: bool) -> ! {
    //! Displays an error message and exits with a specific exit code.
    let exit_code: i32 = match kind {
        // The file doesn't exist!
        ErrorKind::NotFound => {
            // Select an informative help message.
            let help_msg = match file_option_set {
                false => {
                    "Make sure that you have a xshe.toml file in the default location\n\
                    or try setting --file to point to a custom location"
                }
                true => "Is --file set correctly?",
            };

            error!("The file {:?} does not exist\n{}", file_name, help_msg);
            exitcode::NOINPUT
        }

        // Unstable API. Uncomment when it becomes stable.

        // ErrorKind::IsADirectory => {
        //     error!("{:?} is a directory", file);
        //     exitcode::DATAERR
        // }

        // Permission Error!
        ErrorKind::PermissionDenied => {
            error!("Can't access {:?}: Permission denied", file_name);
            exitcode::NOPERM
        }

        // Not UTF-8
        ErrorKind::InvalidData => {
            error!(
                "The file {:?} is not a UTF-8 text file\n\
                Did you specify a file with a different encoding by accident?",
                file_name,
            );
            exitcode::DATAERR
        }

        // Other. Just display the name, and exit.
        _ => {
            error!("Error while trying to access {:?}: {:?}", file_name, kind);
            exitcode::UNAVAILABLE
        }
    };
    exit(exit_code);
}

// Deprecated
fn get_specific_shell<'a>(
    shell: &Shell,
    file_data: &'a ConfigFile,
) -> Option<&'a IndexMap<String, EnvVariableValue>> {
    //! Gets the specific environment variables IndexMap for a specific shell.
    //!
    //! ie. This will return the map for `Shell::Zsh`, which looks like this in TOML:
    //! ```toml
    //! [shell.zsh]
    //! ...
    //! ```
    //! This function's output is meant to be passed into `to_shell_source(...)`.
    let binding = shell.to_possible_value()?;
    let field_name = binding.get_name();
    file_data.shell.as_ref()?.get(field_name)
}

fn get_file_path_default() -> PathBuf {
    //! Gets the default file path for `xshe.toml` if the `-f`/`--file` option is not set.

    // Get the directory where the file is located.
    // If `$XDG_CONFIG_HOME` is set, use that,
    // otherwise use the default location determined by the XDG Base Directory Specification.
    // Spec: https://specifications.freedesktop.org/basedir-spec/basedir-spec-latest.html
    let xdg_config_home: PathBuf = env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| {
                    error!("Could not determine the location of the home directory");
                    exit(exitcode::NOUSER);
                })
                .join(".config")
        });

    info!(
        "Using default xshe.toml location: {}",
        xdg_config_home.to_string_lossy(),
    );

    xdg_config_home.join("xshe.toml")
}

fn read_stdin() -> String {
    //! Read all text from stdin.
    let mut buffer = String::new();
    stdin()
        .lock()
        .read_to_string(&mut buffer)
        .unwrap_or_else(|e| {
            // If something went wrong,
            // display a nice error message instead of panicking.
            error!("The following error occurred while reading from standard input:");
            exit_with_file_error(e.kind(), "<STDIN>", false);
        });
    debug!("The following input was read from stdin:\n{}", &buffer);
    buffer
}
