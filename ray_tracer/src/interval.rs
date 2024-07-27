use crate::vec3::Dot;

pub struct Interval {
    pub min: Dot,
    pub max: Dot,
}

impl Interval {
    pub fn new(min: Dot, max: Dot) -> Self {
        Self { min, max }
    }

    pub fn size(&self) -> Dot {
        self.max - self.min
    }

    pub fn contains(&self, x: Dot) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: Dot) -> bool {
        self.min < x && x < self.max
    }
}

pub const INTERVAL_EMPTY: Interval = Interval { min: Dot::INFINITY, max: Dot::NEG_INFINITY };
pub const INTERVAL_UNIVERSE: Interval = Interval { min: Dot::NEG_INFINITY, max: Dot::INFINITY };
