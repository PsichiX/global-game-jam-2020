use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Letter {
    #[serde(default)]
    pub letter: u8,
}

impl Component for Letter {
    type Storage = VecStorage<Self>;
}

impl Prefab for Letter {}
impl PrefabComponent for Letter {}
