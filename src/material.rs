use crate::camera::{Ray, Vector3};
use crate::color::ColorRgb;
use crate::hit::THit;
use num::Float;
use rand::Rng;

pub type Hit = THit<f64>;

pub enum Material {
    Metal { albedo: ColorRgb, fuzz: f64 },
    Lambertian { albedo: ColorRgb },
    Dielectric { refractive_index: f64 },
}

impl Material {
    fn scatter_lambertian(hit: &Hit, albedo: &ColorRgb) -> Option<(Ray, ColorRgb)> {
        let next_ray = Ray::new(
            hit.point,
            hit.normal + Vector3::random_on_sphere(&mut rand::rng()),
        );
        if next_ray.direction.near_zero() {
            None
        } else {
            Some((next_ray, *albedo))
        }
    }

    fn scatter_metal(
        ray: &Ray,
        hit: &Hit,
        albedo: &ColorRgb,
        fuzz: f64,
    ) -> Option<(Ray, ColorRgb)> {
        let directed_normal = if hit.normal.dot(&ray.direction) < 0. {
            hit.normal
        } else {
            -hit.normal
        };

        let new_direction = ray.direction.reflect(&directed_normal).normalized()
            + Vector3::random_on_sphere(&mut rand::rng()) * fuzz;
        if new_direction.dot(&directed_normal) > 0.0 {
            Some((Ray::new(hit.point, new_direction), *albedo))
        } else {
            None
        }
    }

    fn reflectance(cos: f64, refractive_index: f64) -> f64 {
        let mut r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }

    fn scatter_dielectric(ray: &Ray, hit: &Hit, refractive_index: f64) -> Option<(Ray, ColorRgb)> {
        let unit_direction = ray.direction.normalized();

        let (directed_ri, directed_normal) = if ray.direction.dot(&hit.normal) < 0.0 {
            (1. / refractive_index, hit.normal)
        } else {
            (refractive_index, -hit.normal)
        };

        let cos = Float::min(-unit_direction.dot(&directed_normal), 1.);
        let sin = (1. - cos * cos).sqrt();

        let direction = if sin * directed_ri <= 1.
            && Self::reflectance(cos, directed_ri) <= rand::rng().random()
        {
            unit_direction.refract(&directed_normal, directed_ri)
        } else {
            unit_direction.reflect(&directed_normal)
        };
        Some((Ray::new(hit.point, direction), ColorRgb::white()))
    }

    pub fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<(Ray, ColorRgb)> {
        match self {
            Material::Lambertian { albedo } => Self::scatter_lambertian(hit, albedo),
            Material::Metal { albedo, fuzz } => Self::scatter_metal(ray, hit, albedo, *fuzz),
            Material::Dielectric { refractive_index } => {
                Self::scatter_dielectric(ray, hit, *refractive_index)
            }
        }
    }
}
