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
mod scene;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::random;
use renderer::Renderer;
use color::ColorRgb;
use material::*;

const WIDTH: usize = 600;
const HEIGHT: usize = 600;

use camera::*;
use scene::Scene;
use crate::color::ColorSrgb;

// taken from Ray Tracing In One Weekend
fn big_spheres_scene() -> Scene {
    let ground_material = Material::Lambertian{albedo: ColorRgb{r: 0.5, g: 0.5, b: 0.5}};

    let mut spheres = vec![];
    let mut materials = vec![];

    let ground_center = Point3{x: 0., y: -1000., z: 0.};
    let ground_radius = 1000.;
    spheres.push(Sphere{center: ground_center, radius: ground_radius});
    materials.push(ground_material);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Point3{x: a as f64 + 0.9 * random::<f64>(), y: 0.2, z: b as f64 + 0.9 * random::<f64>()};
            let center = ground_center + (center - ground_center).normalized() * (ground_radius + 0.2);

            if ((center - Point3{x: 4., y: 0.2, z: 0.}).length() > 0.9) {
                let sphere_material;

                if (choose_mat < 0.8) {
                    // diffuse
                    let albedo = ColorRgb{r: random(), g: random(), b: random()};
                    sphere_material = Material::Lambertian{albedo};
                    materials.push(sphere_material);
                } else if (choose_mat < 0.95) {
                    // metal
                    let albedo = ColorRgb{r: random::<f64>() * 0.5 + 0.5, g: random::<f64>() * 0.5 + 0.5, b: random::<f64>() * 0.5 + 0.5};
                    let fuzz = random::<f64>() * 0.5;
                    sphere_material = Material::Metal{albedo, fuzz};
                    materials.push(sphere_material);
                } else {
                    // glass
                    sphere_material = Material::Dielectric{refractive_index: 1.5};
                    materials.push(sphere_material);
                }
                spheres.push(Sphere{center: center, radius: 0.2});
            }
        }
    }




    spheres.push(Sphere{center: Point3{x: 0., y: 1., z: 0.}, radius: 1.});
    materials.push(Material::Dielectric {refractive_index: 1.5});

    spheres.push(Sphere{center: Point3{x: -4., y: 1., z: 0.}, radius: 1.});
    materials.push(Material::Lambertian {albedo: ColorRgb{r: 0.4, g: 0.2, b: 0.1}});

    spheres.push(Sphere{center: Point3{x: 4., y: 1., z: 0.}, radius: 1.});
    materials.push(Material::Metal {albedo: ColorRgb{r: 0.7, g: 0.6, b: 0.5}, fuzz: 0.0});

    Scene{spheres, materials}
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

    //let camera= Camera::new_defocus_blur(&Point3{x: -2. , y: 2., z: 1.}, &Point3{x: 0. , y: 0., z: -1.}, &Vector3{x: 0., y: 1., z: 0.}, 120f64.to_radians(), 0.3);
    let camera= Camera::new(&Point3{x: 13. , y: 2., z: 3.}, &Point3{x: 0. , y: 0., z: 0.}, &Vector3{x: 0., y: 1., z: 0.}, 60f64.to_radians());

    let ground = Sphere{center: Point3{x: 0., y: -100.5, z: -1.}, radius: 100.};
    let ground_material = Material::Lambertian{albedo: ColorRgb{r: 0.8, g: 0.8, b: 0.}};

    let left = Sphere{center: Point3{x: -1., y: 0., z: -1.}, radius: 0.5};
    let left_material = Material::Dielectric{refractive_index: 1.5};
    let left_bubble = Sphere{center: Point3{x: -1., y: 0., z: -1.}, radius: 0.4};
    let left_bubble_material = Material::Dielectric{refractive_index: 1. / 1.5};

    let center = Sphere{center: Point3{x: 0., y: 0., z: -1.2}, radius: 0.5};
    let center_material = Material::Lambertian{albedo: ColorRgb{r: 0.1, g: 0.2, b: 0.5}};

    let right = Sphere{center: Point3{x: 1., y: 0., z: -1.}, radius: 0.5};
    let right_material = Material::Metal{albedo: ColorRgb{r: 0.8, g: 0.6, b: 0.2}, fuzz: 1.};


    let spheres = vec![ground, left, left_bubble, center, right];
    let materials: Vec<Material> = vec![ground_material, left_material, left_bubble_material, center_material, right_material];

    //let scene = Scene{spheres, materials};
    let scene = big_spheres_scene();

    let mut renderer = Renderer::new(camera, WIDTH as i32, HEIGHT as i32, scene, true);

    let mut mouse_x = 0f32;
    let mut mouse_y = 0f32;
    let mut mouse_down = false;

    let mut phi = 0.;
    let mut theta = 60f32.to_radians();
    let mut radius = 5.;

    let mut sum_buffer = vec![ColorRgb::black() ; WIDTH * HEIGHT];

    let mut total_iterations = 0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            if !mouse_down {
                if let Some((xx, yy)) = window.get_mouse_pos(MouseMode::Pass) {
                    mouse_down = true;
                    mouse_x = xx;
                    mouse_y = yy;
                }
            }
            else {
                if let Some((new_mouse_x, new_mouse_y)) = window.get_mouse_pos(MouseMode::Pass) {
                    let dx = new_mouse_x - mouse_x;
                    let dy = new_mouse_y - mouse_y;
                    phi += dx * 6. / WIDTH as f32;
                    theta += dy * 3. / HEIGHT as f32;
                    theta = theta.min(3.14);
                    if dx != 0. || dy != 0. {
                        let pos = Point3{x: (radius * phi.cos() * theta.cos()) as f64, z: (radius * phi.sin() * theta.cos()) as f64, y: (radius * theta.sin()) as f64};
                        let camera= Camera::new(&pos, &Point3{x: 0. , y: 0., z: 0.}, &Vector3{x: 0., y: 1., z: 0.}, 60f64.to_radians());
                        renderer.set_camera(camera);
                        sum_buffer.fill(ColorRgb::black());
                        total_iterations = 0;
                    }
                    mouse_x = new_mouse_x;
                    mouse_y = new_mouse_y;
                }
            }
        }
        else {
            mouse_down = false;
        }

        if let Some((_, ys)) = window.get_scroll_wheel() {
            println!("Scrolling {ys:.2} radius: {radius:.2}");
            radius += ys * -0.3;
            let pos = Point3{x: (radius * phi.cos() * theta.cos()) as f64, z: (radius * phi.sin() * theta.cos()) as f64, y: (radius * theta.sin()) as f64};
            let camera= Camera::new(&pos, &Point3{x: 0. , y: 0., z: 0.}, &Vector3{x: 0., y: 1., z: 0.}, 60f64.to_radians());
            renderer.set_camera(camera);
            sum_buffer.fill(ColorRgb::black());
            total_iterations = 0;
        }

        use std::time::Instant;
        let now = Instant::now();
        {
            renderer.render(& mut sum_buffer, 1);
            total_iterations += 1;
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        buffer.clear();
        buffer.reserve(sum_buffer.len());

        // Use iterators to convert elements from vec_a to vec_b
        buffer.extend(sum_buffer.iter().map(|a| ColorSrgb::from(a.clone() / total_iterations as f64).to_u32()));

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}