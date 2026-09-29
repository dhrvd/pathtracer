use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::math::{Ray, Vec3, PI};

pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center, Vec3::ZEROS, 0.0),
            radius,
            material,
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f32, material: Arc<dyn Material>) -> Self {
        Self {
            center: Ray::new(center1, center2 - center1, 0.0),
            radius,
            material,
        }
    }

    pub fn get_sphere_uv(point: Vec3) -> (f32, f32) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;

        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if root <= ray_tmin || ray_tmax <= root {
            root = (h + sqrtd) / a;
            if root <= ray_tmin || ray_tmax <= root {
                return None;
            }
        }

        let point = ray.at(root);
        let outward_normal = (point - current_center) / self.radius;
        let uv = Sphere::get_sphere_uv(outward_normal);

        Some(HitRecord::new(
            point,
            outward_normal,
            ray,
            root,
            uv,
            Arc::clone(&self.material),
        ))
    }

    fn aabb(&self) -> Aabb {
        let box1 = Aabb::new(
            self.center.at(0.0) - self.radius,
            self.center.at(0.0) + self.radius,
        );
        let box2 = Aabb::new(
            self.center.at(1.0) - self.radius,
            self.center.at(1.0) + self.radius,
        );

        box1.join(&box2)
    }
}

pub struct Quad {
    Q: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    material: Arc<dyn Material>,
    normal: Vec3,
    D: f32,
}

impl Quad {
    pub fn new(Q: Vec3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        let normal = (u * v).normalize();
        let D = normal.dot(Q);
        let w = normal / normal.dot(normal);

        Self {
            Q,
            u,
            v,
            w,
            material,
            normal,
            D,
        }
    }

    fn is_interior(a: f32, b: f32) -> bool {
            true
        } else {
            Some((a, b))
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, ray_tmin: f32, ray_tmax: f32) -> Option<HitRecord> {
        let denom = self.normal.dot(ray.direction);

        if denom.abs() < 1e-8 {
            return None;
        };

        let t = (self.D - self.normal.dot(ray.origin)) / denom;
        if !(ray_tmin < t && t < ray_tmax) {
            return None;
        }

        let intersection = ray.at(t);

        let planar_hitpt = intersection - self.Q;
        let a = self.w.dot(planar_hitpt * self.v);
        let b = self.w.dot(self.u * planar_hitpt);

        if !(0.0 < a && a < 1.0) || !(0.0 < b && b < 1.0) {
            return None;
        }

        Some(HitRecord::new(
            intersection,
            self.normal,
            ray,
            t,
            (a, b),
            Arc::clone(&self.material),
        ))
    }

    fn aabb(&self) -> Aabb {
        let box_diag1 = Aabb::new(self.Q, self.Q + self.u + self.v);
        let box_diag2 = Aabb::new(self.Q + self.u, self.Q + self.v);

        box_diag1.join(&box_diag2)
    }
}
