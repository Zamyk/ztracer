use std::ops::{Add, Sub, Mul};

use super::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct ColorRgb {
    r: f64,
    g: f64,
    b: f64
}

impl ColorRgb
{
    pub fn new(r: f64, g: f64, b: f64) -> ColorRgb {
        ColorRgb { r, g, b }
    }

    pub fn to_u32(self) -> u32 {
        let mut ans: u32 = 0;
        ans |= ((self.r * 255.) as u32) << 16;
        ans |= ((self.g * 255.) as u32) << 8;
        ans |= ((self.b * 255.) as u32);
        ans
    }
}


#[cfg(test)]
mod tests {
    use crate::point::Point3;

    #[test]
    fn add() {

    }
}