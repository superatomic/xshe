# Xshe – Cross-Shell Environment Vars

Set <u>Sh</u>ell <u>E</u>nvironment variables across multiple shells with a single configuration file.

[![Fork me on GitHub!](https://img.shields.io/badge/-Fork%20me%20on%20Github!-blueviolet?style=flat-square&logo=github)](https://github.com/superatomic/xshe/fork)
[![Leave a GitHub Repo Star](https://img.shields.io/badge/-Star%20Repo-blue?style=flat-square&logo=github)](https://github.com/superatomic/xshe/)

---

[![Crates.io](https://img.shields.io/crates/v/xshe?logo=rust&style=for-the-badge)](https://crates.io/crates/xshe)
![Crates.io](https://img.shields.io/crates/l/xshe?style=for-the-badge)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/superatomic/xshe/release?label=release%20build&style=for-the-badge)](https://github.com/superatomic/xshe/actions/workflows/release.yml)
[![GitHub release (latest by date including pre-releases)](https://img.shields.io/github/v/release/superatomic/xshe?include_prereleases&logo=github&style=for-the-badge)](https://github.com/superatomic/xshe/releases/latest)
![GitHub top language](https://img.shields.io/github/languages/top/superatomic/xshe?label=made%20with%20rust&color=blueviolet&logo=rust&style=for-the-badge)

---

## Installation

You can install `xshe` from [Cargo](https://doc.rust-lang.org/cargo/) (Rust's package manager) if you have it installed on your system.
If you don't have Cargo or don't want to use it, you can download the binaries for your system directly from GitHub.

**Note:** After installing `xshe`, you might have to add the resulting `xshe` binary to your `PATH`.
[<sup>*(what's that?)*</sup>](https://askubuntu.com/questions/551990/what-does-path-mean)

### With Cargo

Install [`xshe`](https://crates.io/crates/xshe) from [crates.io](https://crates.io/crates/xshe) with [Cargo](https://doc.rust-lang.org/cargo/).

```shell
cargo install xshe
```

### As a File Download

Instead of using Cargo, you can download the [**latest release binary**](https://github.com/superatomic/xshe/releases/latest) that corresponds with your system
(or view [**all releases**](https://github.com/superatomic/xshe/releases)).

---

## Setup

### Creating a `xshe.toml` file

Create a file called `xshe.toml` in `~/.config`. This is a [TOML file](https://toml.io/en/) that represents environment variables.

[![An example configuration is here: xshe example](https://img.shields.io/badge/Example-xshe.toml-green)](https://gist.github.com/superatomic/8f22ada9864c85984d51e0cc6fae4250)

One variable is set per line. The file is read in order from top to bottom,
so variables that appear earlier in the file can be used to define ones that appear later.

A typical line looks like this:

```toml
CARGO_HOME = "$XDG_DATA_HOME/cargo"
```

This will then be converted into the correct format for whatever shell is being used.
For example, in **bash**, this line becomes:

```bash
export CARGO_HOME="$XDG_DATA_HOME/cargo";
```
While in **fish**, this line is:
```fish
set -gx CARGO_HOME "$XDG_DATA_HOME/cargo";
```

#### Dealing with `PATH` variables

To set variables that are arrays of values, like `$PATH`, use this syntax:

```toml
PATH = ["$PATH", "$BIN_HOME", "$CARGO_HOME/bin"]
```
`xshe` will join each element together based on the shell that is specified.

### Sourcing the `xshe.toml` file

Put the line corresponding to your shell in whatever file runs when loading environment variables.
For **bash**, this is `~/.bash_profile`, for **zsh**, this is `~/.zshenv`, and for **fish**, this is `~/.config/fish/config.fish`.

##### Bash
```bash
eval "$(xshe bash)"
```

##### Zsh
```zsh
eval "$(xshe zsh)"
```

##### Fish
```fish
eval "$(xshe fish)"
```

#### Options

To specify a custom file that is not located at `~/.config/xshe.toml`, pass the `--file` option, like so:

```zsh
eval "$(xshe zsh --file ~/some/other/location.toml)"
```

If `xshe` isn't on your `PATH` (this is the cause of the error `command not found: xshe`), you will have to manually type out the location:

```zsh
eval "$(/path/to/xshe zsh)"
```

---

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE.txt) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT.txt) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

---

[![built with love](https://forthebadge.com/images/badges/built-with-love.svg)](https://forthebadge.com)
