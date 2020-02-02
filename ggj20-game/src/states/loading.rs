use crate::states::game::GameState;
use oxygengine::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum LoadingPhase {
    Boot,
    Splash,
    Game,
    Ready,
}

impl Default for LoadingPhase {
    fn default() -> Self {
        Self::Boot
    }
}

#[derive(Default)]
pub struct LoadingState {
    phase: LoadingPhase,
    splash_loading_text: Option<Entity>,
    splash_tap_text: Option<Entity>,
    tries: usize,
}

impl State for LoadingState {
    fn on_process(&mut self, world: &mut World) -> StateChange {
        if world.read_resource::<AssetsDatabase>().is_ready() {
            match self.phase {
                LoadingPhase::Boot => {
                    self.phase = LoadingPhase::Splash;
                    self.tries = 10;

                    let assets = &mut world.write_resource::<AssetsDatabase>();
                    assets
                        .load("set://splash/assets.txt")
                        .expect("Could not start to load splash assets");
                }
                LoadingPhase::Splash => {
                    let instances = world
                        .write_resource::<PrefabManager>()
                        .instantiate_world("splash", world);
                    let instances = if self.tries > 0 {
                        if instances.is_err() {
                            self.tries -= 1;
                            return StateChange::None;
                        } else {
                            instances.expect("Could not instantiate loading scene")
                        }
                    } else {
                        instances.expect("Could not instantiate loading scene")
                    };

                    self.splash_tap_text = instances.get(2).copied();
                    self.splash_loading_text = instances.get(3).copied();
                    self.phase = LoadingPhase::Game;

                    let assets = &mut world.write_resource::<AssetsDatabase>();
                    assets
                        .load("set://assets.txt")
                        .expect("Could not start to load game assets");
                }
                LoadingPhase::Game => {
                    if let Some(splash_loading_text) = self.splash_loading_text {
                        world
                            .write_storage::<CompositeVisibility>()
                            .get_mut(splash_loading_text)
                            .expect("Could not get splash loading text visibility component")
                            .0 = false;
                    }
                    if let Some(splash_tap_text) = self.splash_tap_text {
                        world
                            .write_storage::<CompositeVisibility>()
                            .get_mut(splash_tap_text)
                            .expect("Could not get splash tap text visibility component")
                            .0 = true;
                    }

                    self.phase = LoadingPhase::Ready;
                }
                LoadingPhase::Ready => {
                    let input = &world.read_resource::<InputController>();
                    // NOTE: web browsers require user input to be triggered before playing any audio.
                    if input.trigger_or_default("mouse-left") == TriggerState::Pressed {
                        return StateChange::Swap(Box::new(GameState::new("surf-shimmy".to_owned())));
                    }
                }
            }
        }
        StateChange::None
    }
}
