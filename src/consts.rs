pub(crate) const PYTHON_VENV_PATH: &str = ".pyenvs";
pub(crate) const PY_VENV_CFG: &str = "pyvenv.cfg";
#[cfg(target_family = "unix")]
pub(crate) const PY_BIN_DIR: &str = "bin";
#[cfg(target_family = "windows")]
pub(crate) const PY_BIN_DIR: &str = "Scripts";
