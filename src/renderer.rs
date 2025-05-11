use crate::camera::{Camera, Ray, Sphere};
use crate::color::ColorRgb;
use crate::hit::THit;
use rand;
use crate::point::TPoint2;

type Hit = THit<f64>;
type Point2 = TPoint2<f64>;

pub struct Renderer {
    pub camera: Camera,
    pub width: i32,
    pub height: i32,
    pub spheres: Vec<Sphere>,
    pub samples_per_pixel: i32,
}

impl Renderer {
    fn get_ray_color(&self, ray: Ray) -> ColorRgb {
        let mut closest_hit: Option<Hit> = None;

        for sphere in &self.spheres {
            if let Some(hit) = sphere.intersect(&ray, 0.) {
                closest_hit = Some(match closest_hit {
                    None => hit,
                    Some(prev) => if hit.t < prev.t { hit } else { prev },
                });
            }
        }

        match closest_hit {
            Some(hit) => {
                ColorRgb{r: (hit.normal.x + 1.) * 0.5, g: (hit.normal.y + 1.) * 0.5, b: (-hit.normal.z + 1.) * 0.5}
            },
            None => ColorRgb{r: 0.529, g: 0.808, b: 0.922}
        }
    }

    fn random_on_square(lower_left: Point2, upper_right: Point2) -> Point2 {
        let x = lower_left.x + (upper_right.x - lower_left.x) * rand::random::<f64>();
        let y = lower_left.y + (upper_right.y - lower_left.y) * rand::random::<f64>();
        Point2 { x, y }
    }

    pub fn get_pixel(&self, x: i32, y: i32) -> ColorRgb {

        let pixel_size = 2. / std::cmp::max(self.width, self.height) as f64;
        let x = -2. * x as f64 / std::cmp::max(self.width, self.height) as f64 + 1.;
        let y = -2. * y as f64 / std::cmp::max(self.width, self.height) as f64 + 1.;

        let ll = Point2{x, y};
        let ur = Point2{x: x + pixel_size, y: y + pixel_size};

        let mut ans = ColorRgb{r: 0., g: 0., b: 0.};

        for _i in 0..self.samples_per_pixel {
            let p = Self::random_on_square(ll, ur);
            ans += self.get_ray_color(self.camera.get_ray(p.x, p.y));
        }

        ans / self.samples_per_pixel as f64
    }
}