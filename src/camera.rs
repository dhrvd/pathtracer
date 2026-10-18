use crate::math::{random, random_rng, vec3, Ray, Vec3};

fn sample_square() -> Vec3 {
    vec3(random() - 0.5, random() - 0.5, 0.0)
}

fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = vec3(random_rng(-1.0, 1.0), random_rng(-1.0, 1.0), 0.0);
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub struct Camera {
    center: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    defocus_angle: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
    background: Vec3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: f32,
        defocus_angle: f32,
        focus_dist: f32,
        image_size: (u32, u32),
        background: Vec3,
    ) -> Self {
        let viewport_height = 2.0 * focus_dist * (vfov / 2.0).tan();
        let viewport_width = viewport_height * (image_size.0 as f32 / image_size.1 as f32);

        let w = (look_from - look_at).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        let pixel_delta_u = viewport_u / image_size.0 as f32;
        let pixel_delta_v = viewport_v / image_size.1 as f32;

        let viewport_upper_left =
            look_from - (w * focus_dist) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).tan();

        Self {
            center: look_from,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            defocus_angle,
            defocus_disk_u: u * defocus_radius,
            defocus_disk_v: v * defocus_radius,
            background,
        }
    }

    pub fn background(&self) -> Vec3 {
        self.background
    }

    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel00_loc
            + (self.pixel_delta_u * (offset.x + i as f32))
            + (self.pixel_delta_v * (offset.y + j as f32));

        let origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };

        Ray::new(origin, pixel_sample - origin, random())
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }
}
