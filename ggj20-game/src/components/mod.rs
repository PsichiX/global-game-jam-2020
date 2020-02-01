use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

pub mod airplane;
pub mod city;
pub mod infection_rate;
pub mod letter;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct MainCameraTag;

impl Component for MainCameraTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for MainCameraTag {}
impl PrefabComponent for MainCameraTag {}
