use crate::Point3;
use super::point::TPoint3;
use super::vector::TVector3;

struct Sphere<T> {
    center: TPoint3<T>,
    radius: T,
}

