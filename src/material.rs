use crate::color::ColorRgb;
use crate::camera::{Ray, Vector3};
use crate::hit::THit;

pub type Hit = THit<f64>;


pub enum Material {
    Metal{albedo: ColorRgb},
    Lambertian{albedo: ColorRgb},
}

impl Material {

    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, ColorRgb)> {
        match self {
            Material::Lambertian{albedo} => {
                let next_ray = Ray{origin: hit.point, direction: hit.normal + Vector3::random_on_sphere(&mut rand::rng())};
                Some((next_ray, *albedo))
            }
            Material::Metal{albedo} => {
                let next_ray = Ray{origin: hit.point, direction: ray.direction.reflect(&hit.normal)};
                Some((next_ray, *albedo))
            }
        }


    }

}