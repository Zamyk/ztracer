use std::ops::{AddAssign, Div, Mul, Add};

#[derive(Copy, Clone, Debug)]
pub struct ColorRgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ColorRgb {
    pub fn white() -> ColorRgb {
        ColorRgb{r: 1.0, g: 1.0, b: 1.0}
    }
    
    pub fn black() -> ColorRgb {
        ColorRgb{r: 0.0, g: 0.0, b: 0.0}
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
impl Mul<f64> for ColorRgb {
    type Output = ColorRgb;
    fn mul(self, rhs: f64) -> Self::Output {
        ColorRgb{r: self.r * rhs, g: self.g * rhs, b: self.b * rhs}
    }
}

impl Add<ColorRgb> for ColorRgb {
    type Output = ColorRgb;
    fn add(self, rhs: ColorRgb) -> Self::Output {
        ColorRgb{r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b}
    }
}

impl Mul<ColorRgb> for ColorRgb {
    type Output = ColorRgb;
    fn mul(self, rhs: ColorRgb) -> Self::Output {
        ColorRgb{r: self.r * rhs.r, g: self.g * rhs.g, b: self.b * rhs.b}
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorSrgb {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ColorSrgb
{
    pub fn to_u32(self) -> u32 {
        let mut ans: u32 = 0;
        ans |= ((self.r * 255.) as u32) << 16;
        ans |= ((self.g * 255.) as u32) << 8;
        ans |= (self.b * 255.) as u32;
        ans
    }
}

impl From<ColorRgb> for ColorSrgb {
    fn from(value: ColorRgb) -> Self {
        ColorSrgb{r: value.r.sqrt(), g: value.g.sqrt(), b: value.b.sqrt()}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn add() {

    }
}