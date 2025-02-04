import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from mashumaro.mixins.toml import DataClassTOMLMixin
from virtualenv.run import cli_run

from .consts import PY_BIN_DIR, PY_ENV_PATH, PY_VENV_CFG


def env_exists(env_path: Path) -> bool:
    return (
        env_path.exists()
        and (env_path / PY_BIN_DIR).exists()
        and (env_path / PY_VENV_CFG).exists()
    )


def cmd_add(name: str, python: Optional[str] = None) -> bool:
    env_path = PY_ENV_PATH / name
    if env_exists(env_path):
        print(f"Virtual environment {name} already exists.")
        return False

    args = ["--no-setuptools", "--no-wheel", "--no-vcs-ignore"]
    args.append(str(env_path))
    if python is not None:
        args.extend(["--python", python])

    cli_run(args)
    return True


def _list_env_paths():
    for env_path in PY_ENV_PATH.iterdir():
        if env_path.is_dir() and env_exists(env_path):
            yield env_path


def cmd_list():
    print("Available virtual environments:")
    for env_path in _list_env_paths():
        print(f"  {env_path.name}")


def cmd_remove(name: str):
    env_path = PY_ENV_PATH / name
    if not env_exists(env_path):
        print(f"Virtual environment {name} does not exist.")
        return

    import shutil

    shutil.rmtree(env_path)
    print(f"Virtual environment {name} removed.")


@dataclass
class Env:
    name: str
    python: Optional[str] = field(default=None)
    libs: list[str] = field(default_factory=list)


@dataclass
class Envs(DataClassTOMLMixin):
    env: list[Env]


ENV_FILE = PY_ENV_PATH / ".envs.toml"


def cmd_export(path: Optional[Path] = None):
    if path is None:
        path = ENV_FILE

    envs = list[Env]()
    for env_path in _list_env_paths():
        cfg_text = (env_path / PY_VENV_CFG).read_text()
        s = re.search(r"version_info = (\d\.\d{1,2})", cfg_text)
        if s is None:
            python = None
        else:
            python = s.group(1)

        lib_path = env_path / "lib"
        libs = list[str]()
        for lib in lib_path.glob("**/*.dist-info"):
            tmp = lib.name.removesuffix(".dist-info")
            name, version = tmp.rsplit("-", 1)
            libs.append(f"{name}=={version}")

        env = Env(name=env_path.name, python=python, libs=libs)
        envs.append(env)

    path.write_text(Envs(env=envs).to_toml())
    print(f"Virtual environments exported to {path}.")


def cmd_import(path: Optional[Path] = None):
    if path is None:
        path = ENV_FILE
    if not path.exists():
        print(f"Virtual environments file {path} does not exist.")
        return

    envs = Envs.from_toml(path.read_text())
    for env in envs.env:
        if not cmd_add(env.name, python=env.python):
            continue

        pip_exe = PY_ENV_PATH / env.name / PY_BIN_DIR / "pip.exe"
        res = subprocess.run(
            f"{pip_exe} install {' '.join(env.libs)}",
            capture_output=True,
        )
        if res.returncode != 0:
            print(f"Failed to install packages for {env.name}: {res.stderr}")
        else:
            print(f"Virtual environment {env.name} imported.")
