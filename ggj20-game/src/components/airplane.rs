use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};
use crate::utils::tween::Tween;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Airplane {
    #[serde(default)]
    pub start_pos: Vec2,
    #[serde(default)]
    pub end_pos: Vec2,
    #[serde(default)]
    pub phase: f32,
    #[serde(default)]
    pub tween: Option<Tween>,
    #[serde(default)]
    pub speed: f32
}

impl Component for Airplane {
    // not all entities has speed so we use `VecStorage`.
    type Storage = VecStorage<Self>;
}

impl Prefab for Airplane {}
impl PrefabComponent for Airplane {}
