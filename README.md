<div align="center">

  # Xshe – Cross-Shell Environment Vars

  [![Documentation][icon-docs]][docs]
  [![View on Crates.io][icon-link-crates]][crates]
  [![Fork me on GitHub][icon-fork]][fork]
  [![Leave a GitHub Repo Star][icon-star]][repo]
  [![Open an Issue][icon-issue]][new issue]

  [![GitHub Release Status][icon-release]][release workflows]
  [![Libraries.io dependency status][icon-depend]][libraries.io tree]
  [![Latest Crates.io Release][icon-crates]][crates]
  [![Latest GitHub Release][icon-gh-release]][gh release]
  [![Crates.io downloads][icon-crates-downloads]][crates]

</div>


**Xshe** allows for setting <u>Sh</u>ell <u>E</u>nvironment Variables across multiple shells with a single TOML
configuration file.

Instead of writing multiple similar files for each shell you use,
you can instead create one file and use it for every shell with **xshe**!

All you have to do is [add a single line](docs/cli.md#sourcing-the-xshetoml-file) to all of your shells' startup scripts,
and `xshe` will set your environment variable across all of them.

To use **xshe**, you write lines in a `xshe.toml` file like this _(in [TOML] format)_:

```toml
CARGO_HOME = "~/.cargo"
EDITOR = "$(which nano)"
```

Create a file like this once and use it everywhere, for every shell!
`xshe` can convert this into the format for every supported shell.

<!--When updating this list, update the icon *AND* the alt text -->
[![Shells - bash | zsh | fish][icon-shells]](#)
[![Coming Soon - elvish | dash | xonsh | tcsh][icon-future-shells]][future shells]

## Usage and Documentation

View the documentation for `xshe` online at [xshe.superatomic.dev][docs]
or [locally by opening the docs](docs/README.md).

[docs]: https://xshe.superatomic.dev

## Quick install
* [With Cargo](docs/install#with-cargo)
* [With Homebrew](docs/install#with-homebrew)
* [As a File Download](docs/install#as-a-file-download)
* [Build from Source](docs/install#build-from-source)

<div align="center">

  ---

  The branch `main` is ahead of the current release.
  If you are looking for the documentation for the latest released version,
  [switch to the `0.4.2` release branch](https://github.com/superatomic/xshe/tree/v0.4.2),
  or view the documentation on [Lib.rs][lib.rs] or [Crates.io][crates].
  
  ![GitHub commits since latest release (by date)](https://img.shields.io/github/commits-since/superatomic/xshe/latest/main)

  ---

</div>

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

<div align=center>
  
  [![GitHub.com][icon-link-github]][repo]
  [![Crates.io][icon-link-crates]][crates]
  [![Lib.rs][icon-link-lib.rs]][lib.rs]
  [![Libraries.io][icon-link-libraries]][libraries.io]

</div>

[icon-link-github]: https://custom-icon-badges.herokuapp.com/badge/-GitHub.com-2ea44f?logo=github&logoColor=white&style=flat
[icon-link-crates]: https://custom-icon-badges.herokuapp.com/badge/-Crates.io-ffc832?logo=package&logoColor=black&style=flat
[icon-link-lib.rs]: https://custom-icon-badges.herokuapp.com/badge/-Lib.rs-bb44ee?logo=book&logoColor=white&style=flat
[icon-link-libraries]: https://custom-icon-badges.herokuapp.com/badge/-Libraries.io-337ab7?logo=codescan&logoColor=white&style=flat

[icon-fork]:  https://custom-icon-badges.herokuapp.com/badge/-Fork%20me%20on%20Github-teal?style=flat&logo=repo-forked&logoColor=white
[icon-docs]:  https://custom-icon-badges.herokuapp.com/badge/-Documentation-9cf?style=flat&logo=book&logoColor=black
[icon-star]:  https://custom-icon-badges.herokuapp.com/badge/-Star%20Repo-action?style=flat&logo=star&logoColor=white&color=F25278
[icon-issue]: https://custom-icon-badges.herokuapp.com/badge/-Open%20an%20Issue-palegreen?style=flat&logo=issue-opened&logoColor=black

[icon-release]: https://custom-icon-badges.herokuapp.com/github/workflow/status/superatomic/xshe/release?label=release%20build&style=flat&logo=file-zip&logoColor=white
[icon-depend]: https://custom-icon-badges.herokuapp.com/librariesio/release/cargo/xshe?style=flat&logo=package-dependencies&logoColor=white
[icon-crates]: https://custom-icon-badges.herokuapp.com/crates/v/xshe?logo=package&style=flat&logoColor=white
[icon-gh-release]: https://custom-icon-badges.herokuapp.com/github/v/release/superatomic/xshe?include_prereleases&logo=github&style=flat
[icon-crates-downloads]: https://custom-icon-badges.herokuapp.com/crates/d/xshe?style=flat&logo=download&logoColor=white

[icon-shells]: https://custom-icon-badges.herokuapp.com/badge/Shells-bash_|_zsh_|_fish-2ea44f?logo=terminal&logoColor=white&style=flat-square
[icon-future-shells]: https://custom-icon-badges.herokuapp.com/badge/Coming_Soon-elvish_|_dash_|_xonsh_|_tcsh-yellow?logo=checklist&logoColor=white&style=flat-square

[fork]: https://github.com/superatomic/xshe/fork
[new issue]: https://github.com/superatomic/xshe/issues/new/choose
[repo]: https://github.com/superatomic/xshe/
[lib.rs]: https://lib.rs/crates/xshe
[libraries.io]: https://libraries.io/cargo/xshe
[crates]: https://crates.io/crates/xshe

[future shells]: https://github.com/users/superatomic/projects/1

[libraries.io tree]: https://libraries.io/cargo/xshe/tree?kind=normal

[gh release]: https://github.com/superatomic/xshe/releases/
[release workflows]: https://github.com/superatomic/xshe/actions/workflows/release.yml

[toml]: https://toml.io/en/
