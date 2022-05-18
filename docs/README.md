<div align="center">

  # Xshe &ndash; Cross-Shell Environment Vars

  [![GitHub Release Status][icon-release]][release workflows]
  [![Libraries.io dependency status][icon-depend]][libraries.io tree]
  [![Latest GitHub Release][icon-gh-release]][gh release]
  [![Crates.io downloads][icon-crates-downloads]][crates]

</div>

**Xshe** allows for setting <u>Sh</u>ell <u>E</u>nvironment Variables across multiple shells with a single TOML
configuration file.

Instead of writing multiple similar files for each shell you use,
you can instead create one file and use it for every shell with **xshe**!

All you have to do is [add a single line](cli.md#sourcing-the-xshetoml-file) to all of your shells' startup scripts,
and `xshe` will set your environment variable across all of them.

To use **xshe**, you write lines in a `xshe.toml` file like this _(in [TOML] format)_:

```toml
CARGO_HOME = "~/.cargo"
EDITOR = "$(which nano)"
```

Create a file like this once and use it everywhere, for every shell!
`xshe` can convert this into the format for every supported shell.

<!--When updating this list, update the icon *AND* the alt text -->

![Shells - bash | zsh | fish][icon-shells]
[![Coming Soon - elvish | dash | xonsh | tcsh][icon-future-shells]][future shells]

**Xshe** support more than just plain conversions.<br />
It can [set lists of paths like `$PATH` and `$MANPATH`](config_file.md#dealing-with-path-variables),
[set certain variables to different values depending on the shell](config_file.md#shell-specific-environment-variables),
[unset environment variables](config_file.md#setting-and-unsetting-variables),
and much more!


## Quick install
* [With Cargo](install#with-cargo)
* [With Homebrew](install#with-homebrew)
* [As a File Download](install#as-a-file-download)
* [Build from Source](install#build-from-source)

---

<div align="center">

  `xshe` is licensed under **MIT OR Apache-2.0**

</div>

[icon-release]: https://custom-icon-badges.herokuapp.com/github/workflow/status/superatomic/xshe/release?label=release%20build&style=flat&logo=file-zip&logoColor=white
[icon-depend]: https://custom-icon-badges.herokuapp.com/librariesio/release/cargo/xshe?style=flat&logo=package-dependencies&logoColor=white
[icon-gh-release]: https://custom-icon-badges.herokuapp.com/github/v/release/superatomic/xshe?include_prereleases&logo=github&style=flat
[icon-crates-downloads]: https://custom-icon-badges.herokuapp.com/crates/d/xshe?style=flat&logo=download&logoColor=white

[icon-shells]: https://custom-icon-badges.herokuapp.com/badge/Shells-bash_|_zsh_|_fish-2ea44f?logo=terminal&logoColor=white&style=flat-square
[icon-future-shells]: https://custom-icon-badges.herokuapp.com/badge/Coming_Soon-elvish_|_dash_|_xonsh_|_tcsh-yellow?logo=checklist&logoColor=white&style=flat-square

[future shells]: https://github.com/users/superatomic/projects/1
[repo]: https://github.com/superatomic/xshe/
[crates]: https://crates.io/crates/xshe

[libraries.io tree]: https://libraries.io/cargo/xshe/tree?kind=normal

[gh release]: https://github.com/superatomic/xshe/releases/
[release workflows]: https://github.com/superatomic/xshe/actions/workflows/release.yml

[TOML]: https://toml.io/en/
