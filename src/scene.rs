use crate::bvh::Bvh;
use crate::color::ColorRgb;
use crate::material::Material;
use crate::primitive::MaterialId;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::triangle::Triangle;

pub struct SceneBuilder {
    materials: Vec<Material>,
    triangles: Vec<Triangle<f64>>,
    triangles_materials: Vec<MaterialId>,
    spheres: Vec<Sphere<f64>>,
    spheres_materials: Vec<MaterialId>,
}

impl SceneBuilder {
    pub fn empty() -> SceneBuilder {
        SceneBuilder {
            materials: vec![],
            triangles: vec![],
            triangles_materials: vec![],
            spheres: vec![],
            spheres_materials: vec![],
        }
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
        )
    }
}

pub struct BvhScene {
    materials: Vec<Material>,
    triangles: Bvh<f64, Triangle<f64>>,
    spheres: Bvh<f64, Sphere<f64>>,
}

impl BvhScene {
    pub fn new(
        materials: Vec<Material>,
        triangles: Vec<Triangle<f64>>,
        triangles_materials: Vec<MaterialId>,
        spheres: Vec<Sphere<f64>>,
        spheres_materials: Vec<MaterialId>,
    ) -> Self {
        BvhScene {
            materials,
            triangles: Bvh::build(triangles, triangles_materials),
            spheres: Bvh::build(spheres, spheres_materials),
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
                let a = 0.5 * (ray.direction.normalized().y + 1.0);
                ColorRgb {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                } * (1.0 - a)
                    + ColorRgb {
                        r: 0.5,
                        g: 0.7,
                        b: 1.0,
                    } * a
            }
        }
    }
}
