use crate::utils;
use std::path::Path;
use std::process::Command;

pub fn activate(venv_path: &Path, name: &String, pwsh: bool) {
    let path = venv_path.join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return;
    }

    if !utils::is_valid_env(path.as_path()) {
        eprintln!("Invalid env `{name}`");
        return;
    }

    if pwsh {
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("start pwsh -NoExit")
            .arg("-Command")
            .arg(path.join("Scripts/activate.ps1"))
            .spawn()
            // .output()
            .expect("Failed to activate env");
    } else {
        let _ = Command::new("cmd")
            .arg("/c")
            .arg("start cmd /k")
            .arg(path.join("Scripts/activate.bat"))
            .spawn()
            // .output()
            .expect("Failed to activate env");
    }
}

