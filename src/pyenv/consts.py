import platform
from pathlib import Path

PY_ENV_PATH = Path.home() / ".pyenvs"
PY_ENV_PATH.mkdir(exist_ok=True)

PY_VENV_CFG = "pyvenv.cfg"

if platform.system() == "Windows":
    PY_BIN_DIR = "Scripts"
else:
    PY_BIN_DIR = "bin"
