use std::path::PathBuf;
use super::*;

#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub profile_name: String,
    pub resolution: Resolution,
    pub fps: u32,
    pub video_bitrate_kbps: u32,
    pub audio_bitrate_kbps: u32,
    pub encoder: Encoder,
    pub container: ContainerFormat,
    pub capture: CaptureTarget,
    pub output_dir: PathBuf,
}

#[derive(Debug, Clone)]
pub struct CloudConfig {
    pub enabled: bool,
    pub bucket: Option<String>,
    pub prefix: Option<String>,
}
