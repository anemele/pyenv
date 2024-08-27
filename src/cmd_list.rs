use crate::get_venv_path;
use crate::utils::is_valid_env;
use std::fs;

pub fn exec() -> anyhow::Result<()> {
    let venv_path = get_venv_path()?;

    println!("Available envs:");
    for path in fs::read_dir(&venv_path)? {
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

    Ok(())
}
