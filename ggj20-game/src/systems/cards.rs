#![allow(clippy::type_complexity)]

use crate::{
    components::{ui_element::*, CardTag, ManTag},
    resources::{beat::Beat, wave::Wave},
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct CardsSystem;

impl<'s> System<'s> for CardsSystem {
    type SystemData = (
        Read<'s, Beat>,
        Read<'s, Wave>,
        ReadStorage<'s, CardTag>,
        ReadStorage<'s, ManTag>,
        WriteStorage<'s, UiElement>,
        WriteStorage<'s, CompositeVisibility>,
    );

    fn run(
        &mut self,
        (beat, wave, cards, mans, mut ui_elements, mut visibilities): Self::SystemData,
    ) {
        let limit = beat.duration as f32;
        let current = wave.score as f32;
        let level = ((25.0 * current) / limit).max(0.0).min(25.0);

        for (mut ui_element, card) in (&mut ui_elements, &cards).join() {
            ui_element.alignment.x = -(card.0 as f32 - 1.0) + level;
            ui_element.rebuild();
        }

        let lvl = (level as usize).max(0).min(25);
        for (mut visibility, man) in (&mut visibilities, &mans).join() {
            visibility.0 = lvl >= man.0;
        }
    }
}
