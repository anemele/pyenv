from typing import Optional

import click

from .cmd import cmd_add, cmd_list, cmd_remove


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


def main():
    cli()


if __name__ == "__main__":
    main()
