use std::path::Path;

pub fn is_valid_env(path: &Path) -> bool {
    path.join("pyvenv.cfg").exists() && path.join("Scripts").exists()
}
