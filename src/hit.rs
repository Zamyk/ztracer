use super::point::Point3;
use super::vector::Vector3;
use crate::flt::FloatP;

pub struct THit<T: FloatP> {
    pub point: Point3<T>,
    pub normal: Vector3<T>,
    pub t: T,
}