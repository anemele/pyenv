use crate::consts::PY_BIN_DIR;
use crate::get_venv_path;
use crate::manifest::{Env, EnvManifest};
use crate::utils::is_valid_env;
use std::fs::read_dir;
use std::process::{Command, Stdio};

const PREFIX: &str = "pyvm_manifest";

fn export_filename() -> String {
    let now = chrono::Local::now().date_naive();
    format!("{PREFIX}_{now}.toml")
}

pub fn exec(output: Option<String>) {
    let Some(s) = export_library() else {
        eprintln!("failed to export envs");
        return;
    };
    let outpath = output.unwrap_or(export_filename());
    if std::fs::write(&outpath, s).is_ok() {
        println!("manifest has been written at {}", outpath);
    } else {
        eprintln!("failed to write manifest.")
    };
}

fn export_library() -> Option<String> {
    let venv_path = get_venv_path();

    let Ok(paths) = read_dir(&venv_path) else {
        eprintln!("failed to read dir: {}", venv_path.display());
        return None;
    };

    let sep = if cfg!(windows) {
        "\r\n"
    } else if cfg!(darwin) {
        "\r"
    } else {
        "\n"
    };

    let mut pipe = vec![];
    for path in paths {
        let Ok(dir) = path else {
            continue;
        };
        let path = dir.path();
        if !is_valid_env(&path) {
            continue;
        }
        let Ok(name) = dir.file_name().into_string() else {
            continue;
        };

        let py = path.join(PY_BIN_DIR).join("python");
        let Ok(child1) = Command::new(py)
            .arg("--version")
            .stdout(Stdio::piped())
            .spawn()
        else {
            continue;
        };
        let pip = path.join(PY_BIN_DIR).join("pip");
        let Ok(child2) = Command::new(pip)
            .arg("freeze")
            .stdout(Stdio::piped())
            .spawn()
        else {
            continue;
        };
        pipe.push((name, child1, child2));
    }

    let mut vec = vec![];
    for (name, child1, child2) in pipe {
        let Ok(output) = child1.wait_with_output() else {
            continue;
        };
        let Ok(output) = String::from_utf8(output.stdout) else {
            continue;
        };
        let Some(ver) = output.strip_prefix("Python") else {
            continue;
        };
        let Ok(output) = child2.wait_with_output() else {
            continue;
        };
        let Ok(output) = String::from_utf8(output.stdout) else {
            continue;
        };
        let output = output
            .trim_end()
            .split(sep)
            .map(|s| s.to_string())
            .collect();
        vec.push(Env {
            name,
            ver: ver.trim().to_string(),
            libs: output,
        });
    }
    // dbg!(&vec);

    match toml::to_string(&EnvManifest { env: vec }) {
        Ok(s) => Some(s),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}
