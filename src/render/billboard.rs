use std::f32::consts::PI;

use crate::media::Atlas;
use crate::world::Actor;

use super::Framebuffer;

// Posiciones de los fuegos en callejones sin salida, en unidades de mundo.
// Cada par es (x, y); el sprite se dibuja centrado en ese punto.
const FIRE_POSITIONS: &[(f32, f32)] = &[
    (5.0 * 64.0 + 32.0, 10.0 * 64.0 + 32.0),
    (15.0 * 64.0 + 32.0, 10.0 * 64.0 + 32.0),
    (20.0 * 64.0 + 32.0, 13.0 * 64.0 + 32.0),
];

/// Pinta los sprites de fuego sobre el framebuffer usando el z-buffer de paredes para
/// descartar los pixeles que queden detras de una pared.
/// `frame_idx` se calcula fuera para que la velocidad de animacion no dependa del FPS.
pub fn paint_sprites(
    fb: &mut Framebuffer,
    actor: &Actor,
    zbuf: &[f32],
    atlas: &Atlas,
    frame_idx: usize,
) {
    let w = fb.width as f32;
    let h = fb.height as f32;
    let proj_dist = (w * 0.5) / (crate::FOV * 0.5).tan();

    for &(sx, sy) in FIRE_POSITIONS {
        let dx = sx - actor.pos.0;
        let dy = sy - actor.pos.1;
        let dist = (dx * dx + dy * dy).sqrt();
        if dist < 1.0 {
            continue;
        }

        // Angulo relativo al heading; se normaliza a (-π, π]
        let mut rel = dy.atan2(dx) - actor.heading;
        while rel > PI {
            rel -= 2.0 * PI;
        }
        while rel < -PI {
            rel += 2.0 * PI;
        }

        if rel.abs() > crate::FOV * 0.5 + 0.1 {
            continue;
        }

        // Columna de pantalla donde cae el centro del sprite
        let screen_cx = (0.5 + rel / crate::FOV) * w;

        // Alto y ancho proyectados; FIRE_SCALE achica el fuego respecto al alto de una pared
        let sprite_h = (proj_dist * crate::TILE_UNITS / dist * crate::FIRE_SCALE) as i32;
        let sprite_w = sprite_h;

        let top = (h as i32 / 2) - sprite_h / 2;
        let bot = top + sprite_h;
        let left = screen_cx as i32 - sprite_w / 2;
        let right = left + sprite_w;

        for col in left.max(0)..right.min(fb.width as i32) {
            // Descarta si la pared en esta columna esta mas cerca que el sprite
            if zbuf[col as usize] < dist {
                continue;
            }

            let u = (col - left) as f32 / sprite_w as f32;

            for row in top.max(0)..bot.min(fb.height as i32) {
                let v = (row - top) as f32 / sprite_h as f32;

                if let Some(color) = atlas.fire_pixel(frame_idx, u, v) {
                    fb.point(col as usize, row as usize, color);
                }
            }
        }
    }
}
