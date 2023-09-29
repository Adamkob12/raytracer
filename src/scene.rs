use crate::{Color, Vec3};
use tracing::warn;

pub struct Scene {
    primitives: Vec<Primitive>,
    lights: Vec<LightSource>,
}

impl Scene {
    pub fn iter_primitives(&self) -> impl Iterator<Item = &Primitive> {
        self.primitives.iter()
    }

    pub fn iter_lights(&self) -> impl Iterator<Item = &LightSource> {
        self.lights.iter()
    }

    pub fn insert_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
    }

    pub fn insert_light_source(&mut self, light: LightSource) {
        self.lights.push(light);
        let mut light_intensity_total = 0.0;
        for light in self.iter_lights() {
            match light {
                LightSource::Point(intensity, _) => light_intensity_total += intensity,
                LightSource::Directional(intensity, _) => light_intensity_total += intensity,
                LightSource::Ambient(intensity) => light_intensity_total += intensity,
            }
        }
        if light_intensity_total > 1.0 {
            warn!("The total light intensity is over 1.0, some points may be overexposed");
        }
    }

    pub fn new() -> Self {
        Scene {
            primitives: vec![],
            lights: vec![],
        }
    }
}

// Point: intensity, position
// Directional: intensity, direction
// Ambient: intensity
pub enum LightSource {
    Point(f32, Vec3),
    Directional(f32, Vec3),
    Ambient(f32),
}

// Sphere: position, radius, color
pub enum Primitive {
    Sphere(Vec3, f32, Color),
}

// const ERROR_MARGIN: f32 = 0.1;
// fn within_error_margin(x: f32, y: f32) -> bool {
//     (x < y + ERROR_MARGIN) && (x > y - ERROR_MARGIN)
// }

impl Primitive {
    pub fn color(&self) -> Option<&Color> {
        match self {
            Primitive::Sphere(_center, _radius, color) => {
                return Some(color);
            }
            #[allow(unreachable_patterns)]
            _ => todo!("implement unimplemented primitive case for `color_at`"),
        }
    }
}
