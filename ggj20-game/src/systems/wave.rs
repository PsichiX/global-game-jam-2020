#![allow(clippy::type_complexity)]

use crate::{
    components::{city::City, airplane::Airplane},
    resources::{wave::Wave},
    utils::{tween::*}
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
        Read<'s, Wave>,
        Write<'s, PrefabManager>,
        ReadStorage<'s, City>,
        ReadStorage<'s, CompositeTransform>,
    );

    fn run(&mut self, (entities, lazy_update, lifecycle, waves, mut prefabs, cities, transforms): Self::SystemData) {
        if waves.is_paused {
            return;
        }

        let sec =  lifecycle.delta_time_seconds();

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

            let city_start = cities[self.city as usize % cities.len()].2.clone();
            let city_end = cities[(self.city + 1) as usize % cities.len()].2.clone();

            let airplane_entity = prefabs
                .instantiate_direct(
                    "airplane",
                    &entities,
                    &lazy_update,
                    lifecycle.current_state_token(),
                )
                .unwrap()[0];

            lazy_update.exec(move |world| {
                let (mut airplane, mut transform) =
                    <(Airplane, CompositeTransform)>::fetch(world, airplane_entity);
                airplane.start_pos = city_start.get_translation();
                airplane.end_pos = city_end.get_translation();
                airplane.phase = 0.0;
                airplane.tween = Some(Tween::new(TweenType::Linear, EaseType::Out));
                airplane.speed = 0.2;
                transform.set_translation(city_start.get_translation());
            })
        }
    }
}
