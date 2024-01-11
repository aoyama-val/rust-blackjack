use rand::prelude::*;
use std::time;

pub const FPS: i32 = 30;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Command {
    None,
    Hit,
    Stand,
}

pub struct Game {
    pub rng: StdRng,
    pub is_debug: bool,
    pub is_over: bool,
    pub is_clear: bool,
    pub requested_sounds: Vec<&'static str>,
}

impl Game {
    pub fn new() -> Self {
        let now = time::SystemTime::now();
        let timestamp = now
            .duration_since(time::UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!")
            .as_secs();
        let rng = StdRng::seed_from_u64(timestamp);
        println!("random seed = {}", timestamp);
        // let rng = StdRng::seed_from_u64(0);

        let mut game = Game {
            rng: rng,
            is_debug: false,
            is_over: false,
            is_clear: false,
            requested_sounds: Vec::new(),
        };

        game
    }

    pub fn update(&mut self, command: Command) {
        if self.is_over || self.is_clear {
            return;
        }

        match command {
            Command::Hit => {
                println!("Hit")
            }
            Command::Stand => {
                println!("Stand")
            }
            Command::None => {}
        }
    }
}

pub fn clamp<T: PartialOrd>(min: T, value: T, max: T) -> T {
    if value < min {
        return min;
    }
    if value > max {
        return max;
    }
    value
}
