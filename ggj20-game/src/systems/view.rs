#![allow(clippy::type_complexity)]

use crate::{components::MainCameraTag, resources::wave::Wave};
use oxygengine::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ViewSystem {
    entities: HashSet<Entity>,
}

impl<'s> System<'s> for ViewSystem {
    type SystemData = (
        Read<'s, Wave>,
        WriteStorage<'s, CompositeTransform>,
        ReadStorage<'s, MainCameraTag>,
    );

    fn run(&mut self, (wave, mut transforms, main_cameras): Self::SystemData) {
        let entities = wave
            .airplane_letters
            .values()
            .filter_map(|e| *e)
            .collect::<HashSet<_>>();
        if self.entities.union(&entities).count() == 0 {
            return;
        }

        self.entities = entities;

        let points = self
            .entities
            .iter()
            .filter_map(|e| transforms.get(*e))
            .map(|t| t.get_translation())
            .collect::<Vec<_>>();

        if let Some(bbox) = Rect::bounding(&points) {
            let bbox = bbox.expand(64.0);
            let center = bbox.center();

            for (_, transform) in (&main_cameras, &mut transforms).join() {
                transform.set_translation(center);
                transform.set_scale((bbox.w.max(bbox.h) * 0.5).into());
            }
        }
    }
}
