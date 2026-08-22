use crate::world::grid::{Grid, Surface, Tile};

pub struct Contact {
    /// Distancia perpendicular al plano de proyeccion, en unidades de mundo.
    /// El DDA la da libre de ojo de pez sin necesidad de correccion posterior.
    pub distance: f32,
    /// Fraccion horizontal dentro del tile impactado (coordenada U para textura).
    #[allow(dead_code)] // se lee en la etapa 5 al muestrear las texturas de pared
    pub column_offset: f32,
    pub surface: Surface,
    /// true = cara norte/sur (limite en Y), false = cara este/oeste (limite en X).
    pub is_horizontal: bool,
}

/// Lanza un rayo desde `origin` en direccion `angle` y devuelve el primer tile solido.
/// Trabaja en espacio de tiles; la distancia del Contact esta en unidades de mundo.
pub fn cast_ray(origin: (f32, f32), angle: f32, grid: &Grid) -> Option<Contact> {
    let dir_x = angle.cos();
    let dir_y = angle.sin();

    // Posicion fraccionaria en espacio de tiles
    let px = origin.0 / crate::TILE_UNITS;
    let py = origin.1 / crate::TILE_UNITS;

    let mut map_x = px as i32;
    let mut map_y = py as i32;

    // Cuanto crece t al cruzar un limite de celda en cada eje
    let delta_x = if dir_x == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / dir_x).abs()
    };
    let delta_y = if dir_y == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / dir_y).abs()
    };

    let (step_x, mut side_x) = if dir_x < 0.0 {
        (-1i32, (px - map_x as f32) * delta_x)
    } else {
        (1i32, (map_x as f32 + 1.0 - px) * delta_x)
    };

    let (step_y, mut side_y) = if dir_y < 0.0 {
        (-1i32, (py - map_y as f32) * delta_y)
    } else {
        (1i32, (map_y as f32 + 1.0 - py) * delta_y)
    };

    for _ in 0..256 {
        // Avanza al siguiente limite de celda
        let is_horizontal = if side_x < side_y {
            side_x += delta_x;
            map_x += step_x;
            false
        } else {
            side_y += delta_y;
            map_y += step_y;
            true
        };

        // t en el punto de impacto (unidades de tile)
        let perp_t = if is_horizontal {
            side_y - delta_y
        } else {
            side_x - delta_x
        };

        let column_offset = |t: f32| -> f32 {
            if is_horizontal {
                let hx = px + t * dir_x;
                hx - hx.floor()
            } else {
                let hy = py + t * dir_y;
                hy - hy.floor()
            }
        };

        match grid.tile_at(map_x as usize, map_y as usize) {
            None => return None,
            Some(Tile::Solid(surface)) => {
                return Some(Contact {
                    distance: perp_t * crate::TILE_UNITS,
                    column_offset: column_offset(perp_t),
                    surface,
                    is_horizontal,
                });
            }
            Some(Tile::Goal) => {
                return Some(Contact {
                    distance: perp_t * crate::TILE_UNITS,
                    column_offset: column_offset(perp_t),
                    surface: Surface::Door,
                    is_horizontal,
                });
            }
            Some(_) => {}
        }
    }
    None
}
