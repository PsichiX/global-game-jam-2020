use crate::utils::tween::Tween;
use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

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
    pub speed: f32,
    #[serde(default)]
    pub returning: bool,
}

impl Component for Airplane {
    type Storage = VecStorage<Self>;
}

impl Prefab for Airplane {}
impl PrefabComponent for Airplane {}
