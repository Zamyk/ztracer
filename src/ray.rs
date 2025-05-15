use super::point::TPoint3;
use super::vector::{TVector3};
use super::flt::FloatP;

#[derive(Copy, Clone, Debug)]
pub struct TRay<T: FloatP> {
    pub origin: TPoint3<T>,
    pub direction: TVector3<T>
}

impl <T: FloatP> TRay<T> {
    pub fn at(&self, t: T) -> TPoint3<T> {
        self.origin + self.direction * t
    }
}