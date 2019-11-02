use crate::{hit::Hit, hit::HitRecord, ray::Ray, material::Scatter, vec3::Vec3};

pub struct Sphere<'a> {
    radius: f32,
    center: Vec3,
    material: &'a dyn Scatter,
}

impl<'a> Sphere<'a> {
    pub fn new(center: Vec3, radius: f32, material: &'a dyn Scatter) -> Sphere<'a> {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl<'a> Hit for Sphere<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let b = ray.direction().dot(ray.origin() - self.center);
        let a = ray.direction().squared_length();
        let c = (ray.origin() - self.center).squared_length() - self.radius.powi(2);
        let determ = b.powi(2) - a * c;
        if determ < 0.0 {
            return None;
        }
        let mut t = (-b - determ.sqrt()) / a;
        if t_min > t || t > t_max {
            t = (-b + determ.sqrt()) / a;
        }
        if t_min > t || t > t_max {
            return None;
        }
        let intersection = ray.point_at_parameter(t);
        let from_sphere_center = intersection - self.center;
        Some(HitRecord {
            t,
            normal: Ray::new(
                intersection,
                from_sphere_center / from_sphere_center.length(),
            ),
            material: self.material,
        })
    }
}
