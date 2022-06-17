## Installation

You can install `xshe` with [Cargo] (Rust's package manager)
or with [Homebrew] (a package manager for macOS and Linux),
provided you have one of them installed on your system.

If you don't have Cargo or Homebrew, or if you don't want to use either of them,
you can also [download the binaries for your system][gh release latest] directly from GitHub,
or [build `xshe` directly](#build-from-source).

?> **Note:** After installing `xshe` with Cargo or from a download,
   you might have to add the resulting `xshe` binary to your `PATH`.
   [(what's that?)][path?]

### With Cargo

If you have [Cargo installed][Install Cargo/Rust],
use this command to install `xshe` from [crates.io][crates] with Cargo:

```shell
cargo install -f xshe
```

### With Homebrew

If you have [Homebrew] installed,
we recommend that you install `xshe` with Homebrew instead of Cargo.

Simply run:

```shell
brew install superatomic/xshe/xshe
```

Installing `xshe` with Homebrew adds autocompletion to any shells that have completion enabled.

### With Eget

If you have [Eget] installed, just run:

```shell
eget superatomic/xshe
```

This will install a prebuilt binary of `xshe`.

### As a file download

?> This method will **not** add `xshe` to your `PATH`.
   Make sure to add the `xshe` binary to your `PATH` manually, 
   or remember to use the full path to the binary whenever you run `xshe`.

Instead of using Cargo,
you can download the [**latest release binary**][gh release latest] that corresponds with your system
(or view [**all releases**][gh release]).

Make sure to add the binary to your `PATH`.

### Build from source

First, download the repo:

!> This installs the latest development version of `xshe`,
   which may be ahead of the current stable release.
   If you want the latest stable version instead, 
   build from the latest git tag or
   [download the source for the latest version][gh release latest]
   from the releases page.

To build `xshe` from source:

```shell
git clone https://github.com/superatomic/xshe
cd xshe
cargo build --release
sudo mv target/release/xshe /usr/local/bin  # <-- optional
```

Make sure to place the generated binary at `target/release/xshe` on your `PATH`.
In this example, `xshe` is installed to `/usr/local/bin`, but `xshe` can anywhere on your `PATH`.

# Next steps
1. [Create an `xshe.toml` file to define your environment variables for every shell →](config_file.md)
2. [Add `xshe` to the startup script of every shell →](cli.md)

[crates]: https://crates.io/crates/xshe
[Cargo]: https://doc.rust-lang.org/cargo/

[Homebrew]: https://brew.sh
[Install Cargo/Rust]: https://www.rust-lang.org/tools/install
[Eget]: https://github.com/zyedidia/eget

[path?]: https://askubuntu.com/questions/551990/what-does-path-mean

[gh release]: https://github.com/superatomic/xshe/releases/
[gh release latest]: https://github.com/superatomic/xshe/releases/latest
