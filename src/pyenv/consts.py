import platform
from pathlib import Path

PY_ENV_PATH = Path.home() / ".pyenvs"
PY_ENV_PATH.mkdir(exist_ok=True)

PY_VENV_CFG = "pyvenv.cfg"

match platform.system():
    case "Windows":
        PY_BIN_DIR = "Scripts"
    case "Linux":
        PY_BIN_DIR = "bin"
    case _:  # do not know aboud Other OS
        PY_BIN_DIR = "bin"
