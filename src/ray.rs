use super::flt::FloatP;
use super::point::Point3;
use super::vector::Vector3;

#[derive(Copy, Clone, Debug)]
pub struct Ray<T: FloatP> {
    pub origin: Point3<T>,
    pub direction: Vector3<T>,
    pub direction_inverse: Vector3<T>,
}

impl<T: FloatP> Ray<T> {
    pub fn new(origin: Point3<T>, direction: Vector3<T>) -> Ray<T> {
        Ray {
            origin,
            direction,
            direction_inverse: Vector3 {
                x: T::one(),
                y: T::one(),
                z: T::one(),
            } / direction,
        }
    }

    pub fn at(&self, t: T) -> Point3<T> {
        self.origin + self.direction * t
    }
}
