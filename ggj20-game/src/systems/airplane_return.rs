#![allow(clippy::type_complexity)]

use crate::components::{
    airplane::Airplane, fade_out::FadeOut, infection_rate::InfectionRate, letter::Letter,
    ComboMissTag,
};
use crate::resources::{beat::Beat, wave::Wave};

use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct AirplaneReturnSystem {
    beat_key_pressed: bool,
    combo_decreased: bool,
}

impl<'s> System<'s> for AirplaneReturnSystem {
    type SystemData = (
        Write<'s, Wave>,
        Read<'s, InputController>,
        WriteStorage<'s, Airplane>,
        WriteStorage<'s, Letter>,
        WriteStorage<'s, InfectionRate>,
        WriteStorage<'s, FadeOut>,
        ReadStorage<'s, ComboMissTag>,
        Read<'s, LazyUpdate>,
        Read<'s, Beat>,
    );

    fn run(
        &mut self,
        (
            mut waves,
            input,
            mut airplanes,
            mut letters,
            infection_rates,
            mut fade_outs,
            miss_tags,
            lazy_update,
            beat,
        ): Self::SystemData,
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

                for (airplane, letter, infection_rate) in
                    (&mut airplanes, &mut letters, &infection_rates)
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
                            info!("bad infection_rate");

                            for (miss_tag, fade_out) in (&miss_tags, &mut fade_outs).join() {
                                fade_out.time = fade_out.max_time;
                            }

                            waves.combo = 0;
                        } else {
                            waves.increase_combo();
                        }

                        break;
                    }
                }
            }

            if !self.beat_key_pressed {
                self.beat_key_pressed = false;
                return;
            }

            if airplaines_letters_to_hide.is_empty() {
                // TODO: Give combo penalty when the player misses a key
                info!("empty airplanes");

                for (miss_tag, fade_out) in (&miss_tags, &mut fade_outs).join() {
                    fade_out.time = fade_out.max_time;
                }

                waves.combo = 0;

                self.beat_key_pressed = false;
                return;
            }

            waves.score += waves.get_combo_multiplier();

            lazy_update.exec(move |world| {
                for entity in airplaines_letters_to_hide {
                    let mut visibility = <CompositeVisibility>::fetch(world, entity);
                    visibility.0 = false;
                }
            });
        }

        let mut key_pressed = false;

        for c in b'a'..=b'z' {
            let letter = c as char;
            let key = format!("key-{}", letter);

            if !input.trigger_or_default(&key[..]).is_pressed() {
                continue;
            }

            key_pressed = true;
        }

        if !beat.is_sync_with_beat(0.1) && key_pressed {
            info!("miss the beat");

            for (miss_tag, fade_out) in (&miss_tags, &mut fade_outs).join() {
                fade_out.time = fade_out.max_time;
            }

            waves.combo = 0;

            return;
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
