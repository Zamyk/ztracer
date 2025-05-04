mod point;
mod color;
mod vector;
mod ray;
mod sphere;

use color::ColorRgb;
use minifb::{Key, Window, WindowOptions};


const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

use point::TPoint3;
use vector::TVector3;
use ray::TRay;

type Point3 = TPoint3<f64>;
type Vector3 = TVector3<f64>;
type Ray = TRay<f64>;

fn get_ray_color(ray: Ray) -> ColorRgb {
    ColorRgb{r: 0.529, g: 0.808, b: 0.922}
}

fn get_ray(x: f64, y: f64, eye: Point3) -> Ray {
    Ray{origin: eye, direction: Point3{x, y, z: 0f64} - eye}
}

fn get_pixel(x: f64, y: f64, eye: Point3) -> ColorRgb {
    get_ray_color(get_ray(x, y, eye))
}


fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (index, c) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = index / WIDTH;
            //*c = x as u32 ^ y as u32;
            //*c = ColorRgb::new(0., 0., 1.).to_u32();
            *c = get_pixel(x as f64, y as f64, Point3{x: WIDTH as f64 / 2. , y: HEIGHT as f64 / 2., z: - 10.}).to_u32();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}