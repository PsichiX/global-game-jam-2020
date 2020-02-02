#![allow(clippy::type_complexity)]

use crate::{
    components::{ui_element::*, CardTag},
    resources::wave::Wave,
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct CardsSystem;

impl<'s> System<'s> for CardsSystem {
    type SystemData = (
        Read<'s, Wave>,
        ReadStorage<'s, CardTag>,
        WriteStorage<'s, UiElement>,
    );

    fn run(&mut self, (wave, cards, mut ui_elements): Self::SystemData) {}
}
