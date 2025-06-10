use crate::bvh::Bvh;
use crate::camera::{Ray, Sphere};
use crate::color::ColorRgb;
use crate::material::{Hit, Material};
use crate::primitive::MaterialId;
use crate::triangle::TTriangle;

pub struct SceneBuilder {
    materials: Vec<Material>,
    triangles: Vec<TTriangle<f64>>,
    triangles_materials: Vec<MaterialId>,
    spheres: Vec<Sphere>,
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

    pub fn add_triangle(&mut self, triangle: TTriangle<f64>, material: MaterialId) {
        self.triangles.push(triangle);
        self.triangles_materials.push(material);
    }

    pub fn add_sphere(&mut self, sphere: Sphere, material: MaterialId) {
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
    triangles: Bvh<f64, TTriangle<f64>>,
    spheres: Bvh<f64, Sphere>,
}

impl BvhScene {
    pub fn new(
        materials: Vec<Material>,
        triangles: Vec<TTriangle<f64>>,
        triangles_materials: Vec<MaterialId>,
        spheres: Vec<Sphere>,
        spheres_materials: Vec<MaterialId>,
    ) -> Self {
        BvhScene {
            materials,
            triangles: Bvh::build(triangles, triangles_materials),
            spheres: Bvh::build(spheres, spheres_materials),
        }
    }

    pub fn get_ray_color(&self, ray: Ray, iterations: i32) -> ColorRgb {
        if iterations == 0 {
            return ColorRgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }

        let mut closest_hit = None;

        let hit1 = self.triangles.intersect(&ray, 0.001);
        let hit2 = self.spheres.intersect(&ray, 0.001);

        if hit1.is_none() {
            closest_hit = hit2;
        } else if hit2.is_none() {
            closest_hit = hit1;
        } else {
            let h1 = hit1.unwrap();
            let h2 = hit2.unwrap();

            closest_hit = if h1.0.t < h2.0.t { Some(h1) } else { Some(h2) }
        }

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

pub struct Scene {
    pub spheres: Vec<Sphere>,
    pub spheres_materials: Vec<usize>,

    pub triangles: Vec<TTriangle<f64>>,
    pub triangles_materials: Vec<usize>,

    pub materials: Vec<Material>,
}

impl Scene {
    pub fn get_ray_color(&self, ray: Ray, iterations: i32) -> ColorRgb {
        if iterations == 0 {
            return ColorRgb {
                r: 0.0,
                g: 0.0,
                b: 0.0,
            };
        }

        let mut closest_hit: Option<Hit> = None;
        let mut hit_index = 0;

        for (i, sphere) in self.spheres.iter().enumerate() {
            if let Some(hit) = sphere.intersect(&ray, 0.001) {
                closest_hit = Some(match closest_hit {
                    None => {
                        hit_index = i;
                        hit
                    }
                    Some(prev) => {
                        if hit.t < prev.t {
                            hit_index = i;
                            hit
                        } else {
                            prev
                        }
                    }
                });
            }
        }

        if closest_hit.is_some() {
            hit_index = self.spheres_materials[hit_index];
        }

        let mut triangle_hit_index = self.triangles.len();
        for (i, triangle) in self.triangles.iter().enumerate() {
            if let Some(hit) = triangle.intersect(&ray, 0.001) {
                closest_hit = Some(match closest_hit {
                    None => {
                        triangle_hit_index = i;
                        hit
                    }
                    Some(prev) => {
                        if hit.t < prev.t {
                            triangle_hit_index = i;
                            hit
                        } else {
                            prev
                        }
                    }
                });
            }
        }

        if triangle_hit_index != self.triangles.len() {
            hit_index = self.triangles_materials[triangle_hit_index];
        }

        match closest_hit {
            Some(hit) => {
                let scatter = self.materials[hit_index].scatter(&ray, &hit);
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
