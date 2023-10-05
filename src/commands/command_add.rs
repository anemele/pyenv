use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

use crate::constants::DEFAULT_PYTHON_VERSION;

pub fn create(venv_path: &Path, name: &String, version: Option<f32>, overwrite: bool) -> i32 {
    let path = venv_path.join(&name);
    if path.is_file() || (path.is_dir() && !overwrite) {
        eprintln!("Env with the same name exists.");
        return 1;
    }

    // Here needs the py.exe in the PATH
    // let mut cmd = &mut Command::new("py");
    // if version.is_some() {
    //     cmd = cmd.arg(format!("-{}", version.unwrap()))
    // }

    let ver = format!("-{}", version.unwrap_or(DEFAULT_PYTHON_VERSION));
    let output = Command::new("py")
        .arg(ver)
        .args(["-m", "virtualenv"])
        .arg(path.as_os_str())
        .args(["--activators", "batch,powershell"])
        .arg("--no-setuptools")
        .arg("--no-wheel")
        .output()
        .expect("Failed to create new env");
    let output_str = String::from_utf8_lossy(&output.stdout);
    println!("{output_str}");

    if path.exists() {
        create_idle(&path)
    } else {
        1
    }
}

fn create_idle(path: &Path) -> i32 {
    let idle = path.join("Scripts/idle.bat");
    match fs::File::create(idle) {
        Ok(mut file) => {
            let _ = file.write_all(b"@call %~dp0python.exe -m idlelib %*");
            0
        }
        Err(_) => 1,
    }
}
