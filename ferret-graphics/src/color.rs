#[derive(Copy, Clone)]
pub struct Color(pub f32, pub f32, pub f32);

impl Color {
    pub fn as_rgb888(&self) -> (u8, u8, u8) {
        let r = (self.0 * 255.0) as u8;
        let g = (self.1 * 255.0) as u8;
        let b = (self.2 * 255.0) as u8;

        (r, g, b)
    }
}