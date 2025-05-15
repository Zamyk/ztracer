use std::ops::{Add, Sub, Mul, Div, DivAssign, Neg};
use super::flt::FloatP;
use num::Float;
#[derive(Copy, Clone, Debug)]
pub struct TVector3<T: FloatP> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T: FloatP> TVector3<T> {

    pub fn random_on_sphere<T2: rand::Rng>(rng: &mut T2) -> TVector3<T> {
        let theta = rng.random::<f64>() * std::f64::consts::PI;
        let phi = rng.random::<f64>() * 2. * std::f64::consts::PI;
        let x = theta.sin() * phi.cos();
        let y = theta.sin() * phi.sin();
        let z = theta.cos();

        TVector3{x: T::value(x), y: T::value(y), z: T::value(z)}
    }

    pub fn random_on_hemisphere<T2: rand::Rng>(rng: &mut T2, normal: &TVector3<T>) -> TVector3<T> {
        let v = Self::random_on_sphere(rng);
        if normal.dot(&v) < T::zero() {
            return -v;
        }
        v
    }

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

    pub fn reflect(&self, normal: &TVector3<T>) -> TVector3<T> {
        *self - *normal * T::value(2.) * normal.dot(&self)
    }

    pub fn refract(&self, normal: &TVector3<T>, relative_refractive: T) -> TVector3<T> {
        let cos = Float::min(-self.dot(normal), T::one());
        let perpendicular = (*self + *normal * cos) * relative_refractive;
        let parallel = *normal * -(self.dot(&self) - perpendicular.dot(&perpendicular)).sqrt();
        perpendicular + parallel
    }
}

impl<T: FloatP> Add<TVector3<T>> for TVector3<T>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self{x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl<T: FloatP> Sub<TVector3<T>> for TVector3<T>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self{x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl<T: FloatP> Mul<TVector3<T>> for TVector3<T>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self{x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl<T: FloatP> Mul<T> for TVector3<T>
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        Self{x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl<T: FloatP> Div<T> for TVector3<T>
{
    type Output = Self;

    fn div(self, rhs: T) -> Self {
        Self{x: self.x / rhs, y: self.y / rhs, z: self.z / rhs}
    }
}
impl<T: FloatP> DivAssign<T> for TVector3<T>
{
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
        self.z = self.z / rhs;
    }
}

impl<T: FloatP> Neg for TVector3<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Self{x: -self.x, y: -self.y, z: -self.z}
    }
}

#[cfg(test)]
mod tests {
    use super::TVector3;

    #[test]
    fn add() {

    }
}