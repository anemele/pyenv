use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn create(venv_path: &Path, name: &String, version: Option<String>, force: bool) {
    let path = venv_path.join(name);

    if path.is_file() || (path.is_dir() && !force) {
        eprintln!("Env with the same name `{name}` exists.");
        return;
    }

    let venv_exe = "virtualenv";
    let mut cmd = Command::new(venv_exe);
    let mut cmd = cmd.arg(path.as_os_str());
    if let Some(ver) = version {
        cmd = cmd.arg("--python").arg(ver);
    }

    let ok = match cmd
        // The following lines are customized settings
        .args(["--activators", "batch,powershell"])
        .args(["--no-setuptools", "--no-wheel", "--no-vcs-ignore"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
    {
        Ok(status) => {
            if let Some(code) = status.code() {
                code == 0
            } else {
                false
            }
        }

        Err(e) => {
            eprintln!("Failed to create env `{name}`: {e}\nMaybe `{venv_exe}` is not in PATH?");
            return;
        }
    };

    if ok && path.exists() {
        create_idle(&path)
    }
}

fn create_idle(path: &Path) {
    let idle = path.join("Scripts/idle.bat");
    if let Ok(mut file) = fs::File::create(idle) {
        let _ = file.write_all(b"@call %~dp0python.exe -m idlelib %*");
    }
}
