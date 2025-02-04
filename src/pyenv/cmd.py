from pathlib import Path
from typing import Optional

from virtualenv.run import cli_run

from .consts import PY_BIN_DIR, PY_ENV_PATH, PY_VENV_CFG


def env_exists(env_path: Path) -> bool:
    return (
        env_path.exists()
        and (env_path / PY_BIN_DIR).exists()
        and (env_path / PY_VENV_CFG).exists()
    )


def cmd_add(name: str, python: Optional[str] = None):
    env_path = PY_ENV_PATH / name
    if env_exists(env_path):
        print(f"Virtual environment {name} already exists.")
        return

    args = ["--no-setuptools", "--no-wheel", "--no-vcs-ignore"]
    args.append(str(env_path))
    if python is not None:
        args.extend(["--python", python])

    cli_run(args)


def cmd_list():
    print("Available virtual environments:")
    for env_path in PY_ENV_PATH.iterdir():
        if env_path.is_dir() and env_exists(env_path):
            print(f"  {env_path.name}")


def cmd_remove(name: str):
    env_path = PY_ENV_PATH / name
    if not env_exists(env_path):
        print(f"Virtual environment {name} does not exist.")
        return

    import shutil

    shutil.rmtree(env_path)
    print(f"Virtual environment {name} removed.")
