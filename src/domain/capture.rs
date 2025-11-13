use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptureTarget {
    Monitor { index: u32 },
    Window { title: String },
    Camera { device: String },
}