use crate::consts::PY_BIN_DIR;
use crate::get_venv_path;
use crate::utils::is_valid_env;
use std::fs::read_dir;
use std::process::Command;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Env {
    pub name: String,
    pub libs: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EnvManifest {
    pub env: Vec<Env>,
}

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

    let mut vec = vec![];
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
        let pip = path.join(PY_BIN_DIR).join("pip");
        let Ok(output) = Command::new(pip).arg("freeze").output() else {
            continue;
        };
        let Ok(output) = String::from_utf8(output.stdout) else {
            continue;
        };
        let output = output.trim_end().replace(sep, " ");
        // println!("{}={:#?}", name, output);
        vec.push(Env { name, libs: output });
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
