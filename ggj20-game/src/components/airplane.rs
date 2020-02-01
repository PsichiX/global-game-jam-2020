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
    #[serde(skip)]
    pub destination_city: Option<Entity>,
    #[serde(skip)]
    pub letter_display: Option<Entity>,
}

impl Component for Airplane {
    type Storage = VecStorage<Self>;
}

impl Airplane {
    pub fn reverse(&mut self) {
        let sp = self.start_pos;
        let ep = self.end_pos;
        self.start_pos = ep;
        self.end_pos = sp;
        self.phase = 1.0 - self.phase;
        self.returning = !self.returning;
    }
}

impl Prefab for Airplane {}
impl PrefabComponent for Airplane {}
