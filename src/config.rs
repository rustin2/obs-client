use crate::domain::*;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RootConfig {
    recording: RecordingConfig,
    cloud: CloudConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigError {
    #[error("could not read configuration file: `{0:?}`: {1}")]
    Io(PathBuf, #[source] std::io::Error),

    #[error("failed to parse TOML: {0}")]
    Toml(toml::de::Error),


}

// create load function that reads the config 

