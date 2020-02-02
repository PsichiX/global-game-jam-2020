#![allow(clippy::type_complexity)]

use crate::components::{
    airplane::Airplane,
    letter::Letter,
    infection_rate::InfectionRate
};
use crate::resources::{
    wave::Wave,
    beat::Beat
};

use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct AirplaneReturnSystem {
    beat_key_pressed: bool,
    combo_decreased: bool
}

impl<'s> System<'s> for AirplaneReturnSystem {
    type SystemData = (
        Write<'s, Wave>,
        Read<'s, InputController>,
        WriteStorage<'s, Airplane>,
        WriteStorage<'s, Letter>,
        WriteStorage<'s, InfectionRate>,
        Read<'s, LazyUpdate>,
        Read<'s, Beat>,
    );

    fn run(
        &mut self,
        (mut waves, input, mut airplanes, mut letters, infection_rates, lazy_update, beat): Self::SystemData,
    ) {
        if beat.is_sync_with_beat(0.1) && !self.beat_key_pressed {
            self.combo_decreased = false;
            let mut airplaines_letters_to_hide = vec![];

            for c in b'a'..=b'z' {
                let letter = c as char;
                let key = format!("key-{}", letter);

                if !input.trigger_or_default(&key[..]).is_pressed() {
                    continue;
                }

                self.beat_key_pressed = true;

                for (airplane, letter, infection_rate) in (&mut airplanes, &mut letters, &infection_rates)
                    .join()
                    .filter(|(airplane, _, _)| !airplane.returning)
                {
                    if letter.letter == c {
                        letter.letter = 0;
                        airplane.reverse();

                        // Remove the letter from wave
                        waves.airplane_letters.insert(c, false);

                        if let Some(disp) = airplane.letter_display {
                            airplaines_letters_to_hide.push(disp);
                        }

                        if infection_rate.rate == 0 {
                            // TODO: Give combo penalty when the player misses a key
                            info!("bad infection_rate");
                            waves.combo = 0;
                        }
                        else {
                            waves.increase_combo();
                        }

                        break;
                    }
                }
            }

            if !self.beat_key_pressed {
                return;
            }

            if airplaines_letters_to_hide.is_empty() {
                // TODO: Give combo penalty when the player misses a key
                info!("empty airplanes");
                waves.combo = 0;

                return;
            }

            lazy_update.exec(move |world| {
                for entity in airplaines_letters_to_hide {
                    let mut visibility = <CompositeVisibility>::fetch(world, entity);
                    visibility.0 = false;
                }
            });
        }

        if !beat.is_sync_with_beat(0.1) && !self.combo_decreased {
            if !self.beat_key_pressed {
                waves.decrease_combo();
            }

            self.beat_key_pressed = false;
            self.combo_decreased = true;
        }
    }
}
