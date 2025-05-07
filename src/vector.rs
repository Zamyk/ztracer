use std::ops::{Add, Sub, Mul, Div, DivAssign, Neg};
use std::cmp::PartialOrd;
pub trait Arithmetic: 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> +
    Div<Output = Self> +
    DivAssign + 
    Copy + 
    PartialOrd +
    Neg<Output = Self> +
    Sized
{
    fn sqrt(self) -> Self;

    fn scalar(value: f64) -> Self;
}

impl Arithmetic for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn scalar(value: f64) -> Self {
        value as f32
    }
}
impl Arithmetic for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn scalar(value: f64) -> Self {
        value
    }
}

#[derive(Copy, Clone, Debug)]
pub struct TVector3<T: Arithmetic> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: Arithmetic> TVector3<T> {
    pub fn normalized(&self) -> TVector3<T> {
        let mut tmp = *self;
        tmp.normalize();
        tmp
    }

    pub fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        *self /= self.length();
    }

    pub fn dot(&self, oth: &TVector3<T>) -> T {
        self.x * oth.x + self.y * oth.y + self.z * oth.z
    }

    pub fn cross(&self, oth: &TVector3<T>) -> TVector3<T> {
        Self{x: self.y * oth.z - self.z * oth.y, y: self.z * oth.x - self.x * oth.z, z: self.x * oth.y - self.y * oth.x}
    }
}

impl<T: Arithmetic> Add<TVector3<T>> for TVector3<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<T: Arithmetic> Sub<TVector3<T>> for TVector3<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T: Arithmetic> Mul<TVector3<T>> for TVector3<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl<T: Arithmetic> Mul<T> for TVector3<T>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl<T: Arithmetic> Div<T> for TVector3<T>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}
impl<T: Arithmetic> DivAssign<T> for TVector3<T>
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