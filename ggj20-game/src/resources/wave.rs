use std::collections::HashMap;

pub const LETTERS: &[u8] = &[b'e', b'a', b'r', b'i', b'o', b't', b'n', b's', b'l', b'c'];

pub const COMBO_STEP: i32 = 25;
pub const COMBO_MAX_STEPS: i32 = 5;
pub const COMBO_INCREASE: i32 = 5;
pub const COMBO_DECREASE: i32 = 1;
pub const MAX_COMBO: i32 = COMBO_STEP * COMBO_MAX_STEPS;

#[derive(Debug, Default, Clone)]
pub struct Wave {
    pub airplanes_count: i32,
    pub airplane_interval: f64,
    pub airplane_letters: HashMap<u8, bool>,
    pub is_paused: bool,

    pub available_letters: usize,
    pub current_start_letter: usize,

    pub current_level: usize,
    pub combo: i32,
}

impl Wave {
    pub fn new(airplanes_count: i32, airplane_interval: f64) -> Self {
        let mut wave = Self {
            airplanes_count,
            airplane_interval,
            airplane_letters: HashMap::new(),
            is_paused: false,
            available_letters: 10,
            current_start_letter: 0,
            current_level: 0,
            combo: 0,
        };

        for c in b'a'..=b'z' {
            wave.airplane_letters.insert(c, false);
        }

        wave
    }

    pub fn decrease_combo(&mut self) {
        self.combo = (self.combo - COMBO_DECREASE).max(0);
    }

    pub fn increase_combo(&mut self) {
        self.combo = (self.combo + COMBO_INCREASE).min(MAX_COMBO);
    }

    pub fn get_combo_multiplier(&self) -> i32 {
        (self.combo / COMBO_STEP).max(1)
    }
}
