use crate::utils::is_valid_env;
use std::fs::remove_dir_all;
use std::path::Path;

pub(crate) fn exec(venv_path: impl AsRef<Path>, name: &str) {
    let path = venv_path.as_ref().join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return;
    }

    if path.is_file() {
        eprintln!("File with the same name `{name}` exists.");
        return;
    }

    if !is_valid_env(&path) {
        eprintln!("Invalid env `{name}`");
        return;
    }

    if remove_dir_all(path).is_ok() {
        println!("Removed env `{name}`")
    } else {
        println!("Failed to remove `{name}`")
    }
}
