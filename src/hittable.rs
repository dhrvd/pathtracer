use std::sync::Arc;

use crate::aabb::Aabb;
use crate::material::Material;
use crate::math::{Ray, Vec3};

pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord>;
    fn aabb(&self) -> Aabb;
}

pub struct HittableList {
    pub hittables: Vec<Box<dyn Hittable>>,
    aabb: Aabb,
}

impl HittableList {
    pub fn new(hittables: Vec<Box<dyn Hittable>>) -> Self {
        let aabb = hittables
            .iter()
            .fold(Aabb::EMPTY, |aabb, h| aabb.join(&h.aabb()));

        HittableList { hittables, aabb }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.aabb.join_mut(&hittable.aabb());
        self.hittables.push(hittable);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let mut temp_rec = None;
        let mut closest_so_far = ray_tmax;

        for hittable in &self.hittables {
            if let Some(rec) = hittable.hit(ray, ray_tmin, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }

        temp_rec
    }

    fn aabb(&self) -> Aabb {
        self.aabb
    }
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub uv: (f32, f32),
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        ray: &Ray,
        t: f32,
        uv: (f32, f32),
        material: Arc<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            point,
            normal,
            t,
            uv,
            front_face,
            material,
        }
    }
}
