#[derive(Debug, Clone)]
pub struct Atom {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Atom {
    pub fn new() -> Self {
        Atom {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }
}