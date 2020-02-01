#![allow(clippy::type_complexity)]

use crate::components::airplane::Airplane;

use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct AirplaneMoveSystem;

impl<'s> System<'s> for AirplaneMoveSystem {
    type SystemData = (
        WriteStorage<'s, Airplane>,
        WriteStorage<'s, CompositeTransform>,
        ReadExpect<'s, AppLifeCycle>,
    );

    fn run(&mut self, (mut airplanes, mut transforms, lifecycle): Self::SystemData) {
        let delta = lifecycle.delta_time_seconds() as f32;

        for (mut airplane, transform) in (&mut airplanes, &mut transforms).join() {
            if let Some(tween) = airplane.tween {
                airplane.phase = (airplane.phase + delta * airplane.speed).min(1.0);

                transform.set_translation(
                    airplane
                        .start_pos
                        .lerp(airplane.end_pos, tween.tween(airplane.phase) as f32),
                );
            }
        }
    }
}
