use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct FadeOut {
    pub time: f32,
    pub max_time: f32,
}

impl Component for FadeOut {
    type Storage = VecStorage<Self>;
}

impl Prefab for FadeOut {}
impl PrefabComponent for FadeOut {}
