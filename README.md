# Rust-implemented Virtual enVironment manager

Based on Python third-party library `virtualenv`

## Usage

Prerequisites

1. Pre-install some version of Python, and install
`virtualenv` with executable in PATH.
2. Set the environment variable `PYTHON_VENV_PATH` to
where you want to place the envs, and make sure it exists.

Then

There are four subcommands to operate the envs

- `add` create a new env, you can specify an existing Python version
- `list` alias `ls` list all envs
- `remove` alias `rm` remove an existing env
- `use` activate an existing env
