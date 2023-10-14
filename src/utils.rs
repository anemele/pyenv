use crate::constants::PYTHON_VENV_PATH_KEY;
use std::env;
use std::path::Path;

pub fn get_venv_path() -> String {
    return match env::var(PYTHON_VENV_PATH_KEY) {
        Ok(val) => val,
        Err(_) => {
            eprintln!("No environment variable {PYTHON_VENV_PATH_KEY} set");
            String::new()
        }
    };
}

pub fn is_valid_env(path: &Path) -> bool {
    path.join("pyvenv.cfg").exists() && (path.join("Scripts").exists() || path.join("bin").exists())
}
