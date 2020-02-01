use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TiledMapObject {
    pub id: usize,
    #[serde(default)]
    pub name: String,
    #[serde(alias = "type")]
    #[serde(default)]
    pub object_type: String,
    #[serde(default)]
    pub visible: bool,
    pub x: f32,
    pub y: f32,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledMapLayer {
    pub id: usize,
    pub name: String,
    pub visible: bool,
    pub objects: Option<Vec<TiledMapObject>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TiledMap {
    pub layers: Vec<TiledMapLayer>,
}
