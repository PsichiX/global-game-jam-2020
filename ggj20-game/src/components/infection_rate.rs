use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct InfectionRate {
    pub rate: i32
}

impl Component for InfectionRate {
    // not all entities has speed so we use `VecStorage`.
    type Storage = VecStorage<Self>;
}

impl Prefab for InfectionRate {}
impl PrefabComponent for InfectionRate {}
