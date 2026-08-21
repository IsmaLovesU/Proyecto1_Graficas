mod app;
mod render;
mod world;

use world::Grid;

pub const SCREEN_WIDTH: i32 = 1000;
pub const SCREEN_HEIGHT: i32 = 600;
/// Lado de una celda del laberinto en unidades de mundo.
pub const TILE_UNITS: f32 = 64.0;
/// Campo de visión horizontal en radianes (60°).
pub const FOV: f32 = std::f32::consts::PI / 3.0;
pub const CEILING_COLOR: u32 = 0x1A1A26;
pub const FLOOR_COLOR: u32 = 0x3B2E24;
/// Velocidad de desplazamiento en unidades/segundo.
pub const WALK_RATE: f32 = 160.0;
/// Velocidad de rotación en radianes/segundo (teclado).
pub const ROTATION_SPEED: f32 = 2.5;
/// Giro en radianes por píxel de movimiento horizontal del mouse.
pub const MOUSE_SENSITIVITY: f32 = 0.003;
/// Radio de colisión del actor en unidades de mundo.
pub const PLAYER_RADIUS: f32 = 12.0;
/// Alto de la llama en proporción al alto de una pared a igual distancia.
pub const FIRE_SCALE: f32 = 0.55;
/// Segundos entre pasos consecutivos mientras el actor camina.
pub const FOOTSTEP_INTERVAL: f32 = 0.3;

fn main() {
    let (rl, thread) = raylib::init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Laberinto")
        .build();

    let grid = Grid::load("maze.txt");

    app::run(rl, thread, grid);
}
