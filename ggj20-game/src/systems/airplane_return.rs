#![allow(clippy::type_complexity)]

use crate::components::{airplane::Airplane, MainCameraTag};

use oxygengine::prelude::*;

const CLICK_RANGE: f32 = 100.0;

#[derive(Debug, Default)]
pub struct AirplaneReturnSystem;

impl<'s> System<'s> for AirplaneReturnSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputController>,
        Read<'s, CompositeCameraCache>,
        WriteStorage<'s, Airplane>,
        ReadStorage<'s, MainCameraTag>,
        ReadStorage<'s, CompositeTransform>,
    );

    fn run(
        &mut self,
        (entities, input, camera_cache, mut airplanes, main_cameras, transforms): Self::SystemData,
    ) {
        if !input.trigger_or_default("mouse-left").is_pressed() {
            return;
        }
        let x = input.axis_or_default("mouse-x");
        let y = input.axis_or_default("mouse-y");
        let pointer_pos = Vec2::new(x, y);
        let camera_entity =
            if let Some((camera_entity, _)) = (&entities, &main_cameras).join().next() {
                camera_entity
            } else {
                return;
            };
        let range_sqr = CLICK_RANGE * CLICK_RANGE;

        for (mut airplane, transform) in (&mut airplanes, &transforms).join() {
            if airplane.returning || airplane.tween.is_none() {
                continue;
            }

            let world_pos = transform.get_translation();
            if let Some(screen_pos) = camera_cache.world_to_screen_space(camera_entity, world_pos) {
                let distance_sqr = (pointer_pos - screen_pos).sqr_magnitude();
                if distance_sqr <= range_sqr {
                    let sp = airplane.start_pos;
                    let ep = airplane.end_pos;
                    airplane.start_pos = ep;
                    airplane.end_pos = sp;
                    airplane.phase = 1.0 - airplane.phase;
                    airplane.returning = true;
                }
            }
        }
    }
}