#![allow(clippy::type_complexity)]

use crate::{
    components::MainCameraTag,
    resources::{beat::Beat, wave::Difficulty, wave::Wave},
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct BeatSystem;

impl<'s> System<'s> for BeatSystem {
    type SystemData = (
        Write<'s, Beat>,
        Write<'s, Wave>,
        ReadStorage<'s, AudioSource>,
        ReadStorage<'s, MainCameraTag>,
    );

    fn run(&mut self, (mut beat, mut waves, audio_sources, main_cameras): Self::SystemData) {
        if let Some((audio_source, _)) = (&audio_sources, &main_cameras).join().next() {
            if let Some(current_time) = audio_source.current_time() {
                beat.process(current_time.into());
            }
        }

        waves.airplane_speed = (0.15 - 0.1 * beat.progress()).max(0.05);
        waves.available_letters = (5 + (beat.progress() * 11.0) as usize).min(15);

        match waves.difficulty {
            Difficulty::Easy => waves.current_level = beat.progress() * 3.9,
            Difficulty::Medium => waves.current_level = beat.progress() * 6.9,
            Difficulty::Hard => waves.current_level = beat.progress() * 10.9,
        }
    }
}
