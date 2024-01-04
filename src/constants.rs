use std::env;

const PYTHON_VENV_PATH_KEY: &str = "PYTHON_VENV_PATH";

pub fn get_venv_path() -> Option<String> {
    if let Ok(val) = env::var(PYTHON_VENV_PATH_KEY) {
        Some(val)
    } else {
        eprintln!("No environment variable {PYTHON_VENV_PATH_KEY} set");
        None
    }
}
