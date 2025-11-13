use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Encoder {
    X264,
    NVENC,
    Amd,
}
