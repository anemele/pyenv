use crate::get_venv_path;
use crate::utils::is_valid_env;
use std::fs::read_dir;

pub fn exec() {
    let venv_path = get_venv_path();

    let Ok(paths) = read_dir(&venv_path) else {
        eprintln!("failed to read dir: {}", venv_path.display());
        return;
    };

    println!("Available envs:");
    for path in paths {
        let Ok(dir) = path else {
            continue;
        };
        if !is_valid_env(dir.path().as_path()) {
            continue;
        }
        if let Ok(name) = dir.file_name().into_string() {
            println!("  {}", name)
        }
    }
}
