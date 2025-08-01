use enigo::{Enigo, Key, Keyboard, Mouse, Settings};
use rand::Rng;
use std::time::Duration;

pub struct JigglerConfig {
    pub interval_seconds: u64,
    pub use_key_press: bool,
}

impl Default for JigglerConfig {
    fn default() -> Self {
        Self {
            interval_seconds: 60,
            use_key_press: false,
        }
    }
}

pub struct Jiggler {
    enigo: Enigo,
    config: JigglerConfig,
}

impl Jiggler {
    pub fn new(config: JigglerConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let enigo = Enigo::new(&Settings::default())?;
        Ok(Self { enigo, config })
    }

    pub fn perform_action(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.config.use_key_press {
            self.press_safe_key()
        } else {
            self.jiggle_mouse()
        }
    }

    fn jiggle_mouse(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut rng = rand::thread_rng();

        let dx = rng.gen_range(-5..=5);
        let dy = rng.gen_range(-5..=5);

        self.enigo.move_mouse(dx, dy, enigo::Coordinate::Rel)?;
        std::thread::sleep(Duration::from_millis(50));
        self.enigo.move_mouse(-dx, -dy, enigo::Coordinate::Rel)?;

        Ok(())
    }

    fn press_safe_key(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.key(Key::Shift, enigo::Direction::Press)?;
        std::thread::sleep(Duration::from_millis(50));
        self.enigo.key(Key::Shift, enigo::Direction::Release)?;

        Ok(())
    }
}
