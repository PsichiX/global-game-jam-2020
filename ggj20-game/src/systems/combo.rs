#![allow(clippy::type_complexity)]

use crate::{
    components::{ComboLeftNumberTag, ComboProgressTag, ComboRightNumberTag},
    resources::{wave::Wave, wave::COMBO_MAX_STEPS, wave::COMBO_STEP},
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct ComboSystem;

impl<'s> System<'s> for ComboSystem {
    type SystemData = (
        Read<'s, Wave>,
        ReadStorage<'s, ComboProgressTag>,
        ReadStorage<'s, ComboLeftNumberTag>,
        ReadStorage<'s, ComboRightNumberTag>,
        WriteStorage<'s, CompositeRenderable>,
    );

    fn run(
        &mut self,
        (waves, combo_progresses, left_combos, right_combos, mut renderables): Self::SystemData,
    ) {
        for (_combo_progress, renderable) in (&combo_progresses, &mut renderables).join() {
            if let Renderable::Image(img) = &mut renderable.0 {
                let mut progress = (waves.combo % COMBO_STEP) as f32 / (COMBO_STEP - 1) as f32;

                if waves.get_combo_multiplier() + 1 > COMBO_MAX_STEPS {
                    progress = 1.0;
                }

                img.source = Some(Rect {
                    x: 0.0,
                    y: 0.0,
                    w: 2500.0 * progress,
                    h: 500.0,
                });

                img.destination = Some(Rect {
                    x: -300.0,
                    y: 0.0,
                    w: 600.0 * progress,
                    h: 150.0,
                });
            }
        }

        for (_left_combo, renderable) in (&left_combos, &mut renderables).join() {
            if let Renderable::Image(img) = &mut renderable.0 {
                img.image = format!("images/texts/{}.png", waves.get_combo_multiplier()).into();
            }
        }

        for (_right_combo, renderable) in (&right_combos, &mut renderables).join() {
            if let Renderable::Image(img) = &mut renderable.0 {
                if waves.get_combo_multiplier() + 1 > COMBO_MAX_STEPS {
                    img.destination = Some(Rect {
                        x: 300.0,
                        y: 0.0,
                        w: 300.0,
                        h: 150.0,
                    });

                    img.image = "images/texts/max.png".into();
                } else {
                    img.destination = Some(Rect {
                        x: 225.0,
                        y: 0.0,
                        w: 150.0,
                        h: 150.0,
                    });

                    img.image =
                        format!("images/texts/{}.png", waves.get_combo_multiplier() + 1).into();
                }
            }
        }
    }
}
