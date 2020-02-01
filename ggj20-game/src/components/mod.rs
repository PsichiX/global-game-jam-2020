use oxygengine::prelude::*;

pub mod airplane;
pub mod city;
pub mod infection_rate;

#[derive(Debug, Default, Copy, Clone)]
pub struct MainCameraTag;

impl Component for MainCameraTag {
    type Storage = VecStorage<Self>;
}
