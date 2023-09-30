use crate::{Color, Vec3};

// Sphere: position, radius, color, shininess
pub enum Primitive {
    Sphere(Vec3, f32, Color, f32),
}

// const ERROR_MARGIN: f32 = 0.1;
// fn within_error_margin(x: f32, y: f32) -> bool {
//     (x < y + ERROR_MARGIN) && (x > y - ERROR_MARGIN)
// }

impl Primitive {
    pub fn color(&self) -> Option<Color> {
        match self {
            Primitive::Sphere(_center, _radius, color, _) => {
                return Some(*color);
            }
            #[allow(unreachable_patterns)]
            _ => todo!("implement unimplemented primitive case for `color`"),
        }
    }

    pub fn compute_normal(&self, point: Vec3) -> Vec3 {
        match self {
            Primitive::Sphere(center, _radius, _color, _) => (point - *center).normalize(),
            #[allow(unreachable_patterns)]
            _ => todo!("implement unimplemented primitive case for `compute_normal`"),
        }
    }

    pub fn get_shininess(&self) -> f32 {
        match self {
            Primitive::Sphere(_center, _radius, _color, s) => *s,
            #[allow(unreachable_patterns)]
            _ => todo!("implement unimplemented primitive case for `get_shininess`"),
        }
    }
}
