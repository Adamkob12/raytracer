#![allow(dead_code)]
mod consts;
mod light;
mod ray;
mod scene;

pub use bevy_math::prelude::*;
pub use consts::*;
use image::{ImageBuffer, Rgb};
use ray::*;
use scene::*;

fn main() {
    let mut buffer = ImageBuffer::new(C_W, C_H);
    let width = C_W as i32;
    let height = C_H as i32;
    let scene = setup_scene();

    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
        // dbg!(x, y);
        let color = trace_ray(
            O,
            canvas_to_viewport(x as i32 - width / 2, y as i32 - height / 2),
            1,
            INF,
            &scene,
        );
        *pixel = Rgb(color.into());
    }
    buffer.save("output/render.png").unwrap();
}

fn setup_scene() -> Scene {
    let mut scene = Scene::new();
    scene.insert_primitive(Primitive::Sphere(
        [1.0, 0.0, 5.0].into(),
        1.0,
        Color::from([255, 0, 0]),
    ));
    scene.insert_primitive(Primitive::Sphere(
        [0.0, 0.0, 4.0].into(),
        1.0,
        Color::from([0, 255, 0]),
    ));
    scene.insert_primitive(Primitive::Sphere(
        [-1.0, 0.0, 6.0].into(),
        1.0,
        Color::from([0, 0, 255]),
    ));
    scene
}
