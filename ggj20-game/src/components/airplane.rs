use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Airplane {
    #[serde(default)]
    pub start_pos: Vec2,
    #[serde(default)]
    pub end_pos: Vec2,
    #[serde(default)]
    pub phase: f32,
}

impl Component for Airplane {
    // not all entities has speed so we use `VecStorage`.
    type Storage = VecStorage<Self>;
}

impl Prefab for Airplane {}
impl PrefabComponent for Airplane {}
