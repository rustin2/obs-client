use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Encoder {
    X264,
    NVENC,
    Amd,
}
