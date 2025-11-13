#[derive(Debug, Clone)]
pub enum Resolution {
    P720,
    P1080,
    Custom{ width: u16, height: u16 },
}