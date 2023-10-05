use crate::utils;
use std::path::Path;
use std::process::Command;

pub fn activate(venv_path: &Path, name: &String) -> i32 {
    let path = venv_path.join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return 1;
    }

    if !utils::is_valid_env(path.as_path()) {
        eprintln!("Invalid env `{name}`");
        return 1;
    }

    let _ = Command::new("cmd")
        .arg("/c")
        .arg("start cmd /k")
        .arg(path.join("Scripts/activate"))
        .spawn()
        // .output()
        .expect("Failed to activate env");
    return 0;
}
