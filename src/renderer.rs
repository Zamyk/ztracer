use super::scene::BvhScene;
use crate::WIDTH;
use crate::camera::Camera;
use crate::color::{ColorRgb, ColorSrgb};
use crate::point::TPoint2;
use rand;
use rayon::prelude::*;

type Point2 = TPoint2<f64>;

pub struct Renderer {
    camera: Camera,
    width: i32,
    height: i32,
    scene: BvhScene,
    multithreaded: bool,
}

pub struct IterativeRenderer {
    renderer: Renderer,
    buffer: Vec<ColorRgb>,
    total_iterations: usize,
}

impl Renderer {
    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn render(&self, buffer: &mut [ColorRgb], samples_per_pixel: i32) {
        if self.multithreaded {
            self.render_multithreaded(buffer, samples_per_pixel);
        } else {
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

        let ll = Point2 { x, y };
        let ur = Point2 {
            x: x + pixel_size,
            y: y + pixel_size,
        };

        let mut ans = ColorRgb {
            r: 0.,
            g: 0.,
            b: 0.,
        };

        for _i in 0..samples_per_pixel {
            let p = Self::random_on_square(ll, ur);
            ans += self
                .scene
                .get_ray_color(self.camera.get_ray(p.x, p.y, &mut rand::rng()), 50);
        }
        ans / samples_per_pixel as f64
    }
    fn render_main_thread(
        &self,
        buffer: &mut [ColorRgb],
        start_index: usize,
        samples_per_pixel: i32,
    ) {
        for (index, c) in buffer.iter_mut().enumerate() {
            let x = (index + start_index) % WIDTH;
            let y = (index + start_index) / WIDTH;
            *c += self.get_pixel(x as i32, y as i32, samples_per_pixel);
        }
    }

    fn render_multithreaded(&self, buffer: &mut [ColorRgb], samples_per_pixel: i32) {
        const MULTIPLE_OF: usize = 64;

        let threads = std::thread::available_parallelism().unwrap().get();
        let chunk_size = MULTIPLE_OF * ((buffer.len() - 1) / (threads * MULTIPLE_OF) + 1);

        buffer
            .par_chunks_mut(chunk_size)
            .enumerate()
            .for_each(|(index, chunk)| {
                self.render_main_thread(chunk, index * chunk_size, samples_per_pixel)
            });
    }
}

impl IterativeRenderer {
    pub fn new(
        camera: Camera,
        width: i32,
        height: i32,
        scene: BvhScene,
        multithreaded: bool,
    ) -> Self {
        IterativeRenderer {
            renderer: Renderer {
                camera,
                width,
                height,
                scene,
                multithreaded,
            },
            buffer: vec![ColorRgb::black(); (width * height) as usize],
            total_iterations: 0,
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.renderer.set_camera(camera);
        self.buffer.fill(ColorRgb::black());
        self.total_iterations = 0;
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>, samples_per_pixel: i32) {
        self.total_iterations += 1;
        self.renderer.render(&mut self.buffer, samples_per_pixel);
        buffer.clear();
        buffer.reserve(self.buffer.len());
        buffer.extend(
            self.buffer
                .iter()
                .map(|a| ColorSrgb::from(a.clone() / self.total_iterations as f64).to_u32()),
        );
    }
}
