use std::path::Path;

pub fn is_valid_env(path: &Path) -> bool {
    path.join("pyvenv.cfg").exists()
        && if cfg!(windows) {
            path.join("Scripts").exists()
        } else {
            path.join("bin").exists()
        }
}
