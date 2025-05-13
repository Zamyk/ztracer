use super::point::TPoint3;
use super::vector::{TVector3, Arithmetic};

#[derive(Copy, Clone, Debug)]
pub struct TRay<T: Arithmetic> {
    pub origin: TPoint3<T>,
    pub direction: TVector3<T>
}

impl <T: Arithmetic> TRay<T> {
    pub fn at(&self, t: T) -> TPoint3<T> {
        self.origin + self.direction * t
    }
}