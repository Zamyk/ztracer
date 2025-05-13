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
        let b = ray.direction.dot(&p);
        let c = p.dot(&p) - self.radius * self.radius;

        let discriminant = b * b - a * c;
        if discriminant < T::scalar(0.) {
            None
        }
        else {
            let discriminant_sqrt = discriminant.sqrt();
            let mut root = (-b - discriminant_sqrt) / a;

            if root < min_t {
                root = (-b + discriminant_sqrt) / a;
                if root < min_t {
                    return None;
                }
            }
            let point = ray.origin + ray.direction * root;
            let normal = (point - self.center) / self.radius;
            Some(THit{point, normal, t: root})
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