use std::process::Command;

use crate::{cmd_export::EnvManifest, consts::PY_BIN_DIR, get_venv_path};

fn read_manifest(manifest: &str) -> Option<EnvManifest> {
    let Ok(s) = std::fs::read_to_string(&manifest) else {
        return None;
    };
    let Ok(t) = toml::from_str(&s) else {
        return None;
    };

    Some(t)
}

pub fn exec(manifest: String) {
    let Some(t) = read_manifest(&manifest) else {
        return;
    };
    // dbg!(&t);

    let venv_path = get_venv_path();
    for env in t.env {
        crate::cmd_add::exec(&env.name, None, false);
        let pip = venv_path.join(env.name).join(PY_BIN_DIR).join("pip");
        let Ok(child) = Command::new(pip).arg("install").args(env.libs).spawn() else {
            continue;
        };
        let _ = child.wait_with_output();
    }
}
