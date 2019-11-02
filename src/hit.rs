use crate::{ray::Ray, material::Scatter};
use std::marker::Sync;
use std::vec::Vec;

pub struct HitRecord<'a> {
    pub t: f32,
    pub normal: Ray,
    pub material: &'a dyn Scatter,
}

pub trait Hit: Sync {
    fn hit<'a>(&'a self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord<'a>>;
}

pub struct HitList<'a> {
    collection: Vec<&'a dyn Hit>,
}

impl<'a> Hit for HitList<'a> {
    fn hit(&self, ray: &Ray, t_min: f32, mut t_max: f32) -> Option<HitRecord> {
        let mut result = None;
        for hittable in &self.collection {
            let cur = hittable.hit(ray, t_min, t_max);
            if let Some(hit_record) = &cur {
                t_max = hit_record.t;
                result = cur;
            }
        }
        result
    }
}

impl<'a> Default for HitList<'a> {
    fn default() -> Self {
        HitList { collection: vec![] }
    }
}

impl<'a> HitList<'a> {
    pub fn push(&mut self, other: &'a impl Hit) {
        self.collection.push(other);
    }
}
