use crate::world::grid::Tile;
use crate::world::{Actor, Grid};

use super::Framebuffer;

const SCALE: usize = 5; // pixeles por tile en el minimapa
const MARGIN: usize = 6;

/// Pinta el minimapa en la esquina inferior derecha del framebuffer.
pub fn paint_overlay(fb: &mut Framebuffer, actor: &Actor, grid: &Grid) {
    let map_w = grid.width * SCALE;
    let map_h = grid.height * SCALE;

    // Origen del minimapa dentro del framebuffer
    let ox = fb.width.saturating_sub(map_w + MARGIN);
    let oy = fb.height.saturating_sub(map_h + MARGIN);

    for ty in 0..grid.height {
        for tx in 0..grid.width {
            let color = match grid.tile_at(tx, ty) {
                Some(Tile::Solid(_)) => 0x888888u32,
                Some(Tile::Goal) => 0x40B040,
                _ => 0x1E1E1E,
            };
            for dy in 0..SCALE {
                for dx in 0..SCALE {
                    fb.point(ox + tx * SCALE + dx, oy + ty * SCALE + dy, color);
                }
            }
        }
    }

    // Punto del jugador — cuadrado de 3×3 en blanco
    let pcx = ox + (actor.pos.0 / crate::TILE_UNITS) as usize * SCALE + SCALE / 2;
    let pcy = oy + (actor.pos.1 / crate::TILE_UNITS) as usize * SCALE + SCALE / 2;
    for dy in 0..3usize {
        for dx in 0..3usize {
            let x = pcx.saturating_sub(1) + dx;
            let y = pcy.saturating_sub(1) + dy;
            if x < fb.width && y < fb.height {
                fb.point(x, y, 0xFFFFFF);
            }
        }
    }
}
