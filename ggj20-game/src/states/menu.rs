use crate::states::game::GameState;
use oxygengine::prelude::*;

#[derive(Default)]
pub struct MenuState;

impl State for MenuState {
    fn on_process(&mut self, world: &mut World) -> StateChange {
        StateChange::None
    }
}
