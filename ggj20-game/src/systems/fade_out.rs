#![allow(clippy::type_complexity)]

use crate::{
    components::{
        fade_out::FadeOut
    }
};
use oxygengine::prelude::*;

#[derive(Debug, Default)]
pub struct FadeOutSystem;

impl<'s> System<'s> for FadeOutSystem {
    type SystemData = (
        WriteStorage<'s, FadeOut>,
        WriteStorage<'s, CompositeRenderable>,
        ReadExpect<'s, AppLifeCycle>,
    );

    fn run(&mut self, (mut fade_outs, mut renderables, lifecycle): Self::SystemData) {
        for (fade_out, renderable) in (&mut fade_outs, &mut renderables).join() {
            if let Renderable::Commands(commands) = &mut renderable.0 {
                for command in commands {
                    if let Command::Alpha(alpha) = command {
                        *alpha = fade_out.time / fade_out.max_time;
                    }
                }
            }

            fade_out.time = (fade_out.time - lifecycle.delta_time_seconds() as f32).max(0.0);
        }
    }
}
