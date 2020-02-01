use easer::functions::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum TweenType {
    Back,
    Bounce,
    Circ,
    Cubic,
    Elastic,
    Expo,
    Linear,
    Quad,
    Quart,
    Quint,
    Sine,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum EaseType {
    In,
    Out,
    InOut,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Tween {
    tween_type: TweenType,
    ease: EaseType,
}

impl Tween {
    pub fn new(tween_type: TweenType, ease: EaseType) -> Tween {
        Tween { tween_type, ease }
    }

    pub fn tween(&self, t: f32) -> f32 {
        match self.tween_type {
            TweenType::Back => match self.ease {
                EaseType::In => Back::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Back::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Back::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Bounce => match self.ease {
                EaseType::In => Bounce::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Bounce::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Bounce::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Circ => match self.ease {
                EaseType::In => Circ::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Circ::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Circ::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Cubic => match self.ease {
                EaseType::In => Cubic::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Cubic::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Cubic::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Elastic => match self.ease {
                EaseType::In => Elastic::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Elastic::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Elastic::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Expo => match self.ease {
                EaseType::In => Expo::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Expo::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Expo::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Linear => match self.ease {
                EaseType::In => Linear::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Linear::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Linear::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Quad => match self.ease {
                EaseType::In => Quad::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Quad::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Quad::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Quart => match self.ease {
                EaseType::In => Quart::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Quart::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Quart::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Quint => match self.ease {
                EaseType::In => Quint::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Quint::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Quint::ease_in_out(t, 0.0, 1.0, 1.0),
            },
            TweenType::Sine => match self.ease {
                EaseType::In => Sine::ease_in(t, 0.0, 1.0, 1.0),
                EaseType::Out => Sine::ease_out(t, 0.0, 1.0, 1.0),
                EaseType::InOut => Sine::ease_in_out(t, 0.0, 1.0, 1.0),
            },
        }
    }
}
