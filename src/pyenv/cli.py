import platform
from pathlib import Path
from typing import Optional

import click

from .cmd import cmd_add, cmd_export, cmd_import, cmd_list, cmd_remove


class OrderedGroup(click.Group):
    def list_commands(self, _):
        return self.commands.keys()


cli = OrderedGroup()


@cli.command(name="add")
@click.argument("name")
@click.option("-p", "--python", help="Python version")
def cli_add(name: str, python: Optional[str] = None):
    """Add a new environment"""
    cmd_add(name, python)


@cli.command(name="ls")
def cli_list():
    """List all available environments"""
    cmd_list()


@cli.command(name="rm")
@click.argument("name")
def cli_remove(name: str):
    """Remove an environment"""
    cmd_remove(name)


if platform.system() == "Linux":
    from .cmd import cmd_use

    @cli.command(name="use")
    @click.argument("name")
    def cli_use(name: str):
        """Return the activate file path"""
        cmd_use(name)


@cli.command(name="export")
@click.argument("name", required=False)
@click.option("-o", "--output", type=Path, help="Output file")
def cli_export(name: Optional[str] = None, output: Optional[Path] = None):
    """Export all environments to a file"""
    cmd_export(name, output)


@cli.command(name="import")
@click.option("-i", "--input", type=Path, help="Input file")
@click.option("-s", "--sync", type=str, help="env to sync")
def cli_import(input: Optional[Path] = None, sync: Optional[str] = None):
    """Import environments from a file"""
    cmd_import(input, sync)


def main():
    try:
        cli()
    except Exception as e:
        print(f"Error: {e}")
        exit(1)


if __name__ == "__main__":
    main()
