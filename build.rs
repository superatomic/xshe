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

use crate::cli::Cli;
use clap::CommandFactory;
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[path = "src/cli.rs"]
mod cli;

// Adapted from https://rust-cli.github.io/book/in-depth/docs.html

fn main() -> io::Result<()> {
    println!("cargo:rerun-if-changed=src/cli.rs");

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").ok_or(io::ErrorKind::NotFound)?);

    generate_man(&out_dir)?;
    generate_completion(&out_dir)?;

    Ok(())
}

fn generate_completion(out_dir: &Path) -> io::Result<()> {
    // We generate Elvish shell completion for anyone who wants to manually install it,
    // but Homebrew is unable to install Elvish shell completion.
    let shells = &[Shell::Bash, Shell::Elvish, Shell::Fish, Shell::Zsh];

    for shell in shells {
        let mut cmd = Cli::command();
        generate_to(*shell, &mut cmd, "xshe", out_dir)?;
    }
    Ok(())
}

fn generate_man(out_dir: &Path) -> io::Result<()> {
    let cmd = Cli::command();

    // Generate Man File
    let man = Man::new(cmd);
    let mut buffer: Vec<u8> = Default::default();
    man.render(&mut buffer)?;

    fs::write(out_dir.join("xshe.1"), buffer)?;
    Ok(())
}
