use oxygengine::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
pub struct UiMargin {
    #[serde(default)]
    pub top: f32,
    #[serde(default)]
    pub bottom: f32,
    #[serde(default)]
    pub left: f32,
    #[serde(default)]
    pub right: f32,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum UiParentAnchor {
    ConstantLength(f32),
    PercentageLength(f32),
}

impl Default for UiParentAnchor {
    fn default() -> Self {
        Self::ConstantLength(0.0)
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum UiAnchor {
    ConstantOriginLength(f32, f32),
    Parent(UiParentAnchor, UiParentAnchor),
}

impl Default for UiAnchor {
    fn default() -> Self {
        Self::ConstantOriginLength(0.0, 0.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiImagePath {
    Single(String),
    Set([Option<String>; 9]),
}

impl UiImagePath {
    pub fn get(&self, index: usize) -> Option<&str> {
        match self {
            Self::Single(p) => Some(p),
            Self::Set(p) => {
                if let Some(p) = &p[index] {
                    Some(p)
                } else {
                    None
                }
            }
        }
    }
}

impl Default for UiImagePath {
    fn default() -> Self {
        Self::Single(Default::default())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiImage {
    #[serde(default)]
    pub image: UiImagePath,
    #[serde(default)]
    pub source_rect: Rect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UiElementType {
    None,
    Image(UiImage),
}

impl Default for UiElementType {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UiElement {
    #[serde(default)]
    pub camera_tag: String,
    #[serde(default)]
    pub element_type: UiElementType,
    #[serde(default)]
    pub margin: UiMargin,
    #[serde(default)]
    pub horizontal_anchor: UiAnchor,
    #[serde(default)]
    pub vertical_anchor: UiAnchor,
    #[serde(skip)]
    dirty: bool,
}

impl UiElement {
    pub fn rebuild(&mut self) {
        self.dirty = true;
    }
}

impl Component for UiElement {
    type Storage = VecStorage<Self>;
}

impl Prefab for UiElement {}
impl PrefabComponent for UiElement {}

impl UiElement {
    pub fn calculate_rect(&self, parent_rect: Rect) -> Rect {
        match &self.element_type {
            UiElementType::Image(_) => {
                let min_width = self.margin.left + self.margin.right;
                let min_height = self.margin.top + self.margin.bottom;
                let (x, width) = match &self.horizontal_anchor {
                    UiAnchor::ConstantOriginLength(o, l) => (*o, l.max(min_width)),
                    UiAnchor::Parent(from, to) => {
                        let f = match from {
                            UiParentAnchor::ConstantLength(v) => *v,
                            UiParentAnchor::PercentageLength(v) => parent_rect.w * *v,
                        };
                        let t = match to {
                            UiParentAnchor::ConstantLength(v) => parent_rect.w - *v,
                            UiParentAnchor::PercentageLength(v) => parent_rect.w * (1.0 - *v),
                        };
                        (f, (t - f).max(min_width))
                    }
                };
                let (y, height) = match &self.vertical_anchor {
                    UiAnchor::ConstantOriginLength(o, l) => (*o, l.max(min_height)),
                    UiAnchor::Parent(from, to) => {
                        let f = match from {
                            UiParentAnchor::ConstantLength(v) => *v,
                            UiParentAnchor::PercentageLength(v) => parent_rect.h * *v,
                        };
                        let t = match to {
                            UiParentAnchor::ConstantLength(v) => parent_rect.h - *v,
                            UiParentAnchor::PercentageLength(v) => parent_rect.h * (1.0 - *v),
                        };
                        (f, (t - f).max(min_height))
                    }
                };
                Rect {
                    x: x + parent_rect.x,
                    y: y + parent_rect.y,
                    w: width,
                    h: height,
                }
            }
            _ => parent_rect,
        }
    }

    pub fn build_commands(&self, parent_rect: Rect) -> Vec<Command<'static>> {
        let rect = self.calculate_rect(parent_rect);
        match &self.element_type {
            UiElementType::Image(image) => {
                let sx1 = image.source_rect.x;
                let sx2 = image.source_rect.x + self.margin.left;
                let sx3 = image.source_rect.x + image.source_rect.w - self.margin.right;
                let sy1 = image.source_rect.y;
                let sy2 = image.source_rect.y + self.margin.top;
                let sy3 = image.source_rect.y + image.source_rect.h - self.margin.bottom;

                let sw1 = self.margin.left;
                let sw2 = image.source_rect.w - self.margin.left - self.margin.right;
                let sw3 = self.margin.right;
                let sh1 = self.margin.top;
                let sh2 = image.source_rect.h - self.margin.top - self.margin.bottom;
                let sh3 = self.margin.bottom;

                let dx1 = rect.x;
                let dx2 = rect.x + self.margin.left;
                let dx3 = rect.x + rect.w - self.margin.right;
                let dy1 = rect.y;
                let dy2 = rect.y + self.margin.top;
                let dy3 = rect.y + rect.h - self.margin.bottom;

                let dw1 = self.margin.left;
                let dw2 = rect.w - self.margin.left - self.margin.right;
                let dw3 = self.margin.right;
                let dh1 = self.margin.top;
                let dh2 = rect.h - self.margin.top - self.margin.bottom;
                let dh3 = self.margin.bottom;

                vec![
                    if let Some(path) = image.image.get(0) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx1,
                                    y: sy1,
                                    w: sw1,
                                    h: sh1,
                                }),
                                destination: Some(Rect {
                                    x: dx1,
                                    y: dy1,
                                    w: dw1,
                                    h: dh1,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(1) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx2,
                                    y: sy1,
                                    w: sw2,
                                    h: sh1,
                                }),
                                destination: Some(Rect {
                                    x: dx2,
                                    y: dy1,
                                    w: dw2,
                                    h: dh1,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(2) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx3,
                                    y: sy1,
                                    w: sw3,
                                    h: sh1,
                                }),
                                destination: Some(Rect {
                                    x: dx3,
                                    y: dy1,
                                    w: dw3,
                                    h: dh1,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(3) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx1,
                                    y: sy2,
                                    w: sw1,
                                    h: sh2,
                                }),
                                destination: Some(Rect {
                                    x: dx1,
                                    y: dy2,
                                    w: dw1,
                                    h: dh2,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(4) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx2,
                                    y: sy2,
                                    w: sw2,
                                    h: sh2,
                                }),
                                destination: Some(Rect {
                                    x: dx2,
                                    y: dy2,
                                    w: dw2,
                                    h: dh2,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(5) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx3,
                                    y: sy2,
                                    w: sw3,
                                    h: sh2,
                                }),
                                destination: Some(Rect {
                                    x: dx3,
                                    y: dy2,
                                    w: dw3,
                                    h: dh2,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(6) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx1,
                                    y: sy3,
                                    w: sw1,
                                    h: sh3,
                                }),
                                destination: Some(Rect {
                                    x: dx1,
                                    y: dy3,
                                    w: dw1,
                                    h: dh3,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(7) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx2,
                                    y: sy3,
                                    w: sw2,
                                    h: sh3,
                                }),
                                destination: Some(Rect {
                                    x: dx2,
                                    y: dy3,
                                    w: dw2,
                                    h: dh3,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                    if let Some(path) = image.image.get(8) {
                        Command::Draw(
                            Image {
                                image: path.to_owned().into(),
                                source: Some(Rect {
                                    x: sx3,
                                    y: sy3,
                                    w: sw3,
                                    h: sh3,
                                }),
                                destination: Some(Rect {
                                    x: dx3,
                                    y: dy3,
                                    w: dw3,
                                    h: dh3,
                                }),
                                alignment: Vec2::new(0.0, 0.0),
                            }
                            .into(),
                        )
                    } else {
                        Command::None
                    },
                ]
            }
            _ => vec![],
        }
    }
}
