#![allow(clippy::type_complexity)]

use crate::{
    components::{
        VirusTag
    },
    resources::{
        beat::Beat
    },
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct VirusBeatSystem;

impl<'s> System<'s> for VirusBeatSystem {
    type SystemData = (
        Read<'s, Beat>,
        ReadStorage<'s, VirusTag>,
        WriteStorage<'s, CompositeTransform>
    );

    fn run(&mut self, (beat, viruses, mut transforms): Self::SystemData) {

        for (virus, transform) in (&viruses, &mut transforms).join() {
            if beat.is_sync_with_beat(0.1) {
                transform.set_scale(Vec2 {
                    x: 1.5,
                    y: 1.5
                });
            }
            else {
                transform.set_scale(Vec2 {
                    x: 1.0,
                    y: 1.0
                });
            }
        }
    }
}
