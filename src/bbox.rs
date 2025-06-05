use super::flt::FloatP;
use super::point::TPoint3;

pub struct BBox<T> {
    pub bottom_left: TPoint3<T>,
    pub upper_right: TPoint3<T>,
}

impl<T: FloatP> BBox<T> {
    pub fn from_points(points: &[TPoint3<T>]) -> Self {
        let mut bl = points[0];
        let mut ur = points[0];

        for p in points {
            bl.x = bl.x.min(p.x);
            bl.y = bl.y.min(p.y);
            ur.x = ur.x.max(p.x);
            ur.y = ur.y.max(p.y);
        }
        
        BBox{bottom_left: bl, upper_right: ur}
    }
}