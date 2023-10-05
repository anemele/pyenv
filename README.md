# Rust-implemented Virtual enVironment manager

Based on Python virtualenv

> (update)
> To use multiple versions Python to create envs,
> this update uses Windows pylauncher `py.exe`,
> which is not compatible on other platforms.

## Usage

Prerequisites

1. Set the environment variable `PYTHON_VENV_PATH` to
where locates the envs.
2. Make sure where `py.exe` exists is in the PATH and
`virtualenv` is already installed.

Then

There are four subcommands to operate the envs

- `add` create a new env, you can specify an existing Python version
- `list` alias `ls` list all envs
- `remove` alias `rm` remove an existing env
- `use` activate an existing env
