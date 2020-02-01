#![allow(clippy::type_complexity)]

use crate::{
    components::{
        airplane::Airplane, 
        city::City, 
        infection_rate::InfectionRate,
        letter::Letter
    },
    resources::wave::Wave,
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
    );

    fn run(
        &mut self,
        (entities, lazy_update, lifecycle, mut waves, mut prefabs, cities, transforms): Self::SystemData,
    ) {
        if waves.is_paused {
            return;
        }

        let sec = lifecycle.delta_time_seconds();

        if sec > 1.0 {
            return;
        }

        self.music_time += sec;

        if self.music_time > waves.airplane_interval {
            self.music_time -= waves.airplane_interval;

            self.city += 1;

            let cities = (&entities, &cities, &transforms).join().collect::<Vec<_>>();

            if cities.is_empty() {
                return;
            }

            let city_entity = cities[self.city as usize % cities.len()].0;
            let city_start = cities[self.city as usize % cities.len()].2.clone();
            let city_end = cities[(self.city + 1) as usize % cities.len()].2.clone();

            let mut letter_ascii = None;

            for c in crate::resources::wave::LETTERS {
                if *waves.airplane_letters.get(c).expect("Letter was not found") {
                    continue;
                }

                letter_ascii = Some(c);
                break;
            }

            let letter_ascii = match letter_ascii {
                Some(c) => *c,
                None => { return; }
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

            let airplane_entity = airplane_entities[0];
            let airplane_letter_entity = airplane_entities[1];

            lazy_update.exec(move |world| {
                {
                    let composite_renderable_storage = &mut world.write_storage::<CompositeRenderable>();
                    let renderable = composite_renderable_storage.get_mut(airplane_letter_entity).expect("Cannot get renderable from airplane letter entity");

                    if let Renderable::Text(data) = &mut renderable.0 {
                        data.text = letter.to_uppercase().to_string().into();
                    }
                }

                {
                    let letter_storage = &mut world.write_storage::<Letter>();
                    let letter_comp = letter_storage.get_mut(airplane_entity).expect("Cannot get the letter component for airplane");

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
                    airplane.speed = 0.2;
                    airplane.destination_city = Some(city_entity);
                }

                {
                    let infection_rate_storage = &mut world.write_storage::<InfectionRate>(); 
                    let infection_rate = infection_rate_storage.get_mut(airplane_entity).expect("");
                    
                    infection_rate.rate = 10.max(city_infection_rate / 100);
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
