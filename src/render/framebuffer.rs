/// Buffer de píxeles en memoria CPU; se vuelca a pantalla a través de una textura de raylib.
/// Cada píxel ocupa 4 bytes en orden R, G, B, A.
pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    /// Bytes RGBA del frame actual. Se pasa a `Texture2D::update_texture` cada frame.
    pub pixels: Vec<u8>,
    background: u32,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0u8; width * height * 4],
            background: 0x000000,
        }
    }

    pub fn set_background(&mut self, color: u32) {
        self.background = color;
    }

    pub fn clear(&mut self) {
        let r = ((self.background >> 16) & 0xFF) as u8;
        let g = ((self.background >> 8) & 0xFF) as u8;
        let b = (self.background & 0xFF) as u8;
        for chunk in self.pixels.as_chunks_mut::<4>().0 {
            chunk[0] = r;
            chunk[1] = g;
            chunk[2] = b;
            chunk[3] = 255;
        }
    }

    pub fn point(&mut self, x: usize, y: usize, color: u32) {
        if x >= self.width || y >= self.height {
            return;
        }
        let i = (y * self.width + x) * 4;
        self.pixels[i] = ((color >> 16) & 0xFF) as u8;
        self.pixels[i + 1] = ((color >> 8) & 0xFF) as u8;
        self.pixels[i + 2] = (color & 0xFF) as u8;
        self.pixels[i + 3] = 255;
    }
}
