mod bbox;
mod bvh;
mod camera;
mod color;
mod flt;
mod hit;
mod interval;
mod material;
mod matrix;
mod obj;
mod point;
mod primitive;
mod ray;
mod renderer;
mod scene;
mod sphere;
mod triangle;
mod vector;

use color::ColorRgb;
use material::*;
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::random;
use renderer::IterativeRenderer;

const WIDTH: usize = 1200;
const HEIGHT: usize = 765;

use crate::bvh::Bvh;
use crate::scene::{BvhScene, SceneBuilder};
use camera::*;
use scene::Scene;

fn dragon_scene() -> (BvhScene, Point3) {
    match std::env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }
    let current_dir = std::env::current_dir().unwrap();
    let obj_file_path = current_dir.join("dragon.obj");
    let triangles = obj::parse(obj_file_path.to_str().unwrap()).unwrap();

    let mut builder = SceneBuilder::empty();
    let dragon_material1 = builder.add_material(Material::Dielectric {
        refractive_index: 1.5,
    });
    for t in &triangles {
        builder.add_triangle(t.clone(), dragon_material1);
    }

    let dragon_material2 = builder.add_material(Material::Metal {
        albedo: ColorRgb {
            r: 0.698,
            g: 0.569,
            b: 0.275,
        },
        fuzz: 0.1,
    });

    for t in &triangles {
        let mut nt = t.clone();
        nt.v1.z -= 10.;
        nt.v2.z -= 10.;
        nt.v3.z -= 10.;

        builder.add_triangle(nt, dragon_material2);
    }

    let dragon_material3 = builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.3,
            g: 0.8,
            b: 0.3,
        },
    });
    for t in &triangles {
        let mut nt = t.clone();
        nt.v1.z += 10.;
        nt.v2.z += 10.;
        nt.v3.z += 10.;
        builder.add_triangle(nt, dragon_material3);
    }

    let ground_material = builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        },
    });
    let ground_center = Point3 {
        x: 0.,
        y: -10000.,
        z: 0.,
    };
    let ground_radius = 10000.;

    builder.add_sphere(
        Sphere {
            center: ground_center,
            radius: ground_radius,
        },
        ground_material,
    );

    (
        builder.build(),
        Point3 {
            x: 0.,
            y: 3.,
            z: 0.,
        },
    )
}

// taken from Ray Tracing In One Weekend
fn big_spheres_scene() -> (BvhScene, Point3) {
    let ground_center = Point3 {
        x: 0.,
        y: -1000.,
        z: 0.,
    };
    let ground_radius = 1000.;

    let mut scene_builder = SceneBuilder::empty();

    let ground_material = scene_builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        },
    });
    scene_builder.add_sphere(
        Sphere {
            center: ground_center,
            radius: ground_radius,
        },
        ground_material,
    );

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let center = Point3 {
                x: a as f64 + 0.9 * random::<f64>(),
                y: 0.2,
                z: b as f64 + 0.9 * random::<f64>(),
            };
            let center =
                ground_center + (center - ground_center).normalized() * (ground_radius + 0.2);

            if (center
                - Point3 {
                    x: 4.,
                    y: 0.2,
                    z: 0.,
                })
            .length()
                > 0.9
            {
                let sphere_material;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = ColorRgb {
                        r: random(),
                        g: random(),
                        b: random(),
                    };
                    sphere_material = scene_builder.add_material(Material::Lambertian { albedo });
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = ColorRgb {
                        r: random::<f64>() * 0.5 + 0.5,
                        g: random::<f64>() * 0.5 + 0.5,
                        b: random::<f64>() * 0.5 + 0.5,
                    };
                    let fuzz = random::<f64>() * 0.5;
                    sphere_material = scene_builder.add_material(Material::Metal { albedo, fuzz });
                } else {
                    // glass
                    sphere_material = scene_builder.add_material(Material::Dielectric {
                        refractive_index: 1.5,
                    });
                }
                scene_builder.add_sphere(
                    Sphere {
                        center,
                        radius: 0.2,
                    },
                    sphere_material,
                );
            }
        }
    }

    let m1 = scene_builder.add_material(Material::Dielectric {
        refractive_index: 1.5,
    });
    scene_builder.add_sphere(
        Sphere {
            center: Point3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            radius: 1.,
        },
        m1,
    );

    let m2 = scene_builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.4,
            g: 0.2,
            b: 0.1,
        },
    });
    scene_builder.add_sphere(
        Sphere {
            center: Point3 {
                x: -4.,
                y: 1.,
                z: 0.,
            },
            radius: 1.,
        },
        m2,
    );

    let m3 = scene_builder.add_material(Material::Metal {
        albedo: ColorRgb {
            r: 0.7,
            g: 0.6,
            b: 0.5,
        },
        fuzz: 0.0,
    });
    scene_builder.add_sphere(
        Sphere {
            center: Point3 {
                x: 4.,
                y: 1.,
                z: 0.,
            },
            radius: 1.,
        },
        m3,
    );

    (
        scene_builder.build(),
        Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    )
}

