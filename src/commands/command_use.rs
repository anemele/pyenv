use crate::utils::is_valid_env;
use std::path::Path;
use std::process::Command;

pub fn activate(venv_path: &Path, name: &String, pwsh: bool) {
    let path = venv_path.join(name);
    if !path.exists() {
        eprintln!("No env `{name}` exists.");
        return;
    }

    if !is_valid_env(path.as_path()) {
        eprintln!("Invalid env `{name}`");
        return;
    }

    match if pwsh {
        Command::new("cmd")
            .arg("/c")
            .arg("start pwsh -NoExit -Command")
            .arg(path.join("Scripts/activate.ps1"))
            .status()
    } else {
        Command::new("cmd")
            .arg("/c")
            .arg("start cmd /k")
            .arg(path.join("Scripts/activate.bat"))
            .status()
    } {
        Ok(_) => {}
        Err(e) => eprint!("Failed to activate env `{name}`:\n{e}"),
    }
}
