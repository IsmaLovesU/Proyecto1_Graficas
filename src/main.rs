mod world;

use raylib::prelude::*;
use world::Grid;

fn main() {
    let (mut rl, thread) = raylib::init().size(900, 600).title("Laberinto").build();

    let grid = Grid::load("maze.txt");
    let (sx, sy) = grid.find_spawn();

    assert_eq!(grid.width, 28, "el mapa debería tener 28 columnas");
    assert_eq!(
        grid.height, 28,
        "la duplicación de filas debería producir 28 filas"
    );
    assert!(
        !grid.is_solid(sx, sy),
        "la celda de spawn no debería ser sólida"
    );

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
    }
}
