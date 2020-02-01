#![allow(clippy::type_complexity)]

use crate::resources::wave::Wave;

use crate::components::{airplane::Airplane, letter::Letter};

use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct AirplaneReturnSystem;

impl<'s> System<'s> for AirplaneReturnSystem {
    type SystemData = (
        Write<'s, Wave>,
        Read<'s, InputController>,
        WriteStorage<'s, Airplane>,
        WriteStorage<'s, Letter>,
        Read<'s, LazyUpdate>,
    );

    fn run(
        &mut self,
        (mut waves, input, mut airplanes, mut letters, lazy_update): Self::SystemData,
    ) {
        let mut airplaines_letters_to_hide = vec![];

        for c in b'a'..=b'z' {
            let letter = c as char;
            let key = format!("key-{}", letter);

            if !input.trigger_or_default(&key[..]).is_pressed() {
                continue;
            }

            for (airplane, letter) in (&mut airplanes, &mut letters)
                .join()
                .filter(|(airplane, _)| !airplane.returning)
            {
                if letter.letter == c {
                    letter.letter = 0;
                    airplane.reverse();

                    // Remove the letter from wave
                    waves.airplane_letters.insert(c, false);

                    if let Some(disp) = airplane.letter_display {
                        airplaines_letters_to_hide.push(disp);
                    }
                }
            }
        }

        if airplaines_letters_to_hide.is_empty() {
            // TODO: Give combo penalty when the player misses a key

            return;
        }

        lazy_update.exec(move |world| {
            for entity in airplaines_letters_to_hide {
                let mut visibility = <CompositeVisibility>::fetch(world, entity);
                visibility.0 = false;
            }
        });
    }
}
