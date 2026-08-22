use raylib::audio::{Music, RaylibAudio, Sound};

pub struct Jukebox<'a> {
    footstep: Sound<'a>,
    victory: Sound<'a>,
    bgm: Option<Music<'a>>,
    footstep_timer: f32,
}

impl<'a> Jukebox<'a> {
    pub fn load(audio: &'a RaylibAudio) -> Self {
        let footstep = audio
            .new_sound("assets/audio/pasos.ogg")
            .expect("no se pudo cargar 'assets/audio/pasos.ogg'");

        let victory = audio
            .new_sound("assets/audio/victoria.ogg")
            .expect("no se pudo cargar 'assets/audio/victoria.ogg'");

        // bgm.ogg es opcional; avisa y continua si no existe.
        let bgm = match audio.new_music("assets/audio/bgm.ogg") {
            Ok(b) => {
                b.play_stream();
                Some(b)
            }
            Err(_) => {
                eprintln!(
                    "bgm.ogg no encontrado en assets/audio/; el juego continua sin musica de fondo"
                );
                None
            }
        };

        Self {
            footstep,
            victory,
            bgm,
            footstep_timer: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, is_walking: bool) {
        if let Some(ref mut bgm) = self.bgm {
            bgm.update_stream();
        }

        if is_walking {
            if self.footstep_timer <= 0.0 {
                self.footstep.play();
                self.footstep_timer = crate::FOOTSTEP_INTERVAL;
            } else {
                self.footstep_timer -= dt;
            }
        } else {
            self.footstep_timer = 0.0;
        }
    }

    pub fn play_victory(&self) {
        self.victory.play();
    }
}
