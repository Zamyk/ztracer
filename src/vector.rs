use crate::Vector3;
use super::point::*;

#[derive(Copy, Clone, Debug)]
pub struct TVector3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> TVector3<T>
where T: std::ops::Add<Output = T> + std::ops::Mul<Output = T> + Copy,
TVector3<T>: std::ops::DivAssign<T>
{

    pub fn normalized(&self) -> TVector3<T> {
        let mut tmp = *self;
        tmp.normalize();
        tmp
    }
    pub fn length(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn normalize(&mut self) {
        *self /= self.length();
    }
    pub fn dot(&self, oth: &TVector3<T>) -> T {
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

impl<T> std::ops::DivAssign<T> for TVector3<T>
where T: std::ops::DivAssign<T> + Copy
{

    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::TVector3;

    #[test]
    fn add() {

    }
}