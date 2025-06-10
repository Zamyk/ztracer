#![allow(unused)] // todo!

use crate::flt::FloatP;

#[derive(Copy, Clone)]
pub struct Interval<T> {
    pub min: T,
    pub max: T,
}

impl<T> Interval<T>
where
    T: FloatP,
{
    pub fn containing(vals: &[T]) -> Self {
        let mut min = vals[0];
        let mut max = vals[0];

        for v in vals {
            if *v < min {
                min = *v;
            } else if *v > max {
                max = *v;
            }
        }

        Self { min, max }
    }

    pub fn union(&self, oth: &Self) -> Self {
        Interval {
            min: self.min.min(oth.min),
            max: self.max.max(oth.max),
        }
    }

    pub fn intersection(&self, oth: &Self) -> Self {
        Interval {
            min: self.min.max(oth.min),
            max: self.max.min(oth.max),
        }
    }

    pub fn empty(&self) -> bool {
        self.max <= self.min
    }
}

#[cfg(test)]
mod tests {
    use crate::interval::Interval;

    #[test]
    fn containing_ordered() {
        let v1 = 10.;
        let v2 = 15.;
        let v3 = 20.;

        let interval = Interval::containing(&[v1, v2, v3]);

        assert!(interval.min == v1 && interval.max == v3);
    }

    #[test]
    fn containing_unordered() {
        let v1 = 10.5;
        let v2 = 0.1;
        let v3 = 1231231.1231;

        let interval = Interval::containing(&[v1, v2, v3]);

        assert!(interval.min == v2 && interval.max == v3);
    }

    #[test]
    fn union() {
        let i1 = Interval { min: 10., max: 20. };
        let i2 = Interval { min: 12., max: 22. };

        let i3 = i1.union(&i2);

        assert!(i3.min == i1.min && i3.max == i2.max);
    }

    #[test]
    fn union_disjoint() {
        let i1 = Interval { min: 10., max: 20. };
        let i2 = Interval { min: 0., max: 4. };

        let i3 = i1.union(&i2);

        assert!(i3.min == i2.min && i3.max == i1.max);
    }

    #[test]
    fn intersection() {
        let i1 = Interval { min: 10., max: 20. };
        let i2 = Interval { min: 12., max: 22. };

        let i3 = i1.intersection(&i2);

        assert!(i3.min == i2.min && i3.max == i1.max);
    }
}
