use crate::utils;
use std::{fs, path::Path};

pub fn list(path: &Path) -> i32 {
    let paths = fs::read_dir(&path).unwrap();

    println!("Available envs:");
    for path in paths {
        match path {
            Ok(dir) => {
                if utils::is_valid_env(dir.path().as_path()) {
                    println!("  {}", dir.file_name().into_string().unwrap())
                }
            }
            Err(_) => todo!(),
        }
    }

    return 0;
}
