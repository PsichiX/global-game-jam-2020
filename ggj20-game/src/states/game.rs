use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct GameState {
    camera: Option<Entity>,
}

impl State for GameState {
    fn on_enter(&mut self, world: &mut World) {
        // instantiate world objects from scene prefab.
        world
            .write_resource::<PrefabManager>()
            .instantiate_world("scene", world)
            .unwrap();
    }

    fn on_process(&mut self, _world: &mut World) -> StateChange {
        StateChange::None
    }
}
