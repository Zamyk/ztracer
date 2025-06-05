use crate::flt::FloatP;
use super::ray::TRay;
use super::bbox::BBox;
use super::hit::THit;
pub trait Primitive<T: FloatP> {
    fn get_bbox(&self) -> BBox<T>;
    fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>>;
}