use crate::utils::is_valid_env;
use std::fs::remove_dir_all;
use std::path::Path;

pub fn remove(venv_path: &Path, name: &String) {
    let path = venv_path.join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return;
    }

    if path.is_file() {
        eprintln!("File with the same name `{name}` exists.");
        return;
    }

    if !is_valid_env(path.as_path()) {
        eprintln!("Invalid env `{name}`");
        return;
    }

    match remove_dir_all(path) {
        Ok(_) => {
            println!("Removed env `{name}`");
        }
        Err(_) => {
            println!("Failed to remove `{name}`");
        }
    }
}
