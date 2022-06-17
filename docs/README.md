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
you can instead create one file and use it for every shell with `xshe`!

All you have to do is [add a single line](cli.md#sourcing-the-xshetoml-file) to all of your shells' startup scripts,
and **xshe** will set your environment variable across all of them.

To use **xshe**, you write lines in a `xshe.toml` file like this _(in [TOML] format)_:

```toml
CARGO_HOME = "~/.cargo"
EDITOR = "$(which nano)"
```

Create a file like this once and use it everywhere, for every shell!
**Xshe** can convert this into the format for every supported shell.

<!--When updating this list, update the icon *AND* the alt text -->

![Shells - bash | zsh | fish][icon-shells]
[![Coming Soon - elvish | dash | xonsh | tcsh][icon-future-shells]][future shells]

**Xshe** support more than just plain conversions.<br />
It can [set lists of paths like `$PATH` and `$MANPATH`](config_file.md#dealing-with-path-variables),
[set certain variables to different values depending on the shell](config_file.md#shell-specific-environment-variables),
[unset environment variables](config_file.md#setting-and-unsetting-variables),
and much more!


## Quick install
* [With Cargo →](install.md#with-cargo)
* [With Homebrew →](install.md#with-homebrew)
* [With Eget →](install.md#with-eget)
* [As a File Download →](install.md#as-a-file-download)
* [Build from Source →](install.md#build-from-source)

---

<div align="center">
  <small>

  `xshe` is licensed under **MIT OR Apache-2.0**
  
  </small>
</div>

[icon-release]: https://img.shields.io/github/workflow/status/superatomic/xshe/release?label=release%20build&style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0wIDEuNzVDMCAuNzg0Ljc4NCAwIDEuNzUgMGgzLjVDNi4yMTYgMCA3IC43ODQgNyAxLjc1djMuNUExLjc1IDEuNzUgMCAwMTUuMjUgN0g0djRhMSAxIDAgMDAxIDFoNHYtMS4yNUM5IDkuNzg0IDkuNzg0IDkgMTAuNzUgOWgzLjVjLjk2NiAwIDEuNzUuNzg0IDEuNzUgMS43NXYzLjVBMS43NSAxLjc1IDAgMDExNC4yNSAxNmgtMy41QTEuNzUgMS43NSAwIDAxOSAxNC4yNXYtLjc1SDVBMi41IDIuNSAwIDAxMi41IDExVjdoLS43NUExLjc1IDEuNzUgMCAwMTAgNS4yNXYtMy41em0xLjc1LS4yNWEuMjUuMjUgMCAwMC0uMjUuMjV2My41YzAgLjEzOC4xMTIuMjUuMjUuMjVoMy41YS4yNS4yNSAwIDAwLjI1LS4yNXYtMy41YS4yNS4yNSAwIDAwLS4yNS0uMjVoLTMuNXptOSA5YS4yNS4yNSAwIDAwLS4yNS4yNXYzLjVjMCAuMTM4LjExMi4yNS4yNS4yNWgzLjVhLjI1LjI1IDAgMDAuMjUtLjI1di0zLjVhLjI1LjI1IDAgMDAtLjI1LS4yNWgtMy41eiI+PC9wYXRoPjwvc3ZnPg==
[icon-depend]: https://img.shields.io/librariesio/release/cargo/xshe?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik02LjEyMi4zOTJhMS43NSAxLjc1IDAgMDExLjc1NiAwbDUuMjUgMy4wNDVjLjU0LjMxMy44NzIuODkuODcyIDEuNTE0VjcuMjVhLjc1Ljc1IDAgMDEtMS41IDBWNS42NzdMNy43NSA4LjQzMnY2LjM4NGExIDEgMCAwMS0xLjUwMi44NjVMLjg3MiAxMi41NjNBMS43NSAxLjc1IDAgMDEwIDExLjA0OVY0Ljk1MWMwLS42MjQuMzMyLTEuMi44NzItMS41MTRMNi4xMjIuMzkyek03LjEyNSAxLjY5bDQuNjMgMi42ODVMNyA3LjEzMyAyLjI0NSA0LjM3NWw0LjYzLTIuNjg1YS4yNS4yNSAwIDAxLjI1IDB6TTEuNSAxMS4wNDlWNS42NzdsNC43NSAyLjc1NXY1LjUxNmwtNC42MjUtMi42ODNhLjI1LjI1IDAgMDEtLjEyNS0uMjE2em0xMS42NzItLjI4MmEuNzUuNzUgMCAxMC0xLjA4Ny0xLjAzNGwtMi4zNzggMi41YS43NS43NSAwIDAwMCAxLjAzNGwyLjM3OCAyLjVhLjc1Ljc1IDAgMTAxLjA4Ny0xLjAzNEwxMS45OTkgMTMuNWgzLjI1MWEuNzUuNzUgMCAwMDAtMS41aC0zLjI1MWwxLjE3My0xLjIzM3oiPjwvcGF0aD48L3N2Zz4=
[icon-gh-release]: https://img.shields.io/github/v/release/superatomic/xshe?include_prereleases&style=flat&logo=github
[icon-crates-downloads]: https://img.shields.io/crates/d/xshe?style=flat&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik03LjQ3IDEwLjc4YS43NS43NSAwIDAwMS4wNiAwbDMuNzUtMy43NWEuNzUuNzUgMCAwMC0xLjA2LTEuMDZMOC43NSA4LjQ0VjEuNzVhLjc1Ljc1IDAgMDAtMS41IDB2Ni42OUw0Ljc4IDUuOTdhLjc1Ljc1IDAgMDAtMS4wNiAxLjA2bDMuNzUgMy43NXpNMy43NSAxM2EuNzUuNzUgMCAwMDAgMS41aDguNWEuNzUuNzUgMCAwMDAtMS41aC04LjV6Ij48L3BhdGg+PC9zdmc+

[icon-shells]: https://img.shields.io/badge/Shells-bash_|_zsh_|_fish-2ea44f?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0wIDIuNzVDMCAxLjc4NC43ODQgMSAxLjc1IDFoMTIuNWMuOTY2IDAgMS43NS43ODQgMS43NSAxLjc1djEwLjVBMS43NSAxLjc1IDAgMDExNC4yNSAxNUgxLjc1QTEuNzUgMS43NSAwIDAxMCAxMy4yNVYyLjc1em0xLjc1LS4yNWEuMjUuMjUgMCAwMC0uMjUuMjV2MTAuNWMwIC4xMzguMTEyLjI1LjI1LjI1aDEyLjVhLjI1LjI1IDAgMDAuMjUtLjI1VjIuNzVhLjI1LjI1IDAgMDAtLjI1LS4yNUgxLjc1ek03LjI1IDhhLjc1Ljc1IDAgMDEtLjIyLjUzbC0yLjI1IDIuMjVhLjc1Ljc1IDAgMTEtMS4wNi0xLjA2TDUuNDQgOCAzLjcyIDYuMjhhLjc1Ljc1IDAgMTExLjA2LTEuMDZsMi4yNSAyLjI1Yy4xNDEuMTQuMjIuMzMxLjIyLjUzem0xLjUgMS41YS43NS43NSAwIDAwMCAxLjVoM2EuNzUuNzUgMCAwMDAtMS41aC0zeiI+PC9wYXRoPjwvc3ZnPg==
[icon-future-shells]: https://img.shields.io/badge/Coming_Soon-elvish_|_dash_|_xonsh_|_tcsh-yellow?style=flat-square&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAxNiAxNiIgd2lkdGg9IjE2IiBoZWlnaHQ9IjE2Ij48cGF0aCBzdHlsZT0iZmlsbDojZmZmIiBmaWxsLXJ1bGU9ImV2ZW5vZGQiIGQ9Ik0yLjUgMS43NWEuMjUuMjUgMCAwMS4yNS0uMjVoOC41YS4yNS4yNSAwIDAxLjI1LjI1djcuNzM2YS43NS43NSAwIDEwMS41IDBWMS43NUExLjc1IDEuNzUgMCAwMDExLjI1IDBoLTguNUExLjc1IDEuNzUgMCAwMDEgMS43NXYxMS41YzAgLjk2Ni43ODQgMS43NSAxLjc1IDEuNzVoMy4xN2EuNzUuNzUgMCAwMDAtMS41SDIuNzVhLjI1LjI1IDAgMDEtLjI1LS4yNVYxLjc1ek00Ljc1IDRhLjc1Ljc1IDAgMDAwIDEuNWg0LjVhLjc1Ljc1IDAgMDAwLTEuNWgtNC41ek00IDcuNzVBLjc1Ljc1IDAgMDE0Ljc1IDdoMmEuNzUuNzUgMCAwMTAgMS41aC0yQS43NS43NSAwIDAxNCA3Ljc1em0xMS43NzQgMy41MzdhLjc1Ljc1IDAgMDAtMS4wNDgtMS4wNzRMMTAuNyAxNC4xNDUgOS4yODEgMTIuNzJhLjc1Ljc1IDAgMDAtMS4wNjIgMS4wNThsMS45NDMgMS45NWEuNzUuNzUgMCAwMDEuMDU1LjAwOGw0LjU1Ny00LjQ1eiI+PC9wYXRoPjwvc3ZnPg==

[future shells]: https://github.com/users/superatomic/projects/1
[repo]: https://github.com/superatomic/xshe/
[crates]: https://crates.io/crates/xshe

[libraries.io tree]: https://libraries.io/cargo/xshe/tree?kind=normal

[gh release]: https://github.com/superatomic/xshe/releases/
[release workflows]: https://github.com/superatomic/xshe/actions/workflows/release.yml

[TOML]: https://toml.io/en/
