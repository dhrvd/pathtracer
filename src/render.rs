use console::Style;
use indicatif::{ProgressBar, ProgressStyle};

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::math::{linear_to_gamma, Ray, Vec3, INFINITY};

fn ray_color<H: Hittable>(ray: &Ray, world: &H, depth: u32, background: Vec3) -> Vec3 {
    if depth == 0 {
        return Vec3::ZEROS;
    }

    let Some(rec) = world.hit(ray, 0.001, INFINITY) else {
        return background;
    };

    let emitted = rec.material.emitted(rec.uv, rec.point);

    let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) else {
        return emitted;
    };

    emitted + attenuation * ray_color(&scattered, world, depth - 1, background)
}

pub fn render<H: Hittable>(
    camera: Camera,
    world: H,
    size: (u32, u32),
    samples_per_pixel: u32,
    max_depth: u32,
    path: &str,
) {
    let pb = ProgressBar::new(size.1 as u64);
    pb.set_style(
        ProgressStyle::with_template("{prefix:.cyan.bold} [{bar:25}] {percent_precise}%")
            .unwrap()
            .progress_chars("=> "),
    );
    pb.set_prefix("Rendering");

    let mut buffer = Vec::with_capacity((size.0 * size.1 * 3) as usize);

    for j in 0..size.1 {
        for i in 0..size.0 {
            let mut color = Vec3::ZEROS;
            for _ in 0..samples_per_pixel {
                color += ray_color(
                    &camera.get_ray(i, j),
                    &world,
                    max_depth,
                    camera.background(),
                )
            }
            color /= samples_per_pixel as f32;

            let rbyte = (255.999 * linear_to_gamma(color.x)) as u8;
            let gbyte = (255.999 * linear_to_gamma(color.y)) as u8;
            let bbyte = (255.999 * linear_to_gamma(color.z)) as u8;

            buffer.push(rbyte);
            buffer.push(gbyte);
            buffer.push(bbyte);
        }

        pb.inc(1);
    }

    pb.finish_and_clear();
    image::save_buffer(path, &buffer, size.0, size.1, image::ColorType::Rgb8).unwrap();

    println!(
        "{} to `{}` in {:?}",
        Style::new().green().bold().apply_to("Rendered"),
        path,
        pb.elapsed()
    );
}
