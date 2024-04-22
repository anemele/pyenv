use crate::consts::PYTHON_VENV_PATH;
use crate::consts::PY_BIN_DIR;
use crate::consts::PY_VENV_CFG;
use homedir::get_my_home;
use std::path::{Path, PathBuf};

pub(crate) fn is_valid_env(path: impl AsRef<Path>) -> bool {
    let path = path.as_ref();

    path.join(PY_VENV_CFG).exists() && path.join(PY_BIN_DIR).exists()
}

pub(crate) fn get_venv_path() -> Option<PathBuf> {
    let Ok(Some(home)) = get_my_home() else {
        return None;
    };

    Some(home.join(PYTHON_VENV_PATH))
}
