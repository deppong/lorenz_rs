pub use sdl2::pixels::Color;

#[derive(Clone, Copy)]
pub struct particle {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub color: Color,
}

impl particle {
    pub fn new(i: f32, c: Color) -> Self {
        particle { x: 0.001 * i, y: 0.0, z: 0.0, color: c  }
    }
}