use super::flt::FloatP;
use super::point::Point3;
use super::ray::Ray;
use crate::vector::Vector3;

#[derive(Clone, Copy)]
pub struct BBox<T> {
    pub bl: Point3<T>,
    pub ur: Point3<T>,
}

impl<T> BBox<T>
where
    T: FloatP,
{
    pub fn containing(vals: &[Point3<T>]) -> Self {
        let mut min = vals[0];
        let mut max = vals[0];

        for v in vals {
            min = min.min(v);
            max = max.max(v);
        }

        // so this eps is required to handle axis aligned triangle
        // maybe there is better workaround
        let eps = T::value(1e-9);
        let eps = Vector3 {
            x: eps,
            y: eps,
            z: eps,
        };
        Self {
            bl: min - eps,
            ur: max + eps,
        }
    }

    pub fn intersect(&self, ray: &Ray<T>, min_t: T) -> bool {
        let l = (self.bl - ray.origin) * ray.direction_inverse;
        let r = (self.ur - ray.origin) * ray.direction_inverse;

        let mut l1 = l.x;
        let mut r1 = r.x;
        if l1 > r1 {
            (l1, r1) = (r1, l1);
        }

        let mut l2 = l.y;
        let mut r2 = r.y;
        if l2 > r2 {
            (l2, r2) = (r2, l2);
        }

        let mut l3 = l.z;
        let mut r3 = r.z;
        if l3 > r3 {
            (l3, r3) = (r3, l3);
        }

        let min = l1.max(l2).max(l3).max(min_t);
        let max = r1.min(r2).min(r3);

        min <= max
        // !Interval::containing(&[l.x, r.x])
        //     .intersection(&Interval::containing(&[l.y, r.y]))
        //     .intersection(&Interval::containing(&[l.z, r.z]))
        //     .intersection(&Interval{min: min_t, max: T::infinity()})
        //     .empty()
    }

    pub fn union(&self, other: &BBox<T>) -> BBox<T> {
        BBox {
            bl: self.bl.min(&other.bl),
            ur: self.ur.max(&other.ur),
        }
    }

    pub fn get_size(&self) -> Vector3<T> {
        self.ur - self.bl
    }
}

#[cfg(test)]
mod tests {
    use crate::bbox::BBox;
    use crate::point::Point3;
    use crate::ray::Ray;
    use crate::vector::Vector3;
    use rand::{Rng, SeedableRng};

    #[test]
    fn random_intersection() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        for _i in 0..1000 {
            let bl = Point3 {
                x: rng.random_range(-10.0..10.0),
                y: rng.random_range(-10.0..10.0),
                z: rng.random_range(-10.0..10.0),
            };
            let ur = Point3 {
                x: rng.random_range(bl.x + 1. ..12.0),
                y: rng.random_range(bl.y + 1. ..12.0),
                z: rng.random_range(bl.z + 1. ..12.0),
            };
            let b = BBox { bl, ur };

            // from bl + 0.25 to ur - 0.25
            let o = Point3 {
                x: rng.random_range(bl.x + 0.25..ur.x - 0.25),
                y: rng.random_range(bl.y + 0.25..ur.y - 0.25),
                z: rng.random_range(bl.z + 0.25..ur.z - 0.25),
            };
            let d = Vector3::random_on_sphere(&mut rng);
            let ray = Ray::<f64>::new(o + d * -50., d);

            assert!(b.intersect(&ray, 0.));
        }
    }

    #[test]
    fn random_no_intersection() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        for _i in 0..1000 {
            let bl = Point3 {
                x: rng.random_range(-10.0..10.0),
                y: rng.random_range(-10.0..10.0),
                z: rng.random_range(-10.0..10.0),
            };
            let ur = Point3 {
                x: rng.random_range(bl.x + 1. ..12.0),
                y: rng.random_range(bl.y + 1. ..12.0),
                z: rng.random_range(bl.z + 1. ..12.0),
            };
            let b = BBox { bl, ur };

            // from bl + 0.25 to ur - 0.25
            let o = Point3 {
                x: rng.random_range(bl.x + 0.25..ur.x - 0.25),
                y: rng.random_range(bl.y + 0.25..ur.y - 0.25),
                z: rng.random_range(bl.z + 0.25..ur.z - 0.25),
            };

            let d = Vector3::random_on_sphere(&mut rng);
            let ray = Ray::<f64>::new(o + d * 50., d);

            assert!(!b.intersect(&ray, 0.));
        }
    }
}
