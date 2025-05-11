use std::ops::{AddAssign, Div};

#[derive(Copy, Clone, Debug)]
pub struct ColorRgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ColorRgb
{
    pub fn to_u32(self) -> u32 {
        let mut ans: u32 = 0;
        ans |= ((self.r * 255.) as u32) << 16;
        ans |= ((self.g * 255.) as u32) << 8;
        ans |= (self.b * 255.) as u32;
        ans
    }
}

impl AddAssign<ColorRgb> for ColorRgb {
    fn add_assign(&mut self, rhs: ColorRgb) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Div<f64> for ColorRgb {
    type Output = ColorRgb;
    fn div(self, rhs: f64) -> Self::Output {
        ColorRgb{r: self.r / rhs, g: self.g / rhs, b: self.b / rhs}
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn add() {

    }
}