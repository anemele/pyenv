use crate::utils;
use std::{fs, path::Path};

pub fn list(path: &Path) {
    let paths = fs::read_dir(path).unwrap();

    println!("Available envs:");
    for path in paths {
        if let Ok(dir) = path {
            if utils::is_valid_env(dir.path().as_path()) {
                println!("  {}", dir.file_name().into_string().unwrap())
            }
        }
    }
}
