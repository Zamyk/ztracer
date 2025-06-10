use super::point::TPoint3;
use super::vector::TVector3;
use crate::ray::TRay;
use crate::sphere::TSphere;

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
    blur_radius: f64,
    focus_distance: f64,
}

impl Camera {
    pub fn new(position: &Point3, look_at: &Point3, up: &Vector3, fov: f64) -> Self {
        let look_at = (look_at - position).normalized();
        let up = (*up - look_at * up.dot(&look_at)).normalized();
        let right = look_at.cross(&up).normalized();
        let size = (fov * 0.5).tan();
        Camera {
            eye: *position,
            look_at,
            up: up * size,
            right: right * size,
            blur_radius: 0.,
            focus_distance: 1.,
        }
    }

    pub fn new_defocus_blur(
        position: &Point3,
        look_at: &Point3,
        up: &Vector3,
        fov: f64,
        focus_distance: f64,
    ) -> Self {
        let look_at = (look_at - position).normalized();
        let up = (*up - look_at * up.dot(&look_at)).normalized();
        let right = look_at.cross(&up).normalized();
        let size = (fov * 0.5).tan() * focus_distance;
        Camera {
            eye: *position,
            look_at,
            up: up * size,
            right: right * size,
            blur_radius: 0.2,
            focus_distance,
        }
    }
    pub fn get_ray<T2: rand::Rng>(&self, x: f64, y: f64, rng: &mut T2) -> Ray {
        Ray::new(
            self.eye + self.random_in_disk(rng) * self.blur_radius,
            self.look_at * self.focus_distance + self.right * x + self.up * y,
        )
    }

    fn random_in_disk<T1: rand::Rng>(&self, rng: &mut T1) -> Vector3 {
        let radius: f64 = rng.random::<f64>() * self.blur_radius;
        let theta: f64 = rng.random::<f64>() * 2. * std::f64::consts::PI;
        let x = theta.cos() * radius;
        let y = theta.sin() * radius;
        self.right * x + self.up * y
    }
}
