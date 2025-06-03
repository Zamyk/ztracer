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
mod triangle;
mod obj;

use std::path::Iter;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::random;
use renderer::IterativeRenderer;
use color::ColorRgb;
use material::*;

const WIDTH: usize = 1200;
 const HEIGHT: usize = 765;
//const WIDTH: usize = 400;
//const HEIGHT: usize = 400;

use camera::*;
use scene::Scene;
use crate::triangle::TTriangle;

//taken from Ray Tracing In One Weekend
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

            if (center - Point3{x: 4., y: 0.2, z: 0.}).length() > 0.9 {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = ColorRgb{r: random(), g: random(), b: random()};
                    sphere_material = Material::Lambertian{albedo};
                    materials.push(sphere_material);
                } else if choose_mat < 0.95 {
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
                spheres.push(Sphere{center, radius: 0.2});
            }
        }
    }

    spheres.push(Sphere{center: Point3{x: 0., y: 1., z: 0.}, radius: 1.});
    materials.push(Material::Dielectric {refractive_index: 1.5});

    spheres.push(Sphere{center: Point3{x: -4., y: 1., z: 0.}, radius: 1.});
    materials.push(Material::Lambertian {albedo: ColorRgb{r: 0.4, g: 0.2, b: 0.1}});

    spheres.push(Sphere{center: Point3{x: 4., y: 1., z: 0.}, radius: 1.});
    materials.push(Material::Metal {albedo: ColorRgb{r: 0.7, g: 0.6, b: 0.5}, fuzz: 0.0});

    let spheres_len = spheres.len();
    Scene{spheres, spheres_materials: (0..spheres_len).collect(), triangles: vec![], triangles_materials: vec![], materials}
}
use std::fs::File;
use std::io::BufReader;
use wavefront_obj::obj::Primitive::Triangle;

fn teapot() -> Scene {
    let material = Material::Metal {albedo: ColorRgb{r: 0.8, g: 0.8, b: 0.9}, fuzz: 0.};


    match std::env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }

    let current_dir = std::env::current_dir().unwrap();


    let obj_file_path = current_dir.join("teapot.obj");
    let mut triangles = obj::parse(obj_file_path.to_str().unwrap()).unwrap();
    let triangles_materials = vec![0; triangles.len()];

    let ground_material = Material::Lambertian{albedo: ColorRgb{r: 0.5, g: 0.5, b: 0.5}};
    let ground_center = Point3{x: 0., y: -1000., z: 0.};
    let ground_radius = 1000.;

    let sphere_material = Material::Lambertian{albedo: ColorRgb{r: 0.9, g: 0.5, b: 0.5}};
    let sphere2 = Sphere{center: Point3{x: 3., y: 0.15, z: 0.}, radius: 0.15};
    let scene = Scene{spheres: vec![Sphere{center: ground_center, radius: ground_radius}, sphere2], spheres_materials: vec![1, 2], triangles, triangles_materials, materials: vec![material, ground_material, sphere_material]};

    scene
}

struct RotateCamera {
    phi: f32,
    theta: f32,
    dist: f32
}

impl RotateCamera {
    fn rotate(& mut self, dx: f32, dy: f32) {
        self.phi += dx * 6. / WIDTH as f32;
        self.theta += dy * 3. / HEIGHT as f32;
        self.theta = self.theta.clamp(0., std::f32::consts::PI);
    }

    fn change_dist(& mut self, d: f32) {
        self.dist += d;
        self.dist.clamp(0., 100.);
    }

    fn get_camera(&self) -> Camera {
        let pos = Point3{x: (self.dist * self.phi.cos() * self.theta.cos()) as f64, z: (self.dist * self.phi.sin() * self.theta.cos()) as f64, y: (self.dist * self.theta.sin()) as f64};
        Camera::new(&pos, &Point3{x: 0. , y: 0., z: 0.}, &Vector3{x: 0., y: 1., z: 0.}, 60f64.to_radians())
    }
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

    let camera= Camera::new(&Point3{x: 8. , y: 2.5, z: 3.}, &Point3{x: 0. , y: 0.1, z: 0.}, &Vector3{x: 0., y: 1., z: 0.}, 70f64.to_radians());
    //let camera= Camera::new(&Point3{x: 0. , y: 2., z: 3.}, &Point3{x: 0. , y: 0., z: 0.}, &Vector3{x: 0., y: 1., z: 0.}, 30f64.to_radians());
    //let scene = big_spheres_scene();
    //let scene = teapot();
    let scene = big_spheres_scene();
    let mut renderer = IterativeRenderer::new(camera, WIDTH as i32, HEIGHT as i32, scene, true);

    let mut mouse_x = 0f32;
    let mut mouse_y = 0f32;
    let mut mouse_down = false;
    let mut rot = RotateCamera{phi: 0., theta: 0., dist: 10.};

    rayon::ThreadPoolBuilder::new().num_threads(14).build_global().unwrap();

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
                    rot.rotate(dx, dy);
                    if dx != 0. || dy != 0. {
                        renderer.set_camera(rot.get_camera());
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
            rot.change_dist(ys * -0.3);
            renderer.set_camera(rot.get_camera());
        }

        use std::time::Instant;
        let now = Instant::now();
        {
            renderer.render(& mut buffer, 1);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}