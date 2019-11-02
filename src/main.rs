use rtiow::{hit, hit::Hit, image, material, ray, sphere, vec3};
use std::env::args;
use std::f32;
use std::fs::File;
use std::io::stdout;
use std::io::Write;
use std::process::exit;

use vec3::Vec3;

fn color(mut v: Vec3) -> image::Color {
    v *= 255.99;
    image::Color {
        red: v.x() as u8,
        green: v.y() as u8,
        blue: v.z() as u8,
    }
}

fn get_color_components(ray: &ray::Ray, hittable: &impl Hit, depth: i32) -> Vec3 {
    match hittable.hit(ray, 0.001, f32::MAX) {
        Some(hit) => match (depth, hit.material.scatter(ray, &hit.normal)) {
            (
                0..=50,
                Some(ray::ScatteredRay {
                    ray: next,
                    attenuation,
                }),
            ) => get_color_components(&next, hittable, depth + 1) * attenuation,
            _ => Vec3::new(0.0, 0.0, 0.0),
        },
        None => {
            let t = (ray.direction().y() + 1.0) * 0.5;
            Vec3::new(1.0, 1.0, 1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn create_image() -> i32 {
    let args = args().collect::<Vec<String>>();
    let stdout_handler;
    let mut stdout_lock_handler;
    let mut file_handler;
    let ostream: &mut dyn Write = match args.len() {
        3..=std::usize::MAX => {
            eprintln!("Too many arguments");
            return 1;
        }
        1 => {
            stdout_handler = stdout();
            stdout_lock_handler = stdout_handler.lock();
            &mut stdout_lock_handler
        }
        _ => match File::create(args[1].clone()) {
            Ok(file) => {
                file_handler = file;
                &mut file_handler
            }
            Err(err) => {
                eprintln!("{}", err);
                return 1;
            }
        },
    };
    let mut image = image::Image::new(400, 800);
    let random_points_number = 100;
    let origin = Vec3::new(0.0, 0.0, 0.0);
    let upper_left_corner = Vec3::new(-2.0, 1.0, -1.0);

    let material1 = material::Lambertian::new(Vec3::new(0.8, 0.3, 0.3));
    let material2 = material::Lambertian::new(Vec3::new(0.8, 0.3, 0.0));
    let material3 = material::Metal::new(Vec3::new(0.8, 0.3, 0.0), 0.3);
    let material4 = material::Metal::new(Vec3::new(0.8, 0.3, 0.0), 1.0);

    let sphere1 = sphere::Sphere::new(Vec3::new(0.0, 0.0, -2.0), 1.0, &material1);
    let sphere2 = sphere::Sphere::new(Vec3::new(0.0, -201.0, -2.0), 200.0, &material2);
    let sphere3 = sphere::Sphere::new(Vec3::new(2.0, 0.0, -2.0), 1.0, &material3);
    let sphere4 = sphere::Sphere::new(Vec3::new(-2.0, 0.0, -2.0), 1.0, &material4);

    let mut world = hit::HitList::default();

    world.push(&sphere1);
    world.push(&sphere2);
    world.push(&sphere3);
    world.push(&sphere4);

    let height = image.height();
    let width = image.width();
    image.process_in_parallel(|i, j| {
        let down = Vec3::new(0.0, i as f32 + rand::random::<f32>(), 0.0) / (height as f32) * 2.0;
        let mut normal = Vec3::new(0.0, 0.0, 0.0);
        for _ in 0..random_points_number {
            let right =
                Vec3::new(j as f32 + rand::random::<f32>(), 0.0, 0.0) / (width as f32) * 4.0;
            let ray = ray::Ray::new(origin, upper_left_corner - down + right);
            normal += get_color_components(&ray, &world, 0);
        }
        normal /= random_points_number as f32;
        color(Vec3::new(
            normal.x().sqrt(),
            normal.y().sqrt(),
            normal.z().sqrt(),
        ))
    });
    match image.write_ppm(ostream) {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    }
}

fn main() {
    exit(create_image());
}
