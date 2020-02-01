use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

pub mod airplane;
pub mod city;
pub mod infection_rate;

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct MainCameraTag;

impl Component for MainCameraTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for MainCameraTag {}
impl PrefabComponent for MainCameraTag {}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct MenuTrackSelectedTag;

impl Component for MenuTrackSelectedTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for MenuTrackSelectedTag {}
impl PrefabComponent for MenuTrackSelectedTag {}
