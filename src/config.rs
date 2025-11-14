use crate::domain::*;
use serde::Deserialize;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Deserialize)]
struct RootConfig {
    recording: RecordingConfig,
    cloud: CloudConfig,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("could not read configuration file: `{0:?}`: {1}")]
    Io(PathBuf, #[source] std::io::Error),

    #[error("failed to parse TOML: {0}")]
    Toml(#[from] toml::de::Error),
}

pub fn load(path: Option<PathBuf>) -> Result<(RecordingConfig, CloudConfig), ConfigError> {
    let path = path.unwrap_or_else(|| PathBuf::from("config.toml"));

    let contents = std::fs::read_to_string(&path).map_err(|e| ConfigError::Io(path.clone(), e))?;
    let root: RootConfig = toml::from_str(&contents)?;
    Ok((root.recording, root.cloud))
}
