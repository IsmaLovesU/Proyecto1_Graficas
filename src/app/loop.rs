use raylib::prelude::*;

use crate::render::Framebuffer;
use crate::world::Grid;

use super::{Clock, Stage};

pub fn run(mut rl: RaylibHandle, thread: RaylibThread, grid: Grid) {
    // Posición del actor; se usa en la etapa 3 al crear Actor.
    let _spawn = grid.find_spawn();

    let mut fb = Framebuffer::new(crate::SCREEN_WIDTH as usize, crate::SCREEN_HEIGHT as usize);

    let img = Image::gen_image_color(crate::SCREEN_WIDTH, crate::SCREEN_HEIGHT, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &img)
        .expect("no se pudo crear la textura del framebuffer");

    let mut clock = Clock::new();
    let mut stage = Stage::Welcome;

    while !rl.window_should_close() {
        clock.tick(rl.get_frame_time());

        // Actualización de estado según la entrada del teclado
        match stage {
            Stage::Welcome => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    stage = Stage::Playing;
                    rl.disable_cursor();
                }
            }
            Stage::Playing => {}
            Stage::Success => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    stage = Stage::Playing;
                    rl.disable_cursor();
                }
            }
        }

        // Render al framebuffer según el estado actual
        match stage {
            Stage::Welcome | Stage::Success => {
                fb.set_background(0x1A1A26);
                fb.clear();
            }
            Stage::Playing => paint_background(&mut fb),
        }

        texture
            .update_texture(&fb.pixels)
            .expect("fallo al actualizar la textura");

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);

        match stage {
            Stage::Welcome => d.draw_text(
                "LABERINTO  --  presiona ENTER para empezar",
                100,
                crate::SCREEN_HEIGHT / 2,
                22,
                Color::WHITE,
            ),
            Stage::Playing => d.draw_fps(10, 10),
            Stage::Success => d.draw_text(
                "Llegaste a la meta!  --  ENTER para jugar de nuevo",
                80,
                crate::SCREEN_HEIGHT / 2,
                22,
                Color::GREEN,
            ),
        }
    }
}

// Pinta el fondo de la vista de juego: media pantalla de techo, media de piso.
// En la etapa 4 esto se reemplaza por columnas con DDA.
fn paint_background(fb: &mut Framebuffer) {
    let mid = fb.height / 2;
    for y in 0..fb.height {
        let color = if y < mid {
            crate::CEILING_COLOR
        } else {
            crate::FLOOR_COLOR
        };
        for x in 0..fb.width {
            fb.point(x, y, color);
        }
    }
}
