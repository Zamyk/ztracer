use crate::bbox::BBox;
use crate::flt::FloatP;
use crate::hit::THit;
use crate::point::TPoint3;
use crate::primitive::Primitive;
use crate::ray::TRay;

#[derive(Clone)]

pub struct TTriangle<T> {
    pub v1: TPoint3<T>,
    pub v2: TPoint3<T>,
    pub v3: TPoint3<T>,
}

impl <T: FloatP> TTriangle<T> {
    pub fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>> {
        let e1 = self.v2 - self.v1;
        let e2 = self.v3 - self.v1;
        let normal = e1.cross(&e2);
        let det = -normal.dot(&ray.direction);


        if det.abs() < T::epsilon() {
            return None;
        }

        let det_inv = T::one() / det;
        let s = ray.origin - self.v1;
        let u = s.cross(&-ray.direction);


        let alpha = -e2.dot(&u) * det_inv;
        if alpha < T::zero() || alpha > T::one() {
            return None;
        }

        let beta = e1.dot(&u) * det_inv;
        if beta < T::zero() || alpha + beta > T::one() {
            return None;
        }

        let t = normal.dot(&s) * det_inv;
        if t > min_t {
            Some(THit{point: ray.at(t), normal: normal.normalized(), t})
        }
        else {
            None
        }
    }
}

impl<T: FloatP> Primitive<T> for TTriangle<T> {
    fn get_bbox(&self) -> BBox<T> {
        BBox::containing(&[self.v1, self.v2, self.v3])
    }

    fn intersect(&self, ray: &TRay<T>, min_t: T) -> Option<THit<T>> {
        self.intersect(ray, min_t)
    }
}
#[cfg(test)]
mod tests {
    use rand::{random};
    use crate::camera::{Point3, Ray, Vector3};
    use crate::triangle::TTriangle;

    type Triangle = TTriangle<f64>;

    #[test]
    fn hit_ok() {
        let v1 = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v2 = Point3{x: 1.0, y: 0.0, z: 0.0};
        let v3 = Point3{x: 0.0, y: 1.0, z: 0.0};
        let origin = Point3{x: 0.0, y: 0.0, z: 1.0};
        let direction = Vector3{x: 0.0, y: 0.0, z: -1.0};
        let triangle = Triangle{v1, v2, v3};
        let ray = Ray{origin, direction};

        let hit = triangle.intersect(&ray, 0.);

        assert!(hit.is_some());
        let hit = hit.unwrap();
        assert!(hit.point.x.abs() < f64::EPSILON);
        assert!(hit.point.y.abs() < f64::EPSILON);
        assert!(hit.point.z.abs() < f64::EPSILON);
    }

    #[test]
    fn no_ok_on_parallelogram() {
        let v1 = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v2 = Point3{x: 1.0, y: 0.0, z: 0.0};
        let v3 = Point3{x: 0.0, y: 1.0, z: 0.0};
        let origin = Point3{x: 0.6, y: 0.6, z: 1.0};
        let direction = Vector3{x: 0.0, y: 0.0, z: -1.0};
        let triangle = Triangle{v1, v2, v3};
        let ray = Ray{origin, direction};

        let hit = triangle.intersect(&ray, 0.);

        assert!(hit.is_none());
    }

    #[test]
    fn no_ok_on_parallelogram2() {
        let v1 = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v2 = Point3{x: 1.0, y: 0.0, z: 0.0};
        let v3 = Point3{x: 0.0, y: 1.0, z: 0.0};
        let origin = Point3{x: 0.0, y: 0.0, z: 1.0};
        let direction = Vector3{x: 1.0, y: 1.0, z: -1.0};
        let triangle = Triangle{v1, v2, v3};
        let ray = Ray{origin, direction};

        let hit = triangle.intersect(&ray, 0.);

        assert!(hit.is_none());
    }

    #[test]
    fn random_easy() {
        let v1 = Point3{x: 0.0, y: 0.0, z: 0.0};
        let v2 = Point3{x: 1.0, y: 0.0, z: 0.0};
        let v3 = Point3{x: 0.0, y: 1.0, z: 0.0};

        for i in 0..1000 {
            let alpha: f64 = random();
            let beta: f64 = random::<f64>() * (1. - alpha);
            let p = v1 + (v2 - v1) * alpha + (v3 - v1) * beta;
            let t = 1f64;
            let d = Vector3::random_on_sphere(&mut rand::rng());
            let o = p - d * t;

            if (!Triangle{v1, v2, v3}.intersect(&Ray{origin: o, direction: d}, 0.).is_some()) {
                println!("{:?} -> {:?}", o, d);
            }
            assert!(Triangle{v1, v2, v3}.intersect(&Ray{origin: o, direction: d}, 0.).is_some())
        }
    }

}