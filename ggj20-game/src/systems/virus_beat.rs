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
}

impl<'s> System<'s> for VirusBeatSystem {
    type SystemData = (
        Read<'s, Beat>,
        ReadStorage<'s, VirusTag>,
        WriteStorage<'s, UiElement>,
    );

    fn run(&mut self, (beat, viruses, mut ui_elements): Self::SystemData) {
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
            if beat.is_sync_with_beat(0.1) {
                ui_element.scale = 1.5.into();
            } else {
                ui_element.scale = 1.0.into();
            }
            ui_element.rebuild();
        }
    }
}
