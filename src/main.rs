mod point;
mod color;
mod vector;
mod ray;
mod sphere;
mod camera;
mod hit;

use color::ColorRgb;
use minifb::{Key, Window, WindowOptions};


const WIDTH: usize = 1000;
const HEIGHT: usize = 1000;

use camera::*;

fn get_ray_color(ray: Ray) -> ColorRgb {
    let sphere = Sphere{center: Point3{x: 0., y: 0., z: 5.}, radius: 3.};
    match sphere.intersect(&ray) {
        Some(hit) => {
           ColorRgb{r: (hit.normal.x + 1.) * 0.5, g: (hit.normal.y + 1.) * 0.5, b: (-hit.normal.z + 1.) * 0.5}
            //ColorRgb{r: 0., g: 0., b: (hit.normal.z + 1.) * 0.5}
        },
        None => ColorRgb{r: 0.529, g: 0.808, b: 0.922}
    }
}

fn get_pixel(camera: &Camera, x: f64, y: f64) -> ColorRgb {
    get_ray_color(camera.get_ray(x, y))
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

    let mut camera= Camera::new(&Point3{x: 0. , y: 0., z: -1.}, &Point3{x: 0. , y: 3., z: 10.}, std::f64::consts::PI / 3.);
    let mut z = -1.;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.is_key_down(Key::Down) {
            z -= 0.1;
            camera = Camera::new(&Point3{x: 0. , y: 0., z}, &Point3{x: 0. , y: 0., z: 10.}, std::f64::consts::PI / 4.);
        }
        if window.is_key_down(Key::Up) {
            z += 0.1;
            camera = Camera::new(&Point3{x: 0. , y: 0., z}, &Point3{x: 0. , y: 0., z: 10.}, std::f64::consts::PI / 4.);
        }

        for (index, c) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = index / WIDTH;
            //*c = x as u32 ^ y as u32;
            //*c = ColorRgb::new(0., 0., 1.).to_u32();
            let y = -2. * y as f64 / std::cmp::max(WIDTH, HEIGHT) as f64 + 1.;
            let x = -2. * x as f64 / std::cmp::max(WIDTH, HEIGHT) as f64 + 1.;
            *c = get_pixel(&camera, x, y).to_u32();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}