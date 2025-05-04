use std::ops::{Add, Sub, Mul};

use super::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct TPoint3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> std::ops::Sub<TPoint3<T>> for TPoint3<T>
where T: std::ops::Sub<Output = T>
{
    type Output = TVector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        TVector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}


#[cfg(test)]
mod tests {
    use crate::point::TPoint3;

    #[test]
    fn add() {

    }    
}