fn teapot_and_spheres() -> (BvhScene, Point3) {
    let material = Material::Metal {
        albedo: ColorRgb {
            r: 0.8,
            g: 0.8,
            b: 0.9,
        },
        fuzz: 0.,
    };
    match std::env::current_dir() {
        Ok(path) => println!("Current working directory: {}", path.display()),
        Err(e) => eprintln!("Error getting current directory: {}", e),
    }

    let current_dir = std::env::current_dir().unwrap();
    let obj_file_path = current_dir.join("teapot.obj");
    let triangles = obj::parse(obj_file_path.to_str().unwrap()).unwrap();

    let mut scene_builder = SceneBuilder::empty();

    let teapot_material = scene_builder.add_material(Material::Metal {
        albedo: ColorRgb {
            r: 1.,
            g: 1.,
            b: 1.,
        },
        fuzz: 0.2,
    });

    for t in triangles {
        scene_builder.add_triangle(t, teapot_material);
    }

    let ground_material = scene_builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.5,
            g: 0.5,
            b: 0.5,
        },
    });
    let ground_center = Point3 {
        x: 0.,
        y: -10000.,
        z: 0.,
    };
    let ground_radius = 10000.;

    scene_builder.add_sphere(
        Sphere {
            center: ground_center,
            radius: ground_radius,
        },
        ground_material,
    );

    let m1 = scene_builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.8,
            g: 0.8,
            b: 0.3,
        },
    });
    scene_builder.add_sphere(
        Sphere {
            center: Point3 {
                x: -1.5,
                y: 0.8,
                z: 3.,
            },
            radius: 0.8,
        },
        m1,
    );

    let m2 = scene_builder.add_material(Material::Lambertian {
        albedo: ColorRgb {
            r: 0.7,
            g: 0.2,
            b: 0.1,
        },
    });
    scene_builder.add_sphere(
        Sphere {
            center: Point3 {
                x: 1.5,
                y: 0.8,
                z: 3.,
            },
            radius: 0.8,
        },
        m2,
    );

    (
        scene_builder.build(),
        Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
    )
}

struct RotateCamera {
    phi: f32,
    theta: f32,
    dist: f32,
    position: Point3,
}

impl RotateCamera {
    fn rotate(&mut self, dx: f32, dy: f32) {
        self.phi += dx * 6. / WIDTH as f32;
        self.theta += dy * 3. / HEIGHT as f32;
        self.theta = self.theta.clamp(0., std::f32::consts::PI);
    }

    fn change_dist(&mut self, d: f32) {
        self.dist += d;
        self.dist = self.dist.clamp(0., 100.);
    }

    fn get_camera(&self) -> Camera {
        let pos = Point3 {
            x: (self.dist * self.phi.cos() * self.theta.cos()) as f64,
            z: (self.dist * self.phi.sin() * self.theta.cos()) as f64,
            y: (self.dist * self.theta.sin()) as f64,
        } + (self.position - Point3::origin());
        Camera::new(
            &pos,
            &self.position,
            &Vector3 {
                x: 0.,
                y: 1.,
                z: 0.,
            },
            60f64.to_radians(),
        )
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

    println!("Which scene do you want to see?");
    println!("a - three dragons");
    println!("b - lots of spheres");
    println!("c (or anything) - teapot");

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let input = s.trim(); // Trim whitespace and newlines from the input
    let (scene, pos) = if input == "a" {
        dragon_scene()
    } else if input.chars().next() == Some('b') {
        big_spheres_scene()
    } else {
        teapot_and_spheres()
    };

    let mut rot = RotateCamera {
        phi: 0.,
        theta: 0.,
        dist: 10.,
        position: pos,
    };

    let mut renderer =
        IterativeRenderer::new(rot.get_camera(), WIDTH as i32, HEIGHT as i32, scene, true);

    let mut mouse_x = 0f32;
    let mut mouse_y = 0f32;
    let mut mouse_down = false;

    rayon::ThreadPoolBuilder::new()
        .num_threads(14)
        .build_global()
        .unwrap();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        if window.get_mouse_down(MouseButton::Left) {
            if !mouse_down {
                if let Some((xx, yy)) = window.get_mouse_pos(MouseMode::Pass) {
                    mouse_down = true;
                    mouse_x = xx;
                    mouse_y = yy;
                }
            } else {
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
        } else {
            mouse_down = false;
        }

        if let Some((_, ys)) = window.get_scroll_wheel() {
            rot.change_dist(ys * -0.3);
            renderer.set_camera(rot.get_camera());
        }

        use std::time::Instant;
        let now = Instant::now();
        {
            renderer.render(&mut buffer, 1);
        }
        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
