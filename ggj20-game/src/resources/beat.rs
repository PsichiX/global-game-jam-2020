use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Beat {
    pub title: String,
    pub bpm: usize,
    #[serde(default)]
    pub base_offset_seconds: f64,
    #[serde(default = "Beat::default_speed")]
    pub speed: f64,
    #[serde(skip)]
    current_time_seconds: f64,
    #[serde(skip)]
    last_time_seconds: f64,
}

impl Beat {
    fn default_speed() -> f64 {
        1.0
    }

    pub fn current_time_seconds(&self) -> f64 {
        self.current_time_seconds
    }

    pub fn beat_duration(&self) -> f64 {
        60.0 / (self.bpm as f64)
    }

    pub fn current_beat_time(&self) -> f64 {
        (self.current_time_seconds - self.base_offset_seconds) % self.beat_duration()
    }

    pub fn beats_count(&self, time_seconds: f64) -> usize {
        ((time_seconds - self.base_offset_seconds) / self.beat_duration()) as usize
    }

    pub fn current_beats_count(&self) -> usize {
        self.beats_count(self.current_time_seconds)
    }

    pub fn current_beat_phase(&self) -> f64 {
        self.current_beat_time() / self.beat_duration()
    }

    pub fn time_to_next_beat(&self) -> f64 {
        self.beat_duration() - self.current_beat_time()
    }

    pub fn phase_to_next_beat(&self) -> f64 {
        1.0 - self.current_beat_phase()
    }

    pub fn is_sync_with_beat(&self, treshold_seconds: f64) -> bool {
        let time = self.current_beat_time().abs();
        time < treshold_seconds || time > self.beat_duration() - treshold_seconds
    }

    pub fn pulse(&self) -> bool {
        self.beats_count(self.last_time_seconds) != self.beats_count(self.current_time_seconds)
    }

    pub fn process(&mut self, current_time_seconds: f64) -> bool {
        self.last_time_seconds =
            std::mem::replace(&mut self.current_time_seconds, current_time_seconds);
        self.pulse()
    }
}
