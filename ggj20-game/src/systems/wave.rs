#![allow(clippy::type_complexity)]

use crate::{
    components::{airplane::Airplane, city::City, infection_rate::InfectionRate, letter::Letter},
    resources::{beat::Beat, wave::Wave},
    utils::tween::*,
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct WaveSystem {
    music_time: f64,
    city: i32,
}

impl<'s> System<'s> for WaveSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, AppLifeCycle>,
        Write<'s, Wave>,
        Write<'s, PrefabManager>,
        ReadStorage<'s, City>,
        ReadStorage<'s, CompositeTransform>,
        Read<'s, Beat>,
    );

    fn run(
        &mut self,
        (entities, lazy_update, lifecycle, mut waves, mut prefabs, cities, transforms, beat): Self::SystemData,
    ) {
        if waves.is_paused {
            return;
        }

        // let sec = lifecycle.delta_time_seconds();

        // if sec > 1.0 {
        //     return;
        // }

        // self.music_time += sec;

        // if self.music_time > waves.airplane_interval {
        //     self.music_time -= waves.airplane_interval;

        if beat.pulse() {
            self.city += 1;

            let cities = (&entities, &cities, &transforms)
                .join()
                .filter(|(_, city, _)| match city.levels_range {
                    Some(r) => waves.current_level >= r.0 && waves.current_level <= r.1,
                    None => false,
                })
                .collect::<Vec<_>>();

            if cities.is_empty() {
                return;
            }

            let city_entity = cities[self.city as usize % cities.len()].0;
            let city_start = cities[self.city as usize % cities.len()].2.clone();
            let city_end = cities[(self.city + 1) as usize % cities.len()].2.clone();

            let mut letter_ascii = None;

            for i in 0..waves.available_letters {
                let letter_index = (i + waves.current_start_letter) % waves.available_letters;
                let c = crate::resources::wave::LETTERS[letter_index];

                if *waves
                    .airplane_letters
                    .get(&c)
                    .expect("Letter was not found")
                {
                    continue;
                }

                letter_ascii = Some(c);
                break;
            }

            let letter_ascii = match letter_ascii {
                Some(c) => c,
                None => {
                    return;
                }
            };

            let letter = letter_ascii as char;

            waves.airplane_letters.insert(letter_ascii, true);

            let airplane_entities = prefabs
                .instantiate_direct(
                    "airplane",
                    &entities,
                    &lazy_update,
                    lifecycle.current_state_token(),
                )
                .unwrap();

            let city_index = self.city;
            let airplane_entity = airplane_entities[0];
            let airplane_letter_entity = airplane_entities[1];

            lazy_update.exec(move |world| {
                {
                    let letter_storage = &mut world.write_storage::<Letter>();
                    let letter_comp = letter_storage
                        .get_mut(airplane_entity)
                        .expect("Cannot get the letter component for airplane");

                    letter_comp.letter = letter_ascii;
                }

                let city_infection_rate = {
                    let city_infection_rate_storage = &world.read_storage::<InfectionRate>();
                    city_infection_rate_storage.get(city_entity).expect("").rate
                };

                {
                    let airplane_storage = &mut world.write_storage::<Airplane>();
                    let mut airplane = airplane_storage.get_mut(airplane_entity).expect("");

                    airplane.start_pos = city_start.get_translation();
                    airplane.end_pos = city_end.get_translation();
                    airplane.phase = 0.0;
                    airplane.tween = Some(Tween::new(TweenType::Cubic, EaseType::InOut));
                    airplane.speed = 0.05;
                    airplane.destination_city = Some(city_entity);
                    airplane.letter_display = Some(airplane_letter_entity);
                }

                let infection_rate = {
                    let infection_rate_storage = &mut world.write_storage::<InfectionRate>();
                    let infection_rate = infection_rate_storage.get_mut(airplane_entity).expect("");

                    if city_index % 7 == 0 {
                        infection_rate.rate = 0;
                    } else {
                        infection_rate.rate = 10.max(city_infection_rate / 100);
                    }

                    infection_rate.rate
                };

                {
                    let composite_renderable_storage =
                        &mut world.write_storage::<CompositeRenderable>();
                    let renderable = composite_renderable_storage
                        .get_mut(airplane_letter_entity)
                        .expect("Cannot get renderable from airplane letter entity");

                    if let Renderable::Image(data) = &mut renderable.0 {
                        data.image = format!("images/letters/{}.png", letter.to_string()).into();
                    }

                    let renderable = composite_renderable_storage
                        .get_mut(airplane_entity)
                        .expect("Cannot get renderable from airplane entity");

                    if let Renderable::Image(data) = &mut renderable.0 {
                        if infection_rate != 0 {
                            data.image = "images/red_airplane.png".into();
                        }
                    }
                }

                {
                    let transform_storage = &mut world.write_storage::<CompositeTransform>();
                    let transform = transform_storage.get_mut(airplane_entity).expect("");

                    transform.set_translation(city_start.get_translation());
                }
            })
        }
    }
}
