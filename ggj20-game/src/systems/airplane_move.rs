#![allow(clippy::type_complexity)]

use crate::components::airplane::Airplane;
use crate::utils::tween::*;

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
            if let Some(_tween) = airplane.tween {
                airplane.phase = (airplane.phase + delta * airplane.speed).min(1.0);

                let path = Tween::new(TweenType::Cubic, EaseType::InOut);

                // TODO: Choose one
                // let path_phase = tween.tween(airplane.phase);
                let path_phase = airplane.phase;

                let x = (path_phase) * (airplane.end_pos - airplane.start_pos).x;
                let y = path.tween(path_phase) * (airplane.end_pos - airplane.start_pos).y;

                if airplane.returning {
                    let next_x = (path_phase + 0.01).min(1.0) * (airplane.end_pos - airplane.start_pos).x;
                    let next_y = path.tween(path_phase + 0.01).min(1.0) * (airplane.end_pos - airplane.start_pos).y;
                    let dir = Vec2 { x: next_x, y: next_y } - Vec2 { x, y };

                    transform.set_rotation(dir.y.atan2(dir.x) + std::f32::consts::PI * 0.5);
                }
                else {
                    let next_x = (path_phase + 0.01).min(1.0) * (airplane.end_pos - airplane.start_pos).x;
                    let next_y = path.tween(path_phase + 0.01).min(1.0) * (airplane.end_pos - airplane.start_pos).y;
                    let dir = Vec2 { x: next_x, y: next_y } - Vec2 { x, y };

                    transform.set_rotation(dir.y.atan2(dir.x) + std::f32::consts::PI * 0.5);
                }

                transform.set_translation(
                    airplane.start_pos + Vec2 { x, y }
                        //.lerp(airplane.end_pos, tween.tween(airplane.phase) as f32),
                );
            }
        }
    }
}
