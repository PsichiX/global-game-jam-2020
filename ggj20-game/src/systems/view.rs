#![allow(clippy::type_complexity)]

use crate::{
    components::{city::City, MainCameraTag},
    resources::wave::Wave,
};
use oxygengine::prelude::*;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct ViewSystem {
    entities: HashSet<Entity>,
}

impl<'s> System<'s> for ViewSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Wave>,
        ReadStorage<'s, City>,
        WriteStorage<'s, CompositeTransform>,
        ReadStorage<'s, MainCameraTag>,
    );

    fn run(&mut self, (entities, wave, cities, mut transforms, main_cameras): Self::SystemData) {
        self.entities = (&entities, &cities)
            .join()
            .filter_map(|(entity, city)| {
                if let Some(levels_range) = city.levels_range {
                    if wave.current_level >= levels_range.0 && wave.current_level <= levels_range.1
                    {
                        return Some(entity);
                    }
                }
                None
            })
            .collect::<HashSet<_>>();

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
