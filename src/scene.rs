use wavefront_obj::mtl::Color;
use crate::bvh::Bvh;
use crate::color::ColorRgb;
use crate::material::Material;
use crate::point::Point2;
use crate::primitive::MaterialId;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::triangle::Triangle;
use crate::texture::{ColorTexture, Texture2d};
use crate::texture::ImageTexture;

pub struct SceneBuilder {
    materials: Vec<Material>,
    triangles: Vec<Triangle<f64>>,
    triangles_materials: Vec<MaterialId>,
    spheres: Vec<Sphere<f64>>,
    spheres_materials: Vec<MaterialId>,
    skybox: Box<dyn Texture2d<ColorRgb, f64>>
}

impl SceneBuilder {
    pub fn empty() -> SceneBuilder {
        SceneBuilder {
            materials: vec![],
            triangles: vec![],
            triangles_materials: vec![],
            spheres: vec![],
            spheres_materials: vec![],
            skybox: Box::new(ColorTexture{color: ColorRgb::white()})
        }
    }


    pub fn set_skybox<T: Texture2d<ColorRgb, f64> + 'static>(&mut self, skybox: T) {
        self.skybox = Box::new(skybox);
    }

    pub fn add_triangle(&mut self, triangle: Triangle<f64>, material: MaterialId) {
        self.triangles.push(triangle);
        self.triangles_materials.push(material);
    }

    pub fn add_sphere(&mut self, sphere: Sphere<f64>, material: MaterialId) {
        self.spheres.push(sphere);
        self.spheres_materials.push(material);
    }

    pub fn add_material(&mut self, material: Material) -> MaterialId {
        self.materials.push(material);
        self.materials.len() - 1
    }

    pub fn build(self) -> BvhScene {
        BvhScene::new(
            self.materials,
            self.triangles,
            self.triangles_materials,
            self.spheres,
            self.spheres_materials,
            self.skybox
        )
    }
}

pub struct BvhScene {
    materials: Vec<Material>,
    triangles: Bvh<f64, Triangle<f64>>,
    spheres: Bvh<f64, Sphere<f64>>,
    skybox: Box<dyn Texture2d<ColorRgb, f64>>
}

impl BvhScene {
    pub fn new(
        materials: Vec<Material>,
        triangles: Vec<Triangle<f64>>,
        triangles_materials: Vec<MaterialId>,
        spheres: Vec<Sphere<f64>>,
        spheres_materials: Vec<MaterialId>,
        skybox: Box<dyn Texture2d<ColorRgb, f64>>,
    ) -> Self {
        BvhScene {
            materials,
            triangles: Bvh::build(triangles, triangles_materials),
            spheres: Bvh::build(spheres, spheres_materials),
            skybox
        }
    }

    pub fn get_ray_color(&self, ray: Ray<f64>, iterations: i32) -> ColorRgb {
        if iterations == 0 {
            return ColorRgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }

        let hit1 = self.triangles.intersect(&ray, 0.001);
        let hit2 = self.spheres.intersect(&ray, 0.001);

        let closest_hit = if hit1.is_none() {
            hit2
        } else if hit2.is_none() {
            hit1
        } else {
            let h1 = hit1.unwrap();
            let h2 = hit2.unwrap();

            if h1.0.t < h2.0.t { Some(h1) } else { Some(h2) }
        };

        match closest_hit {
            Some((hit, id)) => {
                let scatter = self.materials[id].scatter(&ray, &hit);
                match scatter {
                    None => ColorRgb::black(),
                    Some((next_ray, attenuation)) => {
                        self.get_ray_color(next_ray, iterations - 1) * attenuation
                    }
                }
            }
            None => {
                let u = 0.5 - f64::atan2(ray.direction.z, ray.direction.x) / (2.0 * std::f64::consts::PI);
                let v = f64::acos(ray.direction.normalized().y) / std::f64::consts::PI;
                self.skybox.get(&Point2::<f64>{x: u, y: v})
            }
        }
    }
}
