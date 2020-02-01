use oxygengine::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct Wave {
    pub airplanes_count: i32,
    pub airplane_interval: f64,
    pub airplane_letters: HashMap<u8, Option<Entity>>,
    pub is_paused: bool
}

impl Wave {
    pub fn new(airplanes_count: i32, airplane_interval: f64) -> Self {
        let mut wave = Self {
            airplanes_count,
            airplane_interval,
            airplane_letters: HashMap::new(),
            is_paused: false
        };

        for c in (' ' as u8)..('~' as u8) {
            wave.airplane_letters.insert(c, None);
        }

        wave
    }
}
