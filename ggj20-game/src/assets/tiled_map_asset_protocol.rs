use crate::utils::tiled::TiledMap;
use oxygengine::prelude::*;
use std::str::from_utf8;

#[derive(Debug, Clone)]
pub struct TiledMapAsset(TiledMap);

impl TiledMapAsset {
    pub fn get(&self) -> &TiledMap {
        &self.0
    }
}

pub struct TiledMapAssetProtocol;

impl AssetProtocol for TiledMapAssetProtocol {
    fn name(&self) -> &str {
        "tiled"
    }

    fn on_load(&mut self, data: Vec<u8>) -> AssetLoadResult {
        let data = from_utf8(&data).unwrap();
        match serde_json::from_str::<TiledMap>(data) {
            Ok(result) => AssetLoadResult::Data(Box::new(TiledMapAsset(result))),
            Err(error) => {
                AssetLoadResult::Error(format!("Error loading tiled map asset: {:?}", error))
            }
        }
    }
}
