#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Env {
    pub name: String,
    pub ver: String,
    pub libs: Vec<String>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct EnvManifest {
    pub env: Vec<Env>,
}
