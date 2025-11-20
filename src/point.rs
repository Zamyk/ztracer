use super::flt::FloatP;
use super::vector::Vector3;
use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Point3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T: FloatP> Point3<T> {
    pub fn origin() -> Self {
        Point3 {
            x: T::zero(),
            y: T::zero(),
            z: T::zero(),
        }
    }
    pub fn min(&self, other: &Point3<T>) -> Point3<T> {
        Point3 {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    pub fn max(&self, other: &Point3<T>) -> Point3<T> {
        Point3 {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }
}

impl<T: FloatP> Sub<Point3<T>> for Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: FloatP> Sub<&Point3<T>> for &Point3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: &Point3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: FloatP> Add<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Point3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: FloatP> Sub<Vector3<T>> for Point3<T> {
    type Output = Point3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Point3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Point2<T> {
    pub x: T,
    pub y: T,
}

#[cfg(test)]
mod tests {

    #[test]
    fn add() {}
}
