
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


#[cfg(test)]
mod tests {
    #[test]
    fn add() {

    }
}