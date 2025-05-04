use super::point::*;

#[derive(Copy, Clone, Debug)]
pub struct TVector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> TVector3<T>
where T: TVector3::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy
{
    pub fn new(x: T, y: T, z: T) -> TVector3<T> {
        TVector3 { x, y, z }
    }

    pub fn dot(self, oth: &TVector3<T>) -> T {
        self.x * oth.x + self.y * oth.y + self.z * oth.z
    }
}

impl<T> std::ops::Add<TVector3<T>> for TVector3<T>
where T: std::ops::Add<Output = T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<T> std::ops::Sub<TVector3<T>> for TVector3<T>
where T: std::ops::Sub<Output = T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T> std::ops::Mul<TVector3<T>> for TVector3<T>
where T: std::ops::Mul<Output = T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

#[cfg(test)]
mod tests {
    use super::TVector3;

    #[test]
    fn add() {
        let v1 = TVector3::new(1, 2, 3);
        let v2= TVector3::new(4, 5, 6);

        let sum = v1 + v2;

        assert_eq!(sum.x, 5);
        assert_eq!(sum.y, 7);
        assert_eq!(sum.z, 9);
    }
}