use super::flt::FloatP;
use super::hit::THit;
use super::primitive::{MaterialId, Primitive};
use super::ray::Ray;
use crate::bbox::BBox;
use crate::point::Point3;

#[derive(Copy, Clone)]
struct BvhBox<T: FloatP> {
    bbox: BBox<T>,
    index: usize,
}

pub struct Bvh<T: FloatP, P: Primitive<T>> {
    tree: Vec<BBox<T>>,
    primitives: Vec<P>,
    materials: Vec<MaterialId>,
}

impl<T: FloatP, P: Primitive<T>> Bvh<T, P> {
    fn left(i: usize) -> usize {
        i * 2 + 1
    }

    fn right(i: usize) -> usize {
        i * 2 + 2
    }

    pub fn build(primitives: Vec<P>, materials: Vec<MaterialId>) -> Self {
        if primitives.is_empty() {
            return Bvh {
                tree: vec![],
                primitives: vec![],
                materials: vec![],
            };
        }

        let mut boxes: Vec<_> = primitives
            .iter()
            .enumerate()
            .map(|(i, x)| BvhBox {
                bbox: x.get_bbox(),
                index: i,
            })
            .collect();

        let empty_box = BBox {
            bl: Point3 {
                x: T::zero(),
                y: T::zero(),
                z: T::zero(),
            },
            ur: Point3 {
                x: T::zero(),
                y: T::zero(),
                z: T::zero(),
            },
        };
        let mut ans = Bvh {
            tree: vec![empty_box; primitives.len() * 4 + 1],
            primitives,
            materials: vec![],
        };

        ans.build_rec(0, &mut boxes);


        let mut new_primitives = Vec::with_capacity(ans.primitives.len());
        let mut new_materials = Vec::with_capacity(materials.len());

        for b in boxes {
            new_primitives.push(ans.primitives[b.index].clone());
            new_materials.push(materials[b.index]);
        }
        
        ans.primitives = new_primitives;
        ans.materials = new_materials;

        ans
    }

    fn build_rec(&mut self, i: usize, boxes: &mut [BvhBox<T>]) {
        if boxes.len() == 1 {
            self.tree[i] = boxes[0].bbox;
        } else {
            self.tree[i] = boxes[0].bbox;
            for b in boxes.iter_mut() {
                self.tree[i] = self.tree[i].union(&b.bbox);
            }

            let size = self.tree[i].get_size();
            if size.x > size.y && size.x > size.z {
                boxes.sort_by(|a, b| a.bbox.bl.x.total_cmp(&b.bbox.bl.x));
            } else if size.y > size.z {
                boxes.sort_by(|a, b| a.bbox.bl.y.total_cmp(&b.bbox.bl.y));
            } else {
                boxes.sort_by(|a, b| a.bbox.bl.z.total_cmp(&b.bbox.bl.z));
            }

            let (l, r) = boxes.split_at_mut((boxes.len() + 1) / 2);
            self.build_rec(Self::left(i), l);
            self.build_rec(Self::right(i), r);
        }
    }

    fn intersect_rec(
        &self,
        i: usize,
        l: usize,
        r: usize,
        ray: &Ray<T>,
        min_t: T,
    ) -> Option<(THit<T>, MaterialId)> {
        if l == r {
            Some((self.primitives[l].intersect(ray, min_t)?, self.materials[l]))
        } else {
            let mid = (l + r) / 2;

            let i1 = if self.tree[Self::left(i)].intersect(ray, min_t) {
                self.intersect_rec(Self::left(i), l, mid, ray, min_t)
            } else {
                None
            };

            let i2 = if self.tree[Self::right(i)].intersect(ray, min_t) {
                self.intersect_rec(Self::right(i), mid + 1, r, ray, min_t)
            } else {
                None
            };

            if i1.is_none() {
                return i2;
            }

            if i2.is_none() {
                return i1;
            }

            let h1 = i1.unwrap();
            let h2 = i2.unwrap();

            if h1.0.t < h2.0.t { Some(h1) } else { Some(h2) }
        }
    }

    pub fn intersect(&self, ray: &Ray<T>, min_t: T) -> Option<(THit<T>, MaterialId)> {
        if self.primitives.is_empty() {
            None
        } else {
            self.intersect_rec(0, 0, self.primitives.len() - 1, ray, min_t)
        }
    }
}
