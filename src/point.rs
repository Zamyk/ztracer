use super::flt::FloatP;
use super::vector::TVector3;
use num::traits::real::Real;
use std::ops::{Add, Sub};
#[derive(Copy, Clone, Debug)]
pub struct TPoint3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: FloatP> TPoint3<T> {
    pub fn origin() -> Self {
        TPoint3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
    pub fn min(&self, other: &TPoint3<T>) -> TPoint3<T> {
        TPoint3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(&self, other: &TPoint3<T>) -> TPoint3<T> {
        TPoint3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl<T: FloatP> Sub<TPoint3<T>> for TPoint3<T> {
    type Output = TVector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        TVector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: FloatP> Sub<&TPoint3<T>> for &TPoint3<T> {
    type Output = TVector3<T>;

    fn sub(self, rhs: &TPoint3<T>) -> Self::Output {
        TVector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl<T: FloatP> Add<TVector3<T>> for TPoint3<T> {
    type Output = TPoint3<T>;

    fn add(self, rhs: TVector3<T>) -> Self::Output {
        TPoint3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: FloatP> Sub<TVector3<T>> for TPoint3<T> {
    type Output = TPoint3<T>;

    fn sub(self, rhs: TVector3<T>) -> Self::Output {
        TPoint3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TPoint2<T> {
    pub x: T,
    pub y: T,
}

#[cfg(test)]
mod tests {

    #[test]
    fn add() {}
}
