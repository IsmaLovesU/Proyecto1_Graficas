use image::RgbaImage;

/// Texturas de pared y frames del sprite de fuego, listos para muestrear por (u, v).
pub struct Atlas {
    walls: Vec<RgbaImage>,       // [pared1, pared2, pared3, puerta final]
    fire_frames: Vec<RgbaImage>, // fuego_1..fuego_8
}

impl Atlas {
    pub fn load() -> Self {
        Self {
            walls: vec![
                open("assets/textures/pared1.png"),
                open("assets/textures/pared2.png"),
                open("assets/textures/pared3.png"),
                open("assets/textures/puerta final.png"),
            ],
            fire_frames: (1..=8)
                .map(|i| open(&format!("assets/sprites/fuego_{i}.png")))
                .collect(),
        }
    }

    /// Devuelve el color RGB del texel en coordenadas (u, v) dentro de la textura de pared `idx`.
    pub fn wall_pixel(&self, idx: usize, u: f32, v: f32) -> u32 {
        sample_rgb(&self.walls[idx], u, v)
    }

    /// Devuelve el color RGBA del texel de fuego; None si el pixel es transparente (alfa < 128).
    pub fn fire_pixel(&self, frame_idx: usize, u: f32, v: f32) -> Option<u32> {
        let img = &self.fire_frames[frame_idx % self.fire_frames.len()];
        let tx = ((u * img.width() as f32) as u32).min(img.width() - 1);
        let ty = ((v * img.height() as f32) as u32).min(img.height() - 1);
        let p = img.get_pixel(tx, ty);
        if p[3] < 128 {
            None
        } else {
            Some(((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32))
        }
    }
}

fn sample_rgb(img: &RgbaImage, u: f32, v: f32) -> u32 {
    let tx = ((u * img.width() as f32) as u32).min(img.width() - 1);
    let ty = ((v * img.height() as f32) as u32).min(img.height() - 1);
    let p = img.get_pixel(tx, ty);
    ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
}

fn open(path: &str) -> RgbaImage {
    image::open(path)
        .unwrap_or_else(|_| panic!("no se pudo cargar '{path}'; colócalo en {path}"))
        .to_rgba8()
}
