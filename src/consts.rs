pub const PYTHON_VENV_PATH: &str = ".pyenvs";
pub const PY_VENV_CFG: &str = "pyvenv.cfg";
#[cfg(target_family = "unix")]
pub const PY_BIN_DIR: &str = "bin";
#[cfg(target_family = "windows")]
pub const PY_BIN_DIR: &str = "Scripts";
