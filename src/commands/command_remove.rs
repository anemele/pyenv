use crate::utils;
use std::fs;
use std::path::Path;

fn remove_one(venv_path: &Path, name: &String) -> i32 {
    let path = venv_path.join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return 1;
    }

    if path.is_file() {
        eprintln!("File with the same name `{name}` exists.");
        return 1;
    }

    if !utils::is_valid_env(path.as_path()) {
        eprintln!("Invalid env `{name}`");
        return 1;
    }

    match fs::remove_dir_all(path) {
        Ok(_) => {
            println!("Removed env `{name}`");
            0
        }
        Err(_) => {
            println!("Failed to remove `{name}`");
            1
        }
    }
}

pub fn remove(venv_path: &Path, names: &Vec<String>) -> i32 {
    let mut ret = 0;
    for name in names {
        ret += remove_one(venv_path, name);
    }

    if ret == 0 {
        0
    } else {
        1
    }
}
