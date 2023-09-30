#![allow(dead_code)]
mod consts;
pub mod light;
pub mod primitive;
mod ray;
mod scene;

pub use bevy_math::prelude::*;
pub use consts::*;
use image::{ImageBuffer, Rgb};
pub use light::*;
pub use primitive::*;
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
            canvas_to_viewport(x as i32 - width / 2, (y as i32 - height / 2) * -1),
            1,
            INF,
            &scene,
            0,
        );
        *pixel = Rgb(Color::into(color.unwrap_or(BACKGROUND_COLOR)));
    }
    buffer.save("output/render.png").unwrap();
}

fn setup_scene() -> Scene {
    let mut scene = Scene::new();
    scene.insert_primitive(Primitive::Sphere(
        [0.0, -1.0, 3.0].into(),
        1.0,
        Color::from([255, 0, 0]),
        500.0,
        0.7,
    ));
    scene.insert_primitive(Primitive::Sphere(
        [-2.0, 0.0, 4.0].into(),
        1.0,
        Color::from([0, 255, 0]),
        10.0,
        0.2,
    ));
    scene.insert_primitive(Primitive::Sphere(
        [2.0, 0.0, 4.0].into(),
        1.0,
        Color::from([0, 0, 255]),
        500.0,
        0.3,
    ));
    scene.insert_primitive(Primitive::Sphere(
        [0.0, -5001.0, 0.0].into(),
        5000.0,
        Color::from([255, 255, 0]),
        1000.0,
        0.5,
    ));
    scene.insert_light_source(LightSource::Point(0.5, [2.0, 1.0, 0.0].into()));
    // scene.insert_light_source(LightSource::Ambient(0.2));
    scene.insert_light_source(LightSource::Directional(0.5, [1.0, 4.0, 4.0].into()));
    scene
}
