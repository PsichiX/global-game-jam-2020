#![allow(clippy::type_complexity)]

use crate::resources::wave::Wave;

use crate::components::{
    airplane::Airplane, city::City, infection_rate::InfectionRate, letter::Letter,
};

use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct AirplaneLandSystem;

impl<'s> System<'s> for AirplaneLandSystem {
    type SystemData = (
        ReadStorage<'s, Airplane>,
        ReadStorage<'s, Letter>,
        Read<'s, LazyUpdate>,
        Entities<'s>,
    );

    fn run(&mut self, (airplanes, letters, lazy_update, entities): Self::SystemData) {
        let entities_to_delete = (&entities, &airplanes, &letters)
            .join()
            .filter(|(_, airplane, _)| airplane.phase >= 1.0)
            .map(|(entity, airplane, letters)| (entity, airplane.destination_city, letters.letter))
            .collect::<Vec<_>>();

        lazy_update.exec_mut(move |world| {
            for (entity, destination_city, letter) in entities_to_delete {
                if let Some(destination_city) = destination_city {
                    let infection_rate = {
                        let infection_rate_storage = &world.read_storage::<InfectionRate>();
                        infection_rate_storage
                            .get(entity)
                            .expect("Infection rate component was not found in airplane entity")
                            .rate
                    };

                    let city_infection_rate = {
                        let city_infection_rate_storage =
                            &mut world.write_storage::<InfectionRate>();
                        let city_infection_rate_comp = city_infection_rate_storage
                            .get_mut(destination_city)
                            .expect("");

                        city_infection_rate_comp.rate =
                            (city_infection_rate_comp.rate + infection_rate).min(10000);
                        city_infection_rate_comp.rate
                    };

                    let infection_display_entity = {
                        let city_storage = &mut world.read_storage::<City>();
                        city_storage
                            .get(destination_city)
                            .expect("")
                            .infection_display_entity
                    };

                    if let Some(infection_display_entity) = infection_display_entity {
                        let renderable_storage = &mut world.write_storage::<CompositeRenderable>();
                        let mut renderable = renderable_storage
                            .get_mut(infection_display_entity)
                            .expect("");

                        renderable.0 = Renderable::Path(Path {
                            color: Color {
                                r: (((city_infection_rate as f32) / 10000.00) * 255.0) as u8,
                                g: 0,
                                b: 0,
                                a: 255,
                            },
                            elements: vec![PathElement::Rectangle(Rect {
                                x: -50.0,
                                y: -50.0,
                                w: 100.0,
                                h: 100.0,
                            })],
                        });
                    }
                }

                {
                    let waves = &mut world.write_resource::<Wave>();
                    waves.airplane_letters.insert(letter, false);
                }

                world.delete_entity(entity).unwrap();
            }
        });
    }
}
