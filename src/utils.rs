use super::consts::PYTHON_VENV_PATH;
use homedir::get_my_home;
use std::path::{Path, PathBuf};

pub(crate) fn is_valid_env<P>(path: P) -> bool
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    path.join("pyvenv.cfg").exists() && path.join("Scripts").exists()
}

pub fn get_venv_path() -> Option<PathBuf> {
    let Ok(home) = get_my_home() else {
        return None;
    };
    let Some(home) = home else {
        return None;
    };

    Some(home.join(PYTHON_VENV_PATH))
}
