#![allow(clippy::type_complexity)]

use crate::{
    components::{ui_element::UiElement, VirusTag},
    resources::beat::Beat,
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct VirusBeatSystem;

impl<'s> System<'s> for VirusBeatSystem {
    type SystemData = (
        Read<'s, Beat>,
        ReadStorage<'s, VirusTag>,
        WriteStorage<'s, UiElement>,
    );

    fn run(&mut self, (beat, viruses, mut ui_elements): Self::SystemData) {
        for (virus, ui_element) in (&viruses, &mut ui_elements).join() {
            if beat.is_sync_with_beat(0.1) {
                ui_element.scale = 1.5.into();
            } else {
                ui_element.scale = 1.0.into();
            }
            ui_element.rebuild();
        }
    }
}
