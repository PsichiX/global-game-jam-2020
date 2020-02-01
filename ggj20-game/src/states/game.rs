use crate::{
    assets::tiled_map_asset_protocol::TiledMapAsset, components::city::City, resources::wave::Wave,
};
use oxygengine::prelude::*;

#[derive(Debug, Clone)]
enum Command {
    None,
    PrepareData,
    InstantiateCities(Vec<(Option<(usize, usize)>, Vec2)>),
    ModifyCities(Vec<(Entity, Option<(usize, usize)>, Vec2)>),
}

impl Default for Command {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Default)]
pub struct GameState {
    camera: Option<Entity>,
    command: Command,
}

impl State for GameState {
    fn on_enter(&mut self, world: &mut World) {
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("scene", world)
            .expect("Could not instantiate scene");

        self.command = Command::PrepareData;
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
        {
            let input = &world.read_resource::<InputController>();
            if input.trigger_or_default("key-a").is_pressed() {
                world.write_resource::<Wave>().current_level += 1;
                info!("=== ADD LEVEL: {}", world.read_resource::<Wave>().current_level);
            } else {
                if input.trigger_or_default("key-d").is_pressed() {
                    let wave = &mut world.write_resource::<Wave>();
                    if wave.current_level > 0 {
                        wave.current_level -= 1;
                    }
                    info!("=== SUB LEVEL: {}", wave.current_level);
                }
            }
        }

        let command = std::mem::replace(&mut self.command, Command::None);
        match command {
            Command::PrepareData => {
                let assets = &world.read_resource::<AssetsDatabase>();
                let map = assets
                    .asset_by_path("tiled://maps/world.json")
                    .expect("Could not get world map asset")
                    .get::<TiledMapAsset>()
                    .expect("World map asset is not a tiled map")
                    .get();
                for layer in &map.layers {
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
                                    (levels_range, pos)
                                })
                                .collect::<Vec<_>>();
                            self.command = Command::InstantiateCities(entities_data);
                        }
                        _ => {}
                    }
                }
            }
            Command::InstantiateCities(entities_data) => {
                let entities_data = entities_data
                    .into_iter()
                    .map(|(levels_range, pos)| {
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
                self.command = Command::ModifyCities(entities_data);
            }
            Command::ModifyCities(entities_data) => {
                for (entity, levels_range, pos) in entities_data {
                    let (mut city, mut transform) =
                        <(City, CompositeTransform)>::fetch(world, entity);
                    city.levels_range = levels_range;
                    transform.set_translation(pos);
                }
            }
            _ => {}
        }
        StateChange::None
    }
}
