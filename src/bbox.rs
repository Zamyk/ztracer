use super::flt::FloatP;
use super::point::TPoint3;
use super::ray::TRay;
pub use crate::interval::Interval;
use crate::vector::TVector3;

#[derive(Clone, Copy)]
pub struct BBox<T> {
    pub bl: TPoint3<T>,
    pub ur: TPoint3<T>,
}

impl<T> BBox<T>
where
    T: FloatP,
{

    pub fn containing(vals: &[TPoint3<T>]) -> Self {
        let mut min = vals[0];
        let mut max = vals[0];

        for v in vals {
            min = min.min(v);
            max = max.max(v);
        }

        Self {bl: min, ur: max}
    }
    pub fn intersect(&self, ray: &TRay<T>, interval: Interval<T>) -> bool {
        let l = (self.bl - ray.origin) / ray.direction;
        let r = (self.ur - ray.origin) / ray.direction;


        !Interval::containing(&[l.x, r.x])
            .intersection(&Interval::containing(&[l.y, r.y]))
            .intersection(&Interval::containing(&[l.z, r.z]))
            .intersection(&interval)
            .empty()
    }

    pub fn union(&self, other: &BBox<T>) -> BBox<T> {
        BBox{bl: self.bl.min(&other.bl), ur: self.ur.max(&other.ur)}
    }

    pub fn get_size(&self) -> TVector3<T> {
        self.ur - self.bl
    }
}

#[cfg(test)]
mod tests {
    use crate::bbox::{BBox, Interval};
    use crate::point::TPoint3;
    use crate::ray::TRay;
    use crate::vector::TVector3;
    use rand::{Rng, SeedableRng};

    #[test]
    fn random_intersection() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        for i in 0..1000 {
            let bl = TPoint3 {
                x: rng.random_range(-10.0..10.0),
                y: rng.random_range(-10.0..10.0),
                z: rng.random_range(-10.0..10.0),
            };
            let ur = TPoint3 {
                x: rng.random_range(bl.x + 1. ..12.0),
                y: rng.random_range(bl.y + 1. ..12.0),
                z: rng.random_range(bl.z + 1. ..12.0),
            };
            let b = BBox { bl, ur };

            // from bl + 0.25 to ur - 0.25
            let o = TPoint3 {
                x: rng.random_range(bl.x + 0.25..ur.x - 0.25),
                y: rng.random_range(bl.y + 0.25..ur.y - 0.25),
                z: rng.random_range(bl.z + 0.25..ur.z - 0.25),
            };
            let d = TVector3::random_on_sphere(&mut rng);
            let ray = TRay {
                origin: o + d * -50.,
                direction: d,
            };

            assert!(b.intersect(&ray, Interval { min: 0., max: 1e9 }));
        }
    }

    #[test]
    fn random_no_intersection() {
        let mut rng = rand::rngs::StdRng::seed_from_u64(0);

        for i in 0..1000 {
            let bl = TPoint3 {
                x: rng.random_range(-10.0..10.0),
                y: rng.random_range(-10.0..10.0),
                z: rng.random_range(-10.0..10.0),
            };
            let ur = TPoint3 {
                x: rng.random_range(bl.x + 1. ..12.0),
                y: rng.random_range(bl.y + 1. ..12.0),
                z: rng.random_range(bl.z + 1. ..12.0),
            };
            let b = BBox { bl, ur };

            // from bl + 0.25 to ur - 0.25
            let o = TPoint3 {
                x: rng.random_range(bl.x + 0.25..ur.x - 0.25),
                y: rng.random_range(bl.y + 0.25..ur.y - 0.25),
                z: rng.random_range(bl.z + 0.25..ur.z - 0.25),
            };


            let d = TVector3::random_on_sphere(&mut rng);
            let ray = TRay {
                origin: o + d * 50.,
                direction: d,
            };

            assert!(!b.intersect(&ray, Interval { min: 0., max: 1e9 }));
        }
    }
}
