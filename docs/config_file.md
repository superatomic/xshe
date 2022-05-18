### Creating a `xshe.toml` file

Create a file called `xshe.toml` in `~/.config`. This is a [TOML file][TOML] that represents environment variables.

[![Click for an example `xshe.toml` configuration][icon-example]][example]

One variable is set per line.
The file is read in order from top to bottom,
so you can use variables that appear earlier in the file to define ones that appear later on.

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

#### Setting and unsetting variables

Sometimes all that matters is that a variable is set, and the exact value of the variable does not matter.
**Xshe** has a shorthand for this.
Just set a variable to `true`. *(This is equivalent to setting it to `"1"`)*

```toml
HOMEBREW_NO_ANALYTICS = true  # Disable sending analytics when using Homebrew
```

In addition, you can set variables in the toml to `false` to unset them!
This isn't syntactic sugar like setting variables' values to `true`; it's its own construct.

As an example, the line `HOMEBREW_NO_ANALYTICS = false` in the `xshe.toml` file will
expand to `unset HOMEBREW_NO_ANALYTICS;` on **bash** and **zsh**, and to `set -ge HOMEBREW_NO_ANALYTICS;` on **fish**.

#### Shell-specific environment variables

To set environment variables for only one shell, add a `.NAME` prefix after the name of the environment variable,
where `NAME` is one of `bash`, `zsh`, or `fish`.
These environment variables will only be added if the given shell is used.

As an example, these lines make `$HISTFILE` be set to different values between different shells,
and to have `$ZSH_CACHE_DIR` only be set in **zsh**, do this:

```toml
HISTFILE.bash = "$XDG_STATE_HOME/bash_history"
HISTFILE.zsh = "$XDG_STATE_HOME/zsh_history"

ZSH_CACHE_DIR.zsh = "$XDG_CACHE_HOME/oh-my-zsh"
```

You can use `._` instead of using a shell name to specify a default if an option doesn't apply to any of the shells.
For example, these lines set the `$EDITOR` to `nano` on **bash**, but [`micro`][micro] on everything else:

```toml
EDITOR.bash = "$(which nano)"
EDITOR._ = "$(which micro)"
```

[icon-example]: https://shields.io/badge/Example-xshe.toml-blue?labelColor=blue&color=lightblue&logo=file&logoColor=white&style=for-the-badge
[example]: https://gist.github.com/superatomic/52a46e53a4afce75ede4db7ba6354e0a

[TOML]: https://toml.io/en/
[micro]: https://micro-editor.github.io/
