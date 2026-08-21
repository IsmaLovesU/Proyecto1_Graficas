/// Acumula el tiempo de juego y expone el delta del último frame.
#[derive(Default)]
pub struct Clock {
    pub dt: f32,
    pub total: f32,
}

impl Clock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&mut self, dt: f32) {
        self.dt = dt;
        self.total += dt;
    }
}
