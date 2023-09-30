use std::env;
use std::path::Path;
use crate::constants::KEY;

pub fn get_venv_path() -> String {
    match env::var(KEY) {
        Ok(val) => {
            // println!("{}: {}", key, val);
            return val;
        }
        Err(_) => {
            eprintln!("No environment variable {KEY} set");
            return String::new();
        }
    }
}

pub fn exists(path: &String) -> bool {
    Path::new(&path).exists()
}

pub fn is_valid_env(path: &Path) -> bool {
    path.join("pyvenv.cfg").exists() && path.join("Scripts").exists()
}
