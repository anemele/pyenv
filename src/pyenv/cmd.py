import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

from mashumaro.mixins.toml import DataClassTOMLMixin
from virtualenv.run import cli_run

from .consts import PY_BIN_DIR, PY_ENV_PATH, PY_VENV_CFG


def _env_exists(env_path: Path) -> bool:
    return (
        env_path.exists()
        and (env_path / PY_BIN_DIR).exists()
        and (env_path / PY_VENV_CFG).exists()
    )


def cmd_add(name: str, python: Optional[str] = None) -> bool:
    env_path = PY_ENV_PATH / name
    if _env_exists(env_path):
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
        if env_path.is_dir() and _env_exists(env_path):
            yield env_path


def cmd_list():
    print("Available virtual environments:")
    for env_path in _list_env_paths():
        print(f"  {env_path.name}")


def cmd_remove(name: str):
    env_path = PY_ENV_PATH / name
    if not _env_exists(env_path):
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


def _get_env(env_path: Path) -> Env:
    cfg_text = (env_path / PY_VENV_CFG).read_text()
    s = re.search(r"version_info = (\d\.\d{1,2})", cfg_text)
    if s is None:
        python = None
    else:
        python = s.group(1)

    libs = list[str]()
    meta_it = env_path.glob("lib/site-packages/*.dist-info/METADATA")
    for libmeta in meta_it:
        if not libmeta.is_file():
            continue

        metatext = libmeta.read_text(encoding="utf-8")
        gs = re.search(r"Name: ([\w-]+)\nVersion: ([\w\.]+)\n", metatext)
        if gs is None:
            # shoud not happen
            continue

        name = gs.group(1)
        version = gs.group(2)
        libs.append(f"{name}=={version}")

    env = Env(name=env_path.name, python=python, libs=libs)
    return env


ENV_FILE = PY_ENV_PATH / ".envs.toml"


def cmd_export(name: Optional[str] = None, path: Optional[Path] = None):
    if name is None:
        env_paths = _list_env_paths()
    else:
        env_paths = [PY_ENV_PATH / name]

    envs = map(_get_env, env_paths)
    envs = Envs(env=list(envs))

    if path is None:
        path = ENV_FILE
        old_envs = Envs.from_toml(path.read_text())
        has_env = {env.name for env in env_paths}
        envs_mapping = {e.name: e for e in old_envs.env if e.name in has_env}
        envs_mapping.update({e.name: e for e in envs.env})
        envs.env[:] = list(envs_mapping.values())

    path.write_text(envs.to_toml())
    print(f"Virtual environments exported to {path}.")


def _sync_libs(env_real: Env, env_to_sync: Env) -> tuple[set[str], set[str]]:
    libs_real = set(env_real.libs)
    libs_to_sync = set(env_to_sync.libs)

    to_uninstall = libs_real - libs_to_sync
    to_install = libs_to_sync - libs_real

    return to_uninstall, to_install


def _cmd_sync_env(env_path: Path, env_to_sync: Env) -> bool:
    env_real = _get_env(env_path)
    to_uninstall, to_install = _sync_libs(env_real, env_to_sync)

    pip = PY_ENV_PATH / env_real.name / PY_BIN_DIR / "python"
    pip = f"{pip} -m pip"
    res = True
    if len(to_uninstall) > 0:
        res_uni = subprocess.run(
            f"{pip} uninstall -y {' '.join(to_uninstall)}",
            capture_output=True,
        )
        res = res and res_uni.returncode == 0
    if len(to_install) > 0:
        res_ins = subprocess.run(
            f"{pip} install {' '.join(to_install)}",
            capture_output=True,
        )
        res = res and res_ins.returncode == 0
    return res


def cmd_import(path: Optional[Path] = None, sync_name: Optional[str] = None):
    if path is None:
        path = ENV_FILE
    if not path.exists():
        print(f"Virtual environments file {path} does not exist.")
        return

    envs = Envs.from_toml(path.read_text())

    if sync_name is not None:
        env_to_sync = None
        for env in envs.env:
            if env.name == sync_name:
                env_to_sync = env
                break
        else:
            print(f"Virtual environment {sync_name} not found in {path}.")

        if env_to_sync is not None:
            envs.env.remove(env_to_sync)
            res = _cmd_sync_env(PY_ENV_PATH / sync_name, env_to_sync)
            if res:
                print(f"Virtual environment {sync_name} synced.")
            else:
                print(f"Failed to sync virtual environment {sync_name}.")

    for env in envs.env:
        if not cmd_add(env.name, python=env.python):
            continue

        pip = PY_ENV_PATH / env.name / PY_BIN_DIR / "python"
        pip = f"{pip} -m pip"
        res = subprocess.run(
            f"{pip} install {' '.join(env.libs)}",
            capture_output=True,
        )
        if res.returncode != 0:
            print(f"Failed to install packages for {env.name}: {res.stderr}")
        else:
            print(f"Virtual environment {env.name} imported.")
