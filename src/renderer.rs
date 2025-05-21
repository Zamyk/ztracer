use crate::camera::{Camera, Ray, Sphere};
use crate::color::{ColorRgb, ColorSrgb};
use crate::material::*;
use rand;
use crate::point::TPoint2;

type Point2 = TPoint2<f64>;

pub struct Renderer {
    pub camera: Camera,
    pub width: i32,
    pub height: i32,
    pub spheres: Vec<Sphere>,
    pub materials: Vec<Material>,
    pub samples_per_pixel: i32,
}

impl Renderer {
    fn get_ray_color(&self, ray: Ray, iterations: i32) -> ColorRgb {
        if iterations == 0 {
            return ColorRgb{r: 0.0, g: 0.0, b: 0.0};
        }

        let mut closest_hit: Option<Hit> = None;
        let mut hit_index = 0;

        for (i, sphere) in self.spheres.iter().enumerate() {
            if let Some(hit) = sphere.intersect(&ray, 0.001) {
                closest_hit = Some(match closest_hit {
                    None => {hit_index = i; hit},
                    Some(prev) => if hit.t < prev.t { hit_index = i; hit } else { prev },
                });
            }
        }

        match closest_hit {
            Some(hit) => {
                let scatter = self.materials[hit_index].scatter(&ray, &hit);
                match scatter {
                    None => {
                        ColorRgb::black()
                    },
                    Some((next_ray, attenuation)) => {
                        self.get_ray_color(next_ray, iterations - 1) * attenuation
                    }
                }
            },
            None => {
                let a = 0.5 * (ray.direction.normalized().y + 1.0);
                ColorRgb{r: 1., g: 1., b: 1.} * (1.0 - a) + ColorRgb{r: 0.5, g: 0.7, b: 1.0} * a
            }
        }
    }

    fn random_on_square(lower_left: Point2, upper_right: Point2) -> Point2 {
        let x = lower_left.x + (upper_right.x - lower_left.x) * rand::random::<f64>();
        let y = lower_left.y + (upper_right.y - lower_left.y) * rand::random::<f64>();
        Point2 { x, y }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> ColorSrgb {

        let pixel_size = 2. / std::cmp::max(self.width, self.height) as f64;
        let x = 2. * x as f64 / self.width as f64 - 1.;
        let y = -2. * y as f64 / self.height as f64 + 1.;
        let x = x * self.width as f64 / std::cmp::max(self.width, self.height) as f64;
        let y = y * self.height as f64 / std::cmp::max(self.width, self.height) as f64;

        let ll = Point2{x, y};
        let ur = Point2{x: x + pixel_size, y: y + pixel_size};

        let mut ans = ColorRgb{r: 0., g: 0., b: 0.};

        for _i in 0..self.samples_per_pixel {
            let p = Self::random_on_square(ll, ur);
            ans += self.get_ray_color(self.camera.get_ray(p.x, p.y), 20);
        }

        ColorSrgb::from(ans / self.samples_per_pixel as f64)
    }
}