use image::RgbaImage;

/// Texturas de pared cargadas al inicio y listas para muestrear por (u, v).
pub struct Atlas {
    walls: Vec<RgbaImage>, // [pared1, pared2, pared3, puerta final]
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
        }
    }

    /// Devuelve el color RGB del texel en coordenadas (u, v) dentro de la textura `idx`.
    pub fn wall_pixel(&self, idx: usize, u: f32, v: f32) -> u32 {
        let img = &self.walls[idx];
        let tx = ((u * img.width() as f32) as u32).min(img.width() - 1);
        let ty = ((v * img.height() as f32) as u32).min(img.height() - 1);
        let p = img.get_pixel(tx, ty);
        ((p[0] as u32) << 16) | ((p[1] as u32) << 8) | (p[2] as u32)
    }
}

fn open(path: &str) -> RgbaImage {
    image::open(path)
        .unwrap_or_else(|_| {
            panic!("no se pudo cargar '{path}'; colócalo en la carpeta assets/textures/")
        })
        .to_rgba8()
}
