
use crate::ray::TRay;
use crate::sphere::TSphere;
use super::point::TPoint3;
use super::vector::{TVector3};


pub type Point3 = TPoint3<f64>;
pub type Vector3 = TVector3<f64>;
pub type Ray = TRay<f64>;
pub type Sphere = TSphere<f64>;
#[derive(Copy, Clone, Debug)]
pub struct Camera {
    eye: Point3,
    look_at: Vector3,
    up: Vector3,
    right: Vector3,
    distance: f64,
}

impl Camera {

    pub fn new(position: &Point3, look_at: &Point3, fov: f64) -> Self {
        let look_at = (look_at - position).normalized();
        let up = Vector3{x: 0.0, y: 1.0, z: 0.0} ;
        let up = (up - look_at * up.dot(&look_at)).normalized();
        let right = look_at.cross(&up).normalized();
        let distance = 1. / (fov * 0.5).tan();
        Camera{eye: *position, look_at, up, right, distance}
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray{origin: self.eye, direction: self.look_at * self.distance + self.right * x + self.up * y}
    }
}