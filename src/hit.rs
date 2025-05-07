use super::vector::{Arithmetic, TVector3};
use super::point::TPoint3;
pub struct THit<T: Arithmetic> {
    pub point: TPoint3<T>,
    pub normal: TVector3<T>,
    pub t: T,
}