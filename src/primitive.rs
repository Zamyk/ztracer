use super::bbox::BBox;
use super::hit::THit;
use super::ray::TRay;
use crate::flt::FloatP;

pub type MaterialId = usize;
pub trait Primitive<T: FloatP>: Clone {
    fn get_bbox(&self) -> BBox<T>;
    fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>>;
}
