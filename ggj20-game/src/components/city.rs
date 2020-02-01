use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct City {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub infection_protection_time: i32,
    /// (from, to)
    #[serde(default)]
    pub levels_range: Option<(usize, usize)>,
}

impl Component for City {
    // not all entities has speed so we use `VecStorage`.
    type Storage = VecStorage<Self>;
}

impl Prefab for City {}
impl PrefabComponent for City {}
