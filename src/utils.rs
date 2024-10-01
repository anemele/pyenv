use crate::consts::PYTHON_VENV_PATH;
use crate::consts::PY_BIN_DIR;
use crate::consts::PY_VENV_CFG;
use std::path::{Path, PathBuf};

pub fn is_valid_env(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();

    path.join(PY_VENV_CFG).exists() && path.join(PY_BIN_DIR).exists()
}

pub(super) fn get_venv_path() -> anyhow::Result<PathBuf> {
    let home = homedir::get_my_home()?.ok_or(anyhow::anyhow!("failed to get HOME dir"))?;
    Ok(home.join(PYTHON_VENV_PATH))
}
