use crate::{assets::tiled_map_asset_protocol::TiledMapAsset, resources::wave::Wave};
use oxygengine::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct GameState {
    camera: Option<Entity>,
}

impl State for GameState {
    fn on_enter(&mut self, world: &mut World) {
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("scene", world)
            .unwrap();

        world.read_resource::<LazyUpdate>().exec_mut(|world| {
            let assets = &world.read_resource::<AssetsDatabase>();
            let map = assets
                .asset_by_path("tiled://maps/world.json")
                .unwrap()
                .get::<TiledMapAsset>()
                .unwrap()
                .get();
            for layer in &map.layers {
                info!("=== PROCESS LAYER: {} | {}", layer.name, layer.visible);
                if !layer.visible {
                    continue;
                }
                match layer.name.as_str() {
                    "cities" => {
                        let entities_data = layer
                            .objects
                            .as_ref()
                            .unwrap()
                            .iter()
                            .map(|object| {
                                let pos = Vec2::new(object.x, object.y);
                                let parts = object.object_type.split(',').collect::<Vec<_>>();
                                let from = parts[0].parse::<usize>().unwrap();
                                let to = parts[1].parse::<usize>().unwrap();
                                let entity = world
                                    .write_resource::<PrefabManager>()
                                    .instantiate_world("city", world)
                                    .unwrap()[0];
                                (entity, from, to, pos)
                            })
                            .collect::<Vec<_>>();
                        info!("=== COLLECTED ENTITIES DATA: {:#?}", entities_data);
                        let cities_levels = entities_data
                            .iter()
                            .map(|(e, f, t, _)| (*e, (*f, *t)))
                            .collect::<HashMap<_, _>>();
                        info!("=== COLLECTED CITIES LEVELS: {:#?}", cities_levels);

                        world.read_resource::<LazyUpdate>().exec_mut(move |world| {
                            for (entity, _, _, pos) in entities_data {
                                <CompositeTransform>::fetch(world, entity).set_translation(pos);
                            }
                        });

                        info!("=== WRITE CITIES LEVELS");
                        world.write_resource::<Wave>().cities_levels = cities_levels;
                    }
                    _ => {}
                }
            }
        });
    }

    fn on_process(&mut self, _world: &mut World) -> StateChange {
        StateChange::None
    }
}
