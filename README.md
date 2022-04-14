<div align=center>

  # Xshe – Cross-Shell Environment Vars

  [![Fork me on GitHub][icon-fork]][fork]
  [![Leave a GitHub Repo Star][icon-star]][repo]
  [![Open an Issue][icon-issue]][new issue]
  [![View on Lib.rs][icon-lib.rs]][lib.rs]

</div>

`xshe` allows for setting <u>Sh</u>ell <u>E</u>nvironment Variables across multiple shells with a single TOML
configuration file.

Simply write lines in a `xshe.toml` file like this:

```toml
CARGO_HOME = "~/.cargo"
```

Create a file like this once and use it everywhere, for every shell! `xshe` can convert this format into the format for
every supported shell.

<!--When updating this list, update the icon *AND* the alt text -->
[![Shells - bash | zsh | fish][icon-shells]](#sourcing-the-xshetoml-file)
[![Coming Soon - elvish | dash | xonsh | tsch][icon-future-shells]][future shells]

---

<div align=center>

  [![GitHub Release Status][icon-release]][release workflows]
  [![Libraries.io dependency status][icon-depend]][libraries.io tree]
  [![License][icon-license]][license]
  [![Latest Crates.io Release][icon-crates]][crates]
  [![Latest GitHub Release][icon-gh-release]][gh release]
  [![Crates.io downloads][icon-crates-downloads]][lib.rs install]

</div>

---

## Installation

You can install `xshe` from [Cargo] (Rust's package manager) if you have it installed on your system.
If you don't have Cargo or don't want to use it,
you can also [download the binaries for your system][gh release latest] directly from GitHub,
or [install Cargo/Rust] first before proceeding.

**Note:** After installing `xshe`, you might have to add the resulting `xshe` binary to your `PATH`.
[<sup>(what's that?)</sup>][path?]

### With Cargo

Install [`xshe`][crates] from [crates.io][crates] with [Cargo].

```shell
cargo install -f xshe
```

### As a File Download

Instead of using Cargo, you can download the [**latest release binary**][gh release latest] that corresponds with your system
(or view [**all releases**][gh release]).

Make sure to add the `xshe` binary to your `PATH`,
or remember to use the full path to the binary whenever you run `xshe`.

---

## Setup

### Creating a `xshe.toml` file

Create a file called `xshe.toml` in `~/.config`. This is a [TOML file][toml] that represents environment variables.

[![An example configuration is here: xshe example][icon-example]][example]

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

#### Shell Specific Environment Variables

To set environment variables for only one shell, add a new table called `[shell.NAME]` after all standard definitions,
where `NAME` is on of `bash`, `zsh`, or `fish`.
Then list the environment variables that will only be added if `xshe` is being used for the given shell.

For example, to make `$HISTFILE` be different between shells and `$ZSH_CACHE_DIR` only be set in **zsh**, do this:
```toml
[shell.bash]
HISTFILE = "$XDG_STATE_HOME/bash_history"

[shell.zsh]
HISTFILE = "$XDG_STATE_HOME/zsh_history"
ZSH_CACHE_DIR = "$XDG_CACHE_HOME/oh-my-zsh"
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
xshe fish | source
```

#### Use without `xshe` on `PATH`
If `xshe` isn't on your `PATH` (this is the cause of the error `command not found: xshe`), you will have to manually type out the location:

```zsh
eval "$(/path/to/xshe zsh)"
```

#### Options

##### Using `--file` (or `-f`)

To specify a custom file that is not located at `~/.config/xshe.toml`, pass the `--file` option, like so:

```zsh
eval "$(xshe zsh --file ~/some/other/location.toml)"
```

##### Using `--toml` (or `-t`)

To directly specify TOML to parse as a config file, use `--toml`.

For example, this line directly parses the provided line and converts it to zsh:
```zsh
xshe zsh --toml 'BIN_HOME = "$HOME/.local/bin"'
```

##### Using `--pipe` (or `-p`)

To pass a TOML configuration from the standard input, use `--pipe`.

As an example, this command concatenates two files named
`global_xshe.toml` and `user_xshe.toml` by using `cat`,
and then pipes the output into `xshe` to be parsed:
```zsh
cat global_xshe.toml user_xshe.toml | xshe zsh --pipe
```

#### Other CLI Options

##### Output Verbosity

You can control how much info is displayed when Xshe is run.
The default behavior is to only display errors and warnings.

While this default behavior is recommended, you can customize it by using the following flags:

* `-qq` silences all output (this silences errors and is not advised)
* `-q` or `--quiet` shows only errors and hides warnings
* `-v` or `--verbose` shows info messages
* `-vv` shows debug logs
* `-vvv` shows trace logs

##### Help and Version Information

Run `xshe --help` to display command usage.

Run `xshe --version` to display version information.

---

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE.txt) or [www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0))
* MIT license ([LICENSE-MIT](LICENSE-MIT.txt) or [opensource.org/licenses/MIT](https://opensource.org/licenses/MIT))

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

---

![built with love][icon-love]


[icon-fork]:  https://custom-icon-badges.herokuapp.com/badge/-Fork%20me%20on%20Github-teal?style=flat&logo=repo-forked&logoColor=white
[icon-star]:  https://custom-icon-badges.herokuapp.com/badge/-Star%20Repo-action?style=flat&logo=star&logoColor=white&color=F25278
[icon-issue]: https://custom-icon-badges.herokuapp.com/badge/-Open%20an%20Issue-palegreen?style=flat&logo=issue-opened&logoColor=black
[icon-lib.rs]: https://custom-icon-badges.herokuapp.com/badge/-Lib.rs-bb44ee?style=flat&logo=book&logoColor=white

[icon-release]: https://custom-icon-badges.herokuapp.com/github/workflow/status/superatomic/xshe/release?label=release%20build&style=for-the-badge&logo=file-zip&logoColor=white
[icon-depend]: https://custom-icon-badges.herokuapp.com/librariesio/release/cargo/xshe?style=for-the-badge&logo=package-dependencies&logoColor=white
[icon-license]: https://custom-icon-badges.herokuapp.com/crates/l/xshe?style=for-the-badge&logo=law&logoColor=white
[icon-crates]: https://custom-icon-badges.herokuapp.com/crates/v/xshe?logo=package&style=for-the-badge&logoColor=white
[icon-gh-release]: https://custom-icon-badges.herokuapp.com/github/v/release/superatomic/xshe?include_prereleases&logo=github&style=for-the-badge
[icon-crates-downloads]: https://custom-icon-badges.herokuapp.com/crates/d/xshe?style=for-the-badge&logo=download&logoColor=white

[icon-shells]: https://custom-icon-badges.herokuapp.com/badge/Shells-bash_|_zsh_|_fish-2ea44f?logo=terminal&logoColor=white
[icon-future-shells]: https://custom-icon-badges.herokuapp.com/badge/Coming_Soon-elvish_|_dash_|_xonsh_|_tsch-yellow?logo=checklist&logoColor=white

[icon-example]: https://custom-icon-badges.herokuapp.com/badge/Example-xshe.toml-blue?labelColor=blue&color=lightblue&logo=file&logoColor=white

[icon-love]: https://forthebadge.com/images/badges/built-with-love.svg


[fork]: https://github.com/superatomic/xshe/fork
[new issue]: https://github.com/superatomic/xshe/issues/new/choose
[repo]: https://github.com/superatomic/xshe/
[lib.rs]: https://lib.rs/crates/xshe
[lib.rs install]: https://lib.rs/install/xshe
[libraries.io]: https://libraries.io/cargo/xshe
[crates]: https://crates.io/crates/xshe

[future shells]: https://github.com/users/superatomic/projects/1

[license]: https://github.com/search?q=repo%3Asuperatomic%2Fxshe+path%3A%2F+filename%3ALICENSE&type=Code
[libraries.io tree]: https://libraries.io/cargo/xshe/tree?kind=normal

[gh release]: https://github.com/superatomic/xshe/releases/
[gh release latest]: https://github.com/superatomic/xshe/releases/latest
[release workflows]: https://github.com/superatomic/xshe/actions/workflows/release.yml

[Cargo]: https://doc.rust-lang.org/cargo/
[install Cargo/Rust]: https://www.rust-lang.org/tools/install
[toml]: https://toml.io/en/

[example]: https://gist.github.com/superatomic/8f22ada9864c85984d51e0cc6fae4250
[path?]: https://askubuntu.com/questions/551990/what-does-path-mean
