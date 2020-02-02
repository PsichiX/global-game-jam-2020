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

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct MenuTrackSelectedTag;

impl Component for MenuTrackSelectedTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for MenuTrackSelectedTag {}
impl PrefabComponent for MenuTrackSelectedTag {}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct VirusTag;

impl Component for VirusTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for VirusTag {}
impl PrefabComponent for VirusTag {}

// Combo
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ComboProgressTag;

impl Component for ComboProgressTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for ComboProgressTag {}
impl PrefabComponent for ComboProgressTag {}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ComboLeftNumberTag;

impl Component for ComboLeftNumberTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for ComboLeftNumberTag {}
impl PrefabComponent for ComboLeftNumberTag {}

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct ComboRightNumberTag;

impl Component for ComboRightNumberTag {
    type Storage = VecStorage<Self>;
}

impl Prefab for ComboRightNumberTag {}
impl PrefabComponent for ComboRightNumberTag {}

