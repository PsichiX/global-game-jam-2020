#![allow(clippy::type_complexity)]

use crate::{
    components::{ui_element::*, VirusTag},
    resources::beat::Beat,
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct VirusBeatSystem {
    // NOTE: don't put that here.
    beat: usize,
    anim: i32,
    beat_done: bool
}

impl<'s> System<'s> for VirusBeatSystem {
    type SystemData = (
        Read<'s, Beat>,
        ReadStorage<'s, VirusTag>,
        WriteStorage<'s, UiElement>,
        ReadExpect<'s, AppLifeCycle>,
    );

    fn run(&mut self, (beat, viruses, mut ui_elements, lifecycle): Self::SystemData) {
        for (virus, ui_element) in (&viruses, &mut ui_elements).join() {
            if beat.pulse() {
                // NOTE: don't make that like that
                // self.beat = (self.beat << (self.beat + self.beat * 2)) % 11;
                self.beat = (self.beat + 1) % 22;
                if let UiElementType::Image(image) = &mut ui_element.element_type {
                    image.image =
                        UiImagePath::Single(format!("images/{}_virus.png", self.beat / 2));
                }
                ui_element.rebuild();
            }

            // self.anim -= ((lifecycle.delta_time_seconds() * 1000.0) as i32).max(0);

            if beat.is_sync_with_beat(0.1) && !self.beat_done {
                // self.beat_done = true;

                // if self.anim == 0 {
                //     self.anim = 300;
                // }

                // let factor = if self.anim > 150 { 1.0 - (self.anim as f32 - 150.0) / 150.0 } else { self.anim as f32 / 150.0 };

                // ui_element.scale = (1.0 + factor * 0.5).into();
                ui_element.scale = 1.5.into();
            }

            if !beat.is_sync_with_beat(0.1) {
                // self.beat_done = false;

                ui_element.scale = 1.0.into();
            }

            ui_element.rebuild();
        }
    }
}
