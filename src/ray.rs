use super::point::Point3;
use super::vector::Vector3;

pub struct TRay<T> {
    origin: TPoint3<T>,
    direction: TVector3<T>
}

impl<T> TRay<T> {
    pub fn new(origin: TPoint3<T>, direction: TVector3<T>) -> TRay<T> {
        TRay{origin, direction}
    }
}