use crate::{
    assets::tiled_map_asset_protocol::TiledMapAsset, components::city::City, resources::wave::Wave,
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct GameState {
    camera: Option<Entity>,
}

impl State for GameState {
    fn on_enter(&mut self, world: &mut World) {
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("scene", world)
            .expect("Could not instantiate scene");

        world.read_resource::<LazyUpdate>().exec_mut(|world| {
            let assets = &world.read_resource::<AssetsDatabase>();
            let map = assets
                .asset_by_path("tiled://maps/world.json")
                .expect("Could not get world map asset")
                .get::<TiledMapAsset>()
                .expect("World map asset is not a tiled map")
                .get();
            info!("=== TILED: {:#?}", map);
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
                            .expect("Cities layer does not have objects")
                            .iter()
                            .map(|object| {
                                info!("=== OBJECT: {:#?}", object);
                                let pos = Vec2::new(object.x, object.y);
                                let parts = object.object_type.split(',').collect::<Vec<_>>();
                                let levels_range = if parts.len() > 1 {
                                    let from = parts[0]
                                        .parse::<usize>()
                                        .expect("Could not parse city level range: from");
                                    let to = parts[1]
                                        .parse::<usize>()
                                        .expect("Could not parse city level range: to");
                                    Some((from, to))
                                } else {
                                    None
                                };
                                info!("=== pos: {:?} | levels_range: {:?}", pos, levels_range);
                                let entity = world
                                    .write_resource::<PrefabManager>()
                                    .instantiate_world("city", world)
                                    .expect("Could not instantiate city")
                                    .get(0)
                                    .copied()
                                    .expect("City instance has no entities");
                                (entity, levels_range, pos)
                            })
                            .collect::<Vec<_>>();
                        info!("=== COLLECTED ENTITIES DATA: {:#?}", entities_data);

                        world.read_resource::<LazyUpdate>().exec_mut(move |world| {
                            for (entity, levels_range, pos) in entities_data {
                                let (mut city, mut transform) =
                                    <(City, CompositeTransform)>::fetch(world, entity);
                                city.levels_range = levels_range;
                                transform.set_translation(pos);
                            }
                        });
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
