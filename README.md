# Xshe – Cross-Shell Env Vars
Set <u>Sh</u>ell <u>E</u>nvironment variables across multiple shells with a single configuration file.

## Installation

Install [`xshe`](https://crates.io/crates/xshe) from [crates.io](https://crates.io) with [cargo](https://doc.rust-lang.org/cargo/).

```shell
cargo install xshe
```

You might need to add the resulting `xshe` binary to your `PATH`.

[Release binaries](https://github.com/superatomic/xshe/releases) are also available at GitHub.

## Setup

### Creating a `xshe.toml` file

Create a file called `xshe.toml` in `~/.config`. This is a [TOML file](https://toml.io/en/) that represents environment variables.

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


## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE.txt) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT.txt) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
