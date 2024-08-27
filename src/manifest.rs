use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Env {
    pub name: String,
    pub ver: String,
    pub libs: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvManifest {
    pub env: Vec<Env>,
}
