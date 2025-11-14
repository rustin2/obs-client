use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum CaptureTarget {
    Monitor { index: u32 },
    Window { title: String },
    Camera { device: String },
}
