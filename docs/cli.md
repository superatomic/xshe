### Sourcing the `xshe.toml` file

Put the line corresponding to your shell in whatever file runs when loading environment variables.
For **bash**, this is `~/.bash_profile`, for **zsh**, this is `~/.zshenv`,
and for **fish**, this is `~/.config/fish/config.fish`.

#### Bash
```bash
# Goes in ~/.bash_profile, ~/.profile, etc.
eval "$(xshe bash)"
```

#### Zsh
```zsh
# Goes in ~/.zshenv (recommended), ~/.zprofile, etc.
eval "$(xshe zsh)"
```

#### Fish
```fish
# Goes in ~/.config/fish/config.fish
xshe fish | source
```

### Use without `xshe` on `PATH`
If `xshe` isn't on your `PATH` (this is the cause of the error `command not found: xshe`),
you will have to manually type out the location:
```shell
eval "$(/path/to/xshe bash)"
```

### Options

#### Using `--file` (or `-f`)

To specify a custom file that is not located at `~/.config/xshe.toml`, pass the `--file` option, like so:

```bash
eval "$(xshe bash --file ~/some/other/location.toml)"
```

#### Using `--text` (or `-t`)

To directly specify TOML to parse as a config file, use `--text`.

For example, this line directly parses the provided line and converts it to bash:
```bash
xshe bash --text "BIN_HOME = '~/.local/bin'"
```

#### Using `--pipe` (or `-p`)

To pass a TOML configuration from the standard input, use `--pipe`.

As an example, this command concatenates two files named
`global_xshe.toml` and `user_xshe.toml` by using `cat`,
and then pipes the output into `xshe` to be parsed:

```shell
cat global_xshe.toml user_xshe.toml | xshe bash --pipe
```

#### Other CLI options

##### Output verbosity

You can control how much info is displayed when `xshe` is run.
The default behavior is to only display errors and warnings.

While this behavior is recommended, you can customize it by using the following flags:

* `-qq` silences all output (this silences errors and is not advised)
* `-q` or `--quiet` shows only errors and hides warnings
* `-v` or `--verbose` shows info messages
* `-vv` shows debug logs
* `-vvv` shows trace logs

##### Help and version information

Run `xshe --help` to display command usage.

Run `xshe --version` to display version information.
