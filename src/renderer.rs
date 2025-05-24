use crate::camera::{Camera, Ray, Sphere};
use crate::color::{ColorRgb, ColorSrgb};
use rand;
use rand::seq::index::sample;
use crate::point::TPoint2;
use crate::WIDTH;
use super::scene::Scene;
use rayon::prelude::*;

type Point2 = TPoint2<f64>;

pub struct Renderer {
    camera: Camera,
    width: i32,
    height: i32,
    scene: Scene,
    multithreaded: bool
}

impl Renderer {

    pub fn new(camera: Camera, width: i32, height: i32, scene: Scene, multithreaded: bool) -> Self {
        Self { camera, width, height, scene, multithreaded }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn render(&self, buffer: & mut [ColorRgb], samples_per_pixel: i32) {
        if self.multithreaded {
            self.render_multithreaded(buffer, samples_per_pixel);
        }
        else {
            self.render_main_thread(buffer, 0, samples_per_pixel);
        }
    }

    fn random_on_square(lower_left: Point2, upper_right: Point2) -> Point2 {
        let x = lower_left.x + (upper_right.x - lower_left.x) * rand::random::<f64>();
        let y = lower_left.y + (upper_right.y - lower_left.y) * rand::random::<f64>();
        Point2 { x, y }
    }

    fn get_pixel(&self, x: i32, y: i32, samples_per_pixel: i32) -> ColorRgb {
        let pixel_size = 2. / std::cmp::max(self.width, self.height) as f64;
        let x = 2. * x as f64 / self.width as f64 - 1.;
        let y = -2. * y as f64 / self.height as f64 + 1.;
        let x = x * self.width as f64 / std::cmp::max(self.width, self.height) as f64;
        let y = y * self.height as f64 / std::cmp::max(self.width, self.height) as f64;

        let ll = Point2{x, y};
        let ur = Point2{x: x + pixel_size, y: y + pixel_size};

        let mut ans = ColorRgb{r: 0., g: 0., b: 0.};

        for _i in 0..samples_per_pixel {
            let p = Self::random_on_square(ll, ur);
            ans += self.scene.get_ray_color(self.camera.get_ray(p.x, p.y, &mut rand::rng()), 20);
        }
        ans / samples_per_pixel as f64
    }
    fn render_main_thread(&self, buffer: & mut [ColorRgb], start_index: usize, samples_per_pixel: i32) {
        for (index, c) in buffer.iter_mut().enumerate() {
            let x = (index + start_index) % WIDTH;
            let y = (index + start_index) / WIDTH;
            *c += self.get_pixel(x as i32, y as i32, samples_per_pixel);
        }
    }

    fn render_multithreaded(&self, buffer: &mut [ColorRgb], samples_per_pixel: i32) {
        const MULTIPLE_OF: usize = 64;

        let threads = std::thread::available_parallelism().unwrap().get();
        let chunk_size = MULTIPLE_OF * ( (buffer.len() - 1) / (threads * MULTIPLE_OF) + 1 );

        buffer.par_chunks_mut(chunk_size).enumerate().for_each(|(index, chunk)|self.render_main_thread(chunk, index * chunk_size, samples_per_pixel));
    }

}