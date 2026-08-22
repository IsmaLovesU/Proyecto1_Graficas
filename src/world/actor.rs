use raylib::prelude::*;

use crate::world::grid::{Grid, Tile};

pub struct Actor {
    pub pos: (f32, f32),
    pub heading: f32,
}

impl Actor {
    pub fn new(spawn: (usize, usize)) -> Self {
        Self {
            pos: (
                spawn.0 as f32 * crate::TILE_UNITS + crate::TILE_UNITS * 0.5,
                spawn.1 as f32 * crate::TILE_UNITS + crate::TILE_UNITS * 0.5,
            ),
            heading: 0.0,
        }
    }

    pub fn update(&mut self, rl: &RaylibHandle, grid: &Grid, dt: f32) {
        // Teclado en rad/s; mouse ya es desplazamiento por frame (no necesita dt).
        let mut turn = 0.0_f32;
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            turn -= crate::ROTATION_SPEED;
        }
        if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            turn += crate::ROTATION_SPEED;
        }
        self.heading += turn * dt + rl.get_mouse_delta().x * crate::MOUSE_SENSITIVITY;
        self.heading = self.heading.rem_euclid(std::f32::consts::TAU);

        let mut step = 0.0_f32;
        if rl.is_key_down(KeyboardKey::KEY_UP) {
            step += crate::WALK_RATE;
        }
        if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            step -= crate::WALK_RATE;
        }

        if step != 0.0 {
            let dx = self.heading.cos() * step * dt;
            let dy = self.heading.sin() * step * dt;
            self.try_move(dx, dy, grid);
        }
    }

    pub fn has_reached_goal(&self, grid: &Grid) -> bool {
        let tx = (self.pos.0 / crate::TILE_UNITS) as usize;
        let ty = (self.pos.1 / crate::TILE_UNITS) as usize;
        matches!(grid.tile_at(tx, ty), Some(Tile::Goal))
    }

    // Mueve en X e Y por separado para que el actor resbale por las paredes
    // en vez de trabarse en las esquinas.
    fn try_move(&mut self, dx: f32, dy: f32, grid: &Grid) {
        let r = crate::PLAYER_RADIUS;
        let nx = self.pos.0 + dx;
        if !self.overlaps_solid(nx, self.pos.1, r, grid) {
            self.pos.0 = nx;
        }
        let ny = self.pos.1 + dy;
        if !self.overlaps_solid(self.pos.0, ny, r, grid) {
            self.pos.1 = ny;
        }
    }

    fn overlaps_solid(&self, x: f32, y: f32, r: f32, grid: &Grid) -> bool {
        [
            (x - r, y - r),
            (x + r, y - r),
            (x - r, y + r),
            (x + r, y + r),
        ]
        .iter()
        .any(|&(cx, cy)| {
            let tx = (cx / crate::TILE_UNITS) as usize;
            let ty = (cy / crate::TILE_UNITS) as usize;
            grid.is_solid(tx, ty)
        })
    }
}
