#![allow(clippy::type_complexity)]

use crate::components::airplane::Airplane;

use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct AirplaneLandSystem;

impl<'s> System<'s> for AirplaneLandSystem {
    type SystemData = (
        ReadStorage<'s, Airplane>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(&mut self, (airplanes, lazy_update, entities): Self::SystemData) {
        let entities_to_delete = (&entities, &airplanes)
            .join()
            .filter(|(_, airplane)| airplane.phase >= 1.0)
            .map(|(entity, _)| entity)
            .collect::<Vec<_>>();

        lazy_update.exec_mut(move |world| {
            for entity in entities_to_delete {
                world.delete_entity(entity).unwrap();
            }
        });
    }
}
