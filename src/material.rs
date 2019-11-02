use crate::{
    ray::{Ray, ScatteredRay},
    vec3::Vec3,
};
use rand::distributions::{Distribution, UnitSphereSurface};
use std::marker::Sync;

pub trait Scatter: Sync {
    fn scatter(&self, ray: &Ray, normal: &Ray) -> Option<ScatteredRay>;
}

fn rand_point_in_sphere() -> Vec3 {
    let sphere = UnitSphereSurface::new();
    let [x, y, z] = sphere.sample(&mut rand::thread_rng());
    Vec3::new(x as f32, y as f32, z as f32) * rand::random::<f32>()
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn albedo(&self) -> Vec3 {
        self.albedo
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, normal: &Ray) -> Option<ScatteredRay> {
        Some(ScatteredRay {
            ray: Ray::new(normal.origin(), normal.direction() + rand_point_in_sphere()),
            attenuation: self.albedo(),
        })
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, mut fuzz: f32) -> Metal {
        if fuzz > 1.0 {
            fuzz = 1.0
        }
        Metal { albedo, fuzz }
    }

    pub fn albedo(&self) -> Vec3 {
        self.albedo
    }

    pub fn fuzz(&self) -> f32 {
        self.fuzz
    }
}

fn reflect(dir: Vec3, normal: Vec3) -> Vec3 {
    dir - normal * 2.0 * dir.dot(normal)
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, normal: &Ray) -> Option<ScatteredRay> {
        let reflected = reflect(ray.direction(), normal.direction());
        Some(ScatteredRay {
            ray: Ray::new(normal.origin(), reflected + rand_point_in_sphere() * self.fuzz()),
            attenuation: self.albedo(),
        })
        .filter(|_| reflected.dot(normal.direction()) > 0.0)
    }
}
