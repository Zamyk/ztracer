use std::ops::{Add, Sub, Mul};

use super::vector::*;

#[derive(Copy, Clone, Debug)]
pub struct TPoint3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> TPoint3<T>
{
    fn new(x: T, y: T, z: T) -> TPoint3<T> {
        TPoint3 { x, y, z }
    }
}


#[cfg(test)]
mod tests {
    use crate::point::TPoint3;

    #[test]
    fn add() {

    }    
}