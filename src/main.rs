mod aabb;
mod bvh;
mod camera;
mod hittable;
mod material;
mod math;
mod objects;
mod perlin;
mod render;
mod scenes;
mod texture;

const IMAGE_WIDTH: u32 = 400;
const ASPECT_RATIO: f32 = 16.0 / 9.0;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

const SAMPLES_PER_PIXEL: u32 = 100 * 4;
const MAX_DEPTH: u32 = 50 * 2;

fn main() {
    let (world, camera) = scenes::perlin_spheres(IMAGE_WIDTH, IMAGE_HEIGHT);

    render::render(
        camera,
        world,
        (IMAGE_WIDTH, IMAGE_HEIGHT),
        SAMPLES_PER_PIXEL,
        MAX_DEPTH,
        "image.png",
    );
}
