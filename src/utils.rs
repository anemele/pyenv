use super::PYTHON_VENV_PATH;
use std::{
    env,
    path::{Path, PathBuf},
};

pub fn is_valid_env<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    path.join("pyvenv.cfg").exists() && path.join("Scripts").exists()
}

pub fn get_venv_path() -> Option<PathBuf> {
    if let Ok(val) = env::var("USERPROFILE") {
        Some(Path::new(&val).join(PYTHON_VENV_PATH))
    } else {
        eprintln!("failed to get HOME dir");
        None
    }
}
