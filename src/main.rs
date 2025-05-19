mod point;
mod color;
mod vector;
mod ray;
mod sphere;
mod camera;
mod hit;
mod renderer;
mod material;
mod flt;

use minifb::{Key, Window, WindowOptions};
use renderer::Renderer;
use color::ColorRgb;
use material::*;

const WIDTH: usize = 1200;
const HEIGHT: usize = 1200;

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

    let ground = Sphere{center: Point3{x: 0., y: -100.5, z: -1.}, radius: 100.};
    let ground_material = Material::Lambertian{albedo: ColorRgb{r: 0.8, g: 0.8, b: 0.}};

    let left = Sphere{center: Point3{x: -1., y: 0., z: -1.}, radius: 0.5};
    let left_material = Material::Dielectric{refractive_index: 1. / 1.33};
    //let left_material = Material::Dielectric{refractive_index: 1.5};

    let center = Sphere{center: Point3{x: 0., y: 0., z: -1.2}, radius: 0.5};
    let center_material = Material::Lambertian{albedo: ColorRgb{r: 0.1, g: 0.2, b: 0.5}};

    let right = Sphere{center: Point3{x: 1., y: 0., z: -1.}, radius: 0.5};
    let right_material = Material::Metal{albedo: ColorRgb{r: 0.8, g: 0.6, b: 0.2}, fuzz: 1.};


    let spheres = vec![ground, left, center, right];
    let materials: Vec<Material> = vec![ground_material, left_material, center_material, right_material];
    
    let renderer = Renderer{camera, width: WIDTH as i32, height: HEIGHT as i32, spheres, samples_per_pixel: 100, materials};



    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (index, c) in buffer.iter_mut().enumerate() {
            let x = index % WIDTH;
            let y = index / WIDTH;
            *c = renderer.get_pixel(x as i32, y as i32).to_u32();
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}