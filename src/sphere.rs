use super::flt::FloatP;
use super::hit::THit;
use super::point::TPoint3;
use super::primitive::Primitive;
use super::ray::TRay;
use super::vector::TVector3;
use crate::bbox::BBox;

#[derive(Clone)]
pub struct TSphere<T> {
    pub center: TPoint3<T>,
    pub radius: T,
}

impl<T: FloatP> TSphere<T> {
    pub fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>> {
        let p = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&p);
        let c = p.dot(&p) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < T::zero() {
            None
        } else {
            let discriminant_sqrt = discriminant.sqrt();
            let mut root = (-b - discriminant_sqrt) / a;

            if root < min_t {
                root = (-b + discriminant_sqrt) / a;
                if root < min_t {
                    return None;
                }
            }
            let point = ray.at(root);
            let normal = (point - self.center) / self.radius;
            Some(THit {
                point,
                normal,
                t: root,
            })
        }
    }
}

impl<T: FloatP> Primitive<T> for TSphere<T> {
    fn get_bbox(&self) -> BBox<T> {
        BBox {
            bl: self.center
                - TVector3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
            ur: self.center
                + TVector3 {
                    x: self.radius,
                    y: self.radius,
                    z: self.radius,
                },
        }
    }

    fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>> {
        self.intersect(ray, min_t)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn no_intersection() {}

    #[test]
    fn intersection_positive() {}

    #[test]
    fn intersection_negative() {}
}
