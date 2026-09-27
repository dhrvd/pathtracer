use std::sync::Arc;

use crate::bvh::BVHNode;
use crate::camera::Camera;
use crate::hittable::{Hittable, HittableList};
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::math::{degrees_to_radians, random, random_rng, random_vec3, vec3, Vec3};
use crate::objects::Sphere;
use crate::texture::Noise;
use crate::IMAGE_WIDTH;

pub fn bouncing_spheres(image_width: u32, image_height: u32) -> (BVHNode, Camera) {
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let material_ground = Arc::new(Lambertian::checkered(
        0.32,
        vec3(0.2, 0.3, 0.1),
        vec3(0.9, 0.9, 0.9),
    ));
    world.push(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = vec3(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - vec3(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vec3(0.0, 1.0) * random_vec3(0.0, 1.0);
                    let material = Arc::new(Lambertian::solid(albedo));

                    let center2 = center + vec3(0.0, random_rng(0.0, 0.5), 0.0);
                    world.push(Box::new(Sphere::moving(center, center2, 0.2, material)));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3(0.5, 1.0);
                    let fuzz = random_rng(0.0, 0.5);

                    let material = Arc::new(Metal::new(albedo, fuzz));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    let material = Arc::new(Dielectric::new(1.5));
                    world.push(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Box::new(Sphere::new(vec3(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::solid(vec3(0.4, 0.2, 0.1)));
    world.push(Box::new(Sphere::new(vec3(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(vec3(0.7, 0.6, 0.5), 0.0));
    world.push(Box::new(Sphere::new(vec3(4.0, 1.0, 0.0), 1.0, material3)));

    let world = BVHNode::new(&mut world);

    let camera = Camera::new(
        vec3(13.0, 2.0, 3.0),
        Vec3::ZEROS,
        vec3(0.0, 1.0, 0.0),
        degrees_to_radians(20.0),
        degrees_to_radians(0.6),
        10.0,
        (image_width, image_height),
    );

    (world, camera)
}

pub fn checkered_spheres(image_width: u32, image_height: u32) -> (HittableList, Camera) {
    let mut world = HittableList::new(Vec::new());

    let material: Arc<dyn Material> = Arc::new(Lambertian::checkered(
        0.32,
        vec3(0.2, 0.3, 0.1),
        vec3(0.9, 0.9, 0.9),
    ));

    world.add(Box::new(Sphere::new(
        vec3(0.0, -10.0, 0.0),
        10.0,
        Arc::clone(&material),
    )));
    world.add(Box::new(Sphere::new(
        vec3(0.0, 10.0, 0.0),
        10.0,
        Arc::clone(&material),
    )));

    let camera = Camera::new(
        vec3(13.0, 2.0, 3.0),
        Vec3::ZEROS,
        vec3(0.0, 1.0, 0.0),
        degrees_to_radians(20.0),
        degrees_to_radians(0.0),
        1.0,
        (image_width, image_height),
    );

    (world, camera)
}

pub fn perlin_spheres(image_width: u32, image_height: u32) -> (HittableList, Camera) {
    let mut world = HittableList::new(Vec::new());

    let perlin_texture = Box::new(Noise::new(4.0));
    let material: Arc<dyn Material> = Arc::new(Lambertian::new(perlin_texture));

    world.add(Box::new(Sphere::new(
        vec3(0.0, -1000.0, 0.0),
        1000.0,
        Arc::clone(&material),
    )));

    world.add(Box::new(Sphere::new(
        vec3(0.0, 2.0, 0.0),
        2.0,
        Arc::clone(&material),
    )));

    let camera = Camera::new(
        vec3(13.0, 2.0, 3.0),
        Vec3::ZEROS,
        vec3(0.0, 1.0, 0.0),
        degrees_to_radians(20.0),
        degrees_to_radians(0.0),
        1.0,
        (image_width, image_height),
    );

    (world, camera)
}
