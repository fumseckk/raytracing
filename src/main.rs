mod camera;
mod hittable;
mod material;
mod mathutils;
mod ray;
mod vec3;
use camera::*;
use hittable::*;
use material::*;
use rand;
use ray::*;
use std::fs::File;
use std::io::Write;
use std::rc::Rc;
use vec3::*;

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_DEPTH: u32 = 50;

    // World

    let mut world: HittableList = HittableList { objects: vec![] };

    let material_ground = Rc::new(Lambertian {
        albedo: Color::from(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::from(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::from(0.8, 0.8, 0.8),
        fuzz: 0.3,
    });
    let material_right = Rc::new(Metal {
        albedo: Color::from(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    world.add(Rc::new(Sphere {
        center: Point3::from(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: material_ground,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::from(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_center,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::from(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_left,
    }));
    world.add(Rc::new(Sphere {
        center: Point3::from(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: material_right,
    }));

    // Camera
    let cam = Camera::new();

    // Render
    let mut file = init_image_file("render.ppm", IMAGE_WIDTH, IMAGE_HEIGHT).unwrap_or_else(|err| {
        eprintln!("Error initializing image file: {}", err);
        std::process::exit(1);
    });

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);

        for i in 0..IMAGE_WIDTH {
            let mut color = Color::from(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = cam.get_ray(u, v);
                let pixel_color = ray_color(&r, &world, MAX_DEPTH);
                color += pixel_color;
            }
            Color::write_color(&mut file, color, SAMPLES_PER_PIXEL).unwrap();
        }
    }

    eprintln!("Done.");
}

fn init_image_file(name: &str, width: u32, height: u32) -> std::io::Result<File> {
    let mut file = File::create(name)?;
    writeln!(&mut file, "P3")?;
    writeln!(&mut file, "{} {}", width, height)?;
    writeln!(&mut file, "255")?;
    Ok(file)
}

fn ray_color(ray: &Ray, world: &HittableList, depth: u32) -> Color {
    // Base recursion case
    if depth <= 0 {
        return Color::from(0., 0., 0.);
    }

    // World processing
    if let Some(rec) = world.hit(ray, 0.001, f64::MAX) {
        match rec.mat_ptr.scatter(ray, &rec) {
            None => return Color::from(0., 0., 0.), // If no bouncing ray emitted
            Some((scattered, attenuation)) => {
                return attenuation * ray_color(&scattered, world, depth - 1)
            }
        }
    }

    // Skybox
    let unit_direction = ray.direction.normalized();
    let t = 0.5 * (unit_direction.y() + 1.);
    (1.0 - t) * Color::from(1., 1., 1.) + t * Color::from(0.5, 0.7, 1.0)
}
