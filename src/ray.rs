use super::flt::FloatP;
use super::point::TPoint3;
use super::vector::TVector3;

#[derive(Copy, Clone, Debug)]
pub struct TRay<T: FloatP> {
    pub origin: TPoint3<T>,
    pub direction: TVector3<T>,
    pub direction_inverse: TVector3<T>,
}

impl<T: FloatP> TRay<T> {
    pub fn new(origin: TPoint3<T>, direction: TVector3<T>) -> TRay<T> {
        TRay {
            origin,
            direction,
            direction_inverse: TVector3 {
                x: T::one(),
                y: T::one(),
                z: T::one(),
            } / direction,
        }
    }

    pub fn at(&self, t: T) -> TPoint3<T> {
        self.origin + self.direction * t
    }
}
