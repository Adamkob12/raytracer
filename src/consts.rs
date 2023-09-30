// Constants
use bevy_math::prelude::*;
use core::ops::Add;

pub const CAMERA_POS: Vec3 = Vec3::new(0.0, 0.0, 0.0);
pub const O: Vec3 = CAMERA_POS;

pub const CAMERA_DIRECTION: Vec3 = Vec3::Z;
pub const FORWARD: Vec3 = CAMERA_DIRECTION;
pub const UP: Vec3 = Vec3::Y;
pub const RIGHT: Vec3 = Vec3::X;

pub const V_W: f32 = V_H * ASPECT_RATIO;
pub const V_H: f32 = 1.0;
pub const DISTANCE_VIEWPORT_TO_CAMERA: f32 = 1.0;
pub const D: f32 = DISTANCE_VIEWPORT_TO_CAMERA;

pub const IMAGE_RES: UVec2 = UVec2::new(1920, 1080);
const SCALE_FACTOR: f32 = 2.0;
pub const C_W: u32 = (IMAGE_RES.x as f32 * SCALE_FACTOR) as u32;
pub const C_H: u32 = (IMAGE_RES.y as f32 * SCALE_FACTOR) as u32;
const ASPECT_RATIO: f32 = C_W as f32 / C_H as f32;

pub const INF: i32 = i32::max_value();
pub const NEG_INF: i32 = i32::min_value();

pub const BACKGROUND_COLOR: Color = Color {
    r: 50,
    g: 50,
    b: 50,
};

pub const MAX_REFLCT: u32 = 3;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn apply_light_intensity(self, intensity: f32) -> Color {
        Color {
            r: ((self.r as f32) * intensity).min(255.0) as u8,
            g: ((self.g as f32) * intensity).min(255.0) as u8,
            b: ((self.b as f32) * intensity).min(255.0) as u8,
        }
    }
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

pub fn canvas_to_viewport(x: i32, y: i32) -> Vec3 {
    Vec3::new(
        x as f32 * (V_W / C_W as f32),
        y as f32 * (V_H / C_H as f32),
        D,
    )
}

impl Into<[u8; 3]> for Color {
    fn into(self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

impl From<[u8; 3]> for Color {
    fn from(arr: [u8; 3]) -> Color {
        Color {
            r: arr[0],
            g: arr[1],
            b: arr[2],
        }
    }
}

pub fn quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let disc = b * b - 4.0 * a * c;
    if disc < 0.0 {
        return None;
    }
    if disc == 0.0 {
        let tmp = -b / 2.0 * a;
        return Some((tmp, tmp));
    } else {
        let disc = disc.powf(0.5);
        return Some(((-b + disc) / (2.0 * a), (-b - disc) / (2.0 * a)));
    }
}
