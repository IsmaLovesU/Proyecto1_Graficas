use std::fs;

/// Tipo de textura que muestra cada cara de pared sólida.
/// Post, Horizontal y Vertical salen del carácter ASCII del mapa;
/// Door es la textura de la celda meta (puerta final.png).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Vertical,
    Horizontal,
    Post,
    #[allow(dead_code)] // se usa en la etapa 5 al muestrear la textura de la puerta
    Door,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Solid(Surface),
    Floor,
    Spawn,
    Goal,
}

pub struct Grid {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    /// Carga el laberinto desde un archivo ASCII con formato +/-/|.
    /// Las filas de celda (índice impar en el archivo) se duplican para que
    /// los pasillos queden a escala cuadrada — sin esto los corredores
    /// este-oeste se ven el doble de anchos que los norte-sur.
    pub fn load(path: &str) -> Self {
        let content = fs::read_to_string(path).unwrap_or_else(|_| {
            panic!(
                "no se pudo leer '{path}'; \
                 el archivo debe estar en la raíz del proyecto"
            )
        });

        let raw: Vec<&str> = content.lines().collect();
        let width = raw.first().map_or(0, |l| l.len());

        let mut expanded: Vec<&str> = Vec::with_capacity(raw.len() + 9);
        for (i, line) in raw.iter().copied().enumerate() {
            expanded.push(line);
            if i % 2 == 1 {
                // Fila de celda: se duplica para igualar la escala vertical
                // con la horizontal (los pasillos miden 2×1 tiles sin esto).
                expanded.push(line);
            }
        }

        let height = expanded.len();
        let mut tiles = vec![Tile::Floor; width * height];

        for (y, line) in expanded.iter().copied().enumerate() {
            for (x, ch) in line.chars().take(width).enumerate() {
                tiles[y * width + x] = match ch {
                    '|' => Tile::Solid(Surface::Vertical),
                    '-' => Tile::Solid(Surface::Horizontal),
                    '+' => Tile::Solid(Surface::Post),
                    'p' => Tile::Spawn,
                    'g' => Tile::Goal,
                    _ => Tile::Floor,
                };
            }
        }

        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        if x < self.width && y < self.height {
            Some(self.tiles[y * self.width + x])
        } else {
            None
        }
    }

    pub fn is_solid(&self, x: usize, y: usize) -> bool {
        matches!(
            self.tile_at(x, y),
            Some(Tile::Solid(_)) | Some(Tile::Goal) | None
        )
    }

    /// Coordenadas en celdas del primer Spawn encontrado en el mapa.
    pub fn find_spawn(&self) -> (usize, usize) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.tiles[y * self.width + x] == Tile::Spawn {
                    return (x, y);
                }
            }
        }
        (1, 1)
    }
}
