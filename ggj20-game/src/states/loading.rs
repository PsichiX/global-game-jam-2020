use crate::states::game::GameState;
use oxygengine::prelude::*;

#[derive(Default)]
pub struct LoadingState {
    // preloader: Option<AssetPackPreloader>,
}

impl State for LoadingState {
    fn on_enter(&mut self, world: &mut World) {
        let assets = &mut world.write_resource::<AssetsDatabase>();
        assets.load("set://assets.txt");
    }

    fn on_process(&mut self, world: &mut World) -> StateChange {
        if world.read_resource::<AssetsDatabase>().is_ready() {
            let input = &world.read_resource::<InputController>();
            // NOTE: web browsers require user input to be triggered before playing any audio.
            if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
                return StateChange::Swap(Box::new(GameState::new("werq".to_owned())));
            }
        }
        // let assets = &mut world.write_resource::<AssetsDatabase>();
        // if let Some(preloader) = &mut self.preloader {
        //     if preloader.process(assets).unwrap() {
        //         let input = &world.read_resource::<InputController>();
        //         // NOTE: web browsers require user input to be triggered before playing any audio.
        //         if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
        //             return StateChange::Swap(Box::new(GameState::new("werq".to_owned())));
        //         }
        //     }
        // } else {
        //     self.preloader = Some(
        //         AssetPackPreloader::new("assets.pack", assets, vec!["set://assets.txt"])
        //             .expect("could not create asset pack preloader"),
        //     );
        // }
        StateChange::None
    }
}
