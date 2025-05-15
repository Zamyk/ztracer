use super::vector::{TVector3};
use super::point::TPoint3;
use crate::flt::FloatP;

pub struct THit<T: FloatP> {
    pub point: TPoint3<T>,
    pub normal: TVector3<T>,
    pub t: T,
}