use num::Float;
use std::cmp::Ordering;

pub trait FloatP: Float {
    fn value(x: f64) -> Self;

    fn total_cmp(&self, other: &Self) -> Ordering;
}

impl FloatP for f32 {
    fn value(x: f64) -> Self {
        x as f32
    }

    fn total_cmp(&self, other: &Self) -> Ordering {
        f32::total_cmp(self, other)
    }
}

impl FloatP for f64 {
    fn value(x: f64) -> Self {
        x
    }

    fn total_cmp(&self, other: &Self) -> Ordering {
        f64::total_cmp(self, other)
    }
}
