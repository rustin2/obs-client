#[derive(Debug, Clone)]
pub enum CaptureTarget {
    Monitor { index: u32 },
    Window { title: String },
    Camera { device: String },
}