use std::ops::{Add, Sub};
use super::vector::{TVector3};
use super::flt::FloatP;
#[derive(Copy, Clone, Debug)]
pub struct TPoint3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: FloatP> Sub<TPoint3<T>> for TPoint3<T> {
    type Output = TVector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        TVector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T: FloatP> Sub<&TPoint3<T>> for &TPoint3<T> {
    type Output = TVector3<T>;

    fn sub(self, rhs: &TPoint3<T>) -> Self::Output {
        TVector3{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}
impl<T: FloatP> Add<TVector3<T>> for TPoint3<T> {
    type Output = TPoint3<T>;

    fn add(self, rhs: TVector3<T>) -> Self::Output {
        TPoint3{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}


#[derive(Copy, Clone, Debug)]
pub struct TPoint2<T> {
    pub x: T,
    pub y: T,
}

#[cfg(test)]
mod tests {
    use crate::point::TPoint3;

    #[test]
    fn add() {

    }    
}