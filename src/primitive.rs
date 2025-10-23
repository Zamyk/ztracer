use super::bbox::BBox;
use super::hit::THit;
use super::ray::Ray;
use crate::flt::FloatP;

pub type MaterialId = usize;
pub trait Primitive<T: FloatP>: Clone {
    fn get_bbox(&self) -> BBox<T>;
    fn intersect(&self, ray: &Ray<T>, min_t: T) -> Option<THit<T>>;
}
