use crate::camera::{Ray, Vector3};
use crate::color::ColorRgb;
use crate::hit::THit;
use num::Float;

pub type Hit = THit<f64>;

pub enum Material {
    Metal { albedo: ColorRgb, fuzz: f64 },
    Lambertian { albedo: ColorRgb },
    Dielectric { refractive_index: f64 },
}

impl Material {
    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, ColorRgb)> {
        match self {
            Material::Lambertian { albedo } => {
                let next_ray = Ray {
                    origin: hit.point,
                    direction: hit.normal + Vector3::random_on_sphere(&mut rand::rng()),
                };
                if next_ray.direction.near_zero() {
                    None
                }
                else {
                    Some((next_ray, *albedo))
                }
            }
            Material::Metal { albedo, fuzz } => {
                let new_direction = ray.direction.reflect(&hit.normal).normalized()
                    + Vector3::random_on_sphere(&mut rand::rng()) * *fuzz;
                if new_direction.dot(&hit.normal) > 0.0 {
                    Some((
                        Ray {
                            origin: hit.point,
                            direction: new_direction,
                        },
                        *albedo,
                    ))
                } else {
                    None
                }
            }
            Material::Dielectric { refractive_index } => {
                let unit_direction = ray.direction.normalized();


                let (directed_ri, directed_normal) = if ray.direction.dot(&hit.normal) < 0.0 {
                    (1. / *refractive_index, hit.normal)
                } else {
                    (*refractive_index, -hit.normal)
                };

                let cos = unit_direction.dot(&directed_normal);
                let sin = Float::max(1. - cos * cos, 0.).sqrt();

                let direction = if sin * directed_ri <= 1. {
                    unit_direction.refract(&directed_normal, directed_ri)
                }
                else {
                    unit_direction.reflect(&directed_normal)
                };
                Some((
                    Ray {
                        origin: hit.point,
                        direction,
                    },
                    ColorRgb::white(),
                ))
            }
        }
    }
}
