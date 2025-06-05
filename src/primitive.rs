

pub trait Primitive<T : FloatP> {
    fn get_bb() -> BBox<T>;
    fn intersect() -> THit<T>;
}