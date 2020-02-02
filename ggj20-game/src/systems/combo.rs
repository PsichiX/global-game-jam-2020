#![allow(clippy::type_complexity)]

use crate::{
    components::{
        ComboProgressTag
    },
    resources::{
        wave::Wave,
        wave::COMBO_STEP
    },
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct ComboSystem;

impl<'s> System<'s> for ComboSystem {
    type SystemData = (
        Read<'s, Wave>,
        ReadStorage<'s, ComboProgressTag>,
        WriteStorage<'s, CompositeRenderable>
    );

    fn run(&mut self, (waves, combo_progresses, mut renderables): Self::SystemData) {
        for (combo_progress, renderable) in (&combo_progresses, &mut renderables).join() {
            if let Renderable::Image(img) = &mut renderable.0 {
                let progress = (waves.combo % COMBO_STEP) as f32 / (COMBO_STEP - 1) as f32;

                img.source = Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 2500.0 * progress,
                    h: 500.0
                });

                img.destination = Some(Rect {
                    x: -300.0,
                    y: 0.0,
                    w: 600.0 * progress,
                    h: 150.0,
                });
            }
        }
    }
}
