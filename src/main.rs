mod point;
mod color;
mod vector;
mod ray;
mod sphere;
mod camera;
mod hit;
mod renderer;


use minifb::{Key, Window, WindowOptions};
use renderer::Renderer;

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

use camera::*;

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

    let camera= Camera::new(&Point3{x: 0. , y: 0., z: 0.}, &Point3{x: 0. , y: 0., z: -1.}, 120f64.to_radians());
    let sphere1 = Sphere{center: Point3{x: 0., y: 0., z: -1.}, radius: 0.5};
    let sphere2 = Sphere{center: Point3{x: 0., y: -100.5, z: -1.}, radius: 100.};
    let spheres = vec![sphere1, sphere2];
    let renderer = Renderer{camera, width: WIDTH as i32, height: HEIGHT as i32, spheres, samples_per_pixel: 10};

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (index, c) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = index / WIDTH;
            // let tmp = renderer.get_pixel(x as i32, y as i32);
            *c = renderer.get_pixel(x as i32, y as i32).to_u32();
            //*c = ColorRgb{r: 1., g: 0., b: 0.}.to_u32();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}