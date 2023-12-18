use crate::prelude::*;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub const EMPTY: Self = Self {
        min: INFINITY,
        max: -INFINITY,
    };

    pub const UNIVERSE: Self = Self {
        min: -INFINITY,
        max: INFINITY,
    };

    pub fn new() -> Self {
        Self {
            min: -INFINITY,
            max: INFINITY,
        }
    }

    pub fn from_f64(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}
