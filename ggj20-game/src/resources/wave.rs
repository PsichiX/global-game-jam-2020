use oxygengine::prelude::*;
use std::collections::HashMap;

const LETTERS: &[u8] = &[b'e', b'a', b'r', b'i', b'o', b't', b'n', b's', b'l', b'c'];

#[derive(Debug, Default, Clone)]
pub struct Wave {
    pub airplanes_count: i32,
    pub airplane_interval: f64,
    pub airplane_letters: HashMap<u8, Option<Entity>>,
    pub is_paused: bool,
    pub available_letters: usize,
    pub current_level: usize,
}

impl Wave {
    pub fn new(airplanes_count: i32, airplane_interval: f64) -> Self {
        let mut wave = Self {
            airplanes_count,
            airplane_interval,
            airplane_letters: HashMap::new(),
            is_paused: false,
            available_letters: 1,
            current_level: 0,
        };

        for c in b'a'..=b'z' {
            wave.airplane_letters.insert(c, None);
        }

        wave
    }
}
