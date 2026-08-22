use crate::world::grid::Surface;
use crate::world::Actor;
use crate::world::Grid;

use super::dda;
use super::Framebuffer;

/// Pinta el techo, el piso y las columnas de pared para el frame actual.
/// Llena `zbuf` con la distancia perpendicular de cada columna para el recorte de sprites.
pub fn paint_scene(fb: &mut Framebuffer, actor: &Actor, grid: &Grid, zbuf: &mut [f32]) {
    let w = fb.width;
    let h = fb.height;

    // Fondo: techo arriba, piso abajo
    let mid = h / 2;
    for y in 0..h {
        let bg = if y < mid {
            crate::CEILING_COLOR
        } else {
            crate::FLOOR_COLOR
        };
        for x in 0..w {
            fb.point(x, y, bg);
        }
    }

    // proj_dist: con esta distancia al plano de proyeccion, un tile de alto a 1 tile
    // de distancia proyecta exactamente a proj_dist pixeles de alto en pantalla.
    let proj_dist = (w as f32 * 0.5) / (crate::FOV * 0.5).tan();

    for (col, z) in zbuf[..w].iter_mut().enumerate() {
        let ray_angle = actor.heading + (col as f32 / w as f32 - 0.5) * crate::FOV;

        match dda::cast_ray(actor.pos, ray_angle, grid) {
            None => {
                *z = f32::INFINITY;
            }
            Some(contact) => {
                *z = contact.distance;

                let span = (proj_dist * crate::TILE_UNITS / contact.distance) as i32;
                let top = (h as i32 / 2) - span / 2;
                let bot = top + span;

                // Las caras horizontales (norte/sur) se oscurecen para dar relieve.
                let color = if contact.is_horizontal {
                    darken(flat_color(contact.surface))
                } else {
                    flat_color(contact.surface)
                };

                for row in (top.max(0) as usize)..(bot.min(h as i32) as usize) {
                    fb.point(col, row, color);
                }
            }
        }
    }
}

fn flat_color(surface: Surface) -> u32 {
    match surface {
        Surface::Vertical | Surface::Horizontal | Surface::Post => 0xB87848,
        Surface::Door => 0xD4A030,
    }
}

fn darken(color: u32) -> u32 {
    let r = ((color >> 16) & 0xFF) * 2 / 3;
    let g = ((color >> 8) & 0xFF) * 2 / 3;
    let b = (color & 0xFF) * 2 / 3;
    (r << 16) | (g << 8) | b
}
