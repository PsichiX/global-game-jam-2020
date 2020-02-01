use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Beat {
    pub title: String,
    pub bmp: usize,
    #[serde(default)]
    pub base_offset_seconds: f64,
    #[serde(skip)]
    pub current_time_seconds: f64,
}

impl Beat {
    pub fn beat_duration(&self) -> f64 {
        60.0 / (self.bmp as f64)
    }

    pub fn current_beat_time(&self) -> f64 {
        (self.current_time_seconds - self.base_offset_seconds) % self.beat_duration()
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
}
