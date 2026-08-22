use raylib::prelude::*;

use crate::media::Atlas;
use crate::render::billboard::paint_sprites;
use crate::render::columns::paint_scene;
use crate::render::overlay::paint_overlay;
use crate::render::Framebuffer;
use crate::world::{Actor, Grid};

use super::{Clock, Stage};

pub fn run(mut rl: RaylibHandle, thread: RaylibThread, grid: Grid) {
    let atlas = Atlas::load();
    let mut actor = Actor::new(grid.find_spawn());

    let mut fb = Framebuffer::new(crate::SCREEN_WIDTH as usize, crate::SCREEN_HEIGHT as usize);
    let mut zbuf = vec![f32::INFINITY; crate::SCREEN_WIDTH as usize];

    let img = Image::gen_image_color(crate::SCREEN_WIDTH, crate::SCREEN_HEIGHT, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &img)
        .expect("no se pudo crear la textura del framebuffer");

    let mut clock = Clock::new();
    let mut stage = Stage::Welcome;

    while !rl.window_should_close() {
        clock.tick(rl.get_frame_time());

        match stage {
            Stage::Welcome => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    stage = Stage::Playing;
                    rl.disable_cursor();
                }
            }
            Stage::Playing => {
                actor.update(&rl, &grid, clock.dt);
                if actor.has_reached_goal(&grid) {
                    stage = Stage::Success;
                    rl.enable_cursor();
                }
            }
            Stage::Success => {
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    actor = Actor::new(grid.find_spawn());
                    stage = Stage::Playing;
                    rl.disable_cursor();
                }
            }
        }

        match stage {
            Stage::Welcome | Stage::Success => {
                fb.set_background(0x1A1A26);
                fb.clear();
            }
            Stage::Playing => {
                paint_scene(&mut fb, &actor, &grid, &mut zbuf, &atlas);
                // El frame avanza por tiempo (no por numero de render) para que la
                // velocidad de la llama no dependa de los FPS del juego.
                let frame_idx = (clock.total * 10.0) as usize % 8;
                paint_sprites(&mut fb, &actor, &zbuf, &atlas, frame_idx);
                paint_overlay(&mut fb, &actor, &grid);
            }
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
