use super::point::TPoint3;
use super::ray::TRay;
use super::vector::Arithmetic;
use super::hit::THit;
pub struct TSphere<T> {
    pub center: TPoint3<T>,
    pub radius: T,
}

impl <T: Arithmetic> TSphere<T> {
    pub fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>> {
        let p = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = ray.direction.dot(&p) * T::scalar(2.);
        let c = p.dot(&p) - self.radius * self.radius;

        let discriminant = b * b - T::scalar(4.) * a * c;
        if discriminant < T::scalar(0.) {
            None
        }
        else {
            let t = (-b - discriminant.sqrt()) / (T::scalar(2.) * a);
            if t < min_t {
                return None;
            }
            let point = ray.origin + ray.direction * t;
            let normal = (point - self.center) / self.radius;
            Some(THit{point, normal, t})
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn no_intersection() {

    }

    #[test]
    fn intersection_positive() {

    }

    #[test]
    fn intersection_negative() {

    }
}