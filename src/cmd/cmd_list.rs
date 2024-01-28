use crate::utils::is_valid_env;
use std::{fs::read_dir, path::PathBuf};

pub fn list(venv_path: PathBuf) {
    let Ok(paths) = read_dir(&venv_path) else {
        eprintln!("failed to read dir: {}", venv_path.display());
        return;
    };

    println!("Available envs:");
    for path in paths {
        let Ok(dir) = path else {
            continue;
        };
        if is_valid_env(dir.path().as_path()) {
            println!("  {}", dir.file_name().into_string().unwrap())
        }
    }
}
