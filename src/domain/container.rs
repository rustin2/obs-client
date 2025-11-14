use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum ContainerFormat {
    Mp4,
    Mkv,
}
