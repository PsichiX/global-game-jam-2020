#![allow(clippy::type_complexity)]

use crate::{components::MainCameraTag, resources::beat::Beat};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct BeatSystem;

impl<'s> System<'s> for BeatSystem {
    type SystemData = (
        Write<'s, Beat>,
        ReadStorage<'s, AudioSource>,
        ReadStorage<'s, MainCameraTag>,
    );

    fn run(&mut self, (mut beat, audio_sources, main_cameras): Self::SystemData) {
        if let Some((audio_source, _)) = (&audio_sources, &main_cameras).join().next() {
            if let Some(current_time) = audio_source.current_time() {
                beat.process(current_time.into());
            }
        }
    }
}
