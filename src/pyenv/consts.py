from pathlib import Path

PY_ENV_PATH = Path.home() / ".pyenvs"
PY_ENV_PATH.mkdir(exist_ok=True)

PY_VENV_CFG = "pyvenv.cfg"
PY_BIN_DIR = "Scripts"
