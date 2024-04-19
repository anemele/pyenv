use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

const VENV_EXE: &str = "virtualenv";

pub(crate) fn exec<P>(venv_path: P, name: &str, version: Option<String>, force: bool)
where
    P: AsRef<Path>,
{
    let path = venv_path.as_ref().join(name);

    if path.is_file() || (path.is_dir() && !force) {
        eprintln!("Env with the same name `{name}` exists.");
        return;
    }

    let mut cmd = &mut Command::new(VENV_EXE);
    cmd = cmd.arg(path.as_os_str());
    if let Some(ver) = version {
        cmd = cmd.arg("--python").arg(ver);
    }

    // something unnecessary
    cmd = cmd.args(["--no-setuptools", "--no-wheel", "--no-vcs-ignore"]);

    let ok = match cmd
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
            eprintln!(
                "Failed to create env `{name}`: {e}\nMaybe `{}` is not in PATH?",
                VENV_EXE
            );
            return;
        }
    };

    if cfg!(windows) && ok && path.exists() {
        // create idle for Windows
        let idle = path.join("Scripts/idle.bat");
        if let Ok(mut file) = File::create(idle) {
            let _ = file.write(b"@call %~dp0python.exe -m idlelib %*");
        }
    }
}
