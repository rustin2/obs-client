use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContainerFormat {
    Mp4,
    Mkv,
}
