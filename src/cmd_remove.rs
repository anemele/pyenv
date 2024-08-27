use anyhow::anyhow;

use crate::get_venv_path;
use crate::utils::is_valid_env;
use std::fs;

pub fn exec(name: &str) -> anyhow::Result<()> {
    let path = get_venv_path()?.join(name);
    if !path.exists() {
        return Err(anyhow!("No env `{name}` exists."));
    }

    if path.is_file() {
        return Err(anyhow!("File with the same name `{name}` exists."));
    }

    if !is_valid_env(&path) {
        return Err(anyhow!("Invalid env `{name}`"));
    }

    if fs::remove_dir_all(path).is_err() {
        return Err(anyhow!("Failed to remove `{name}`"));
    }

    println!("Removed env `{name}`");
    Ok(())
}
