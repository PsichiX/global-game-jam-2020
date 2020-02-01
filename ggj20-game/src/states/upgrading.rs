use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct UpgradingState {
    camera: Option<Entity>,
}

impl State for UpgradingState {
    fn on_enter(&mut self, world: &mut World) {
        
    }

    fn on_process(&mut self, _world: &mut World) -> StateChange {
        StateChange::None
    }
}
