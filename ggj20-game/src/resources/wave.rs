use std::collections::HashMap;

pub const LETTERS: &[u8] = &[b'e', b'a', b'r', b'i', b'o', b't', b'n', b's', b'l', b'c', b'w', b'g', b'k', b'z', b'b'];

pub const COMBO_STEP: i32 = 50;
pub const COMBO_MAX_STEPS: i32 = 5;
pub const COMBO_INCREASE: i32 = 10;
pub const COMBO_DECREASE: i32 = 1;
pub const MAX_COMBO: i32 = COMBO_STEP * COMBO_MAX_STEPS;
pub const BEAT_THRESHOLD: f64 = 0.1;

// !!! Do not forget to also change this value in city.yaml!
pub const CITY_INITIAL_INFECTION_RATE: i32 = 7;

#[derive(Debug, Clone)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

impl Default for Difficulty {
    fn default() -> Difficulty {
        Difficulty::Easy
    }
}

#[derive(Debug, Default, Clone)]
pub struct Wave {
    pub airplanes_count: i32,
    pub airplane_interval: f64,
    pub airplane_letters: HashMap<u8, bool>,
    pub is_paused: bool,

    pub available_letters: usize,
    pub current_start_letter: usize,

    pub current_level: f32,
    pub combo: i32,
    pub score: i32,
    pub difficulty: Difficulty,
    pub plane_spawning_every_beats: i32,
    pub airplane_speed: f32
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
            current_level: 0.0,
            combo: 0,
            score: 0,
            difficulty: Difficulty::Hard,
            plane_spawning_every_beats: 1,
            airplane_speed: 0.15
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
        (self.combo / COMBO_STEP + 1).max(1)
    }
}
