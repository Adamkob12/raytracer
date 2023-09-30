use crate::ray::intersect_ray;
use crate::Primitive;
use crate::Vec3;

// Point: intensity, position
// Directional: intensity, direction
// Ambient: intensity
pub enum LightSource {
    Point(f32, Vec3),
    Directional(f32, Vec3),
    Ambient(f32),
}

// Gets a point on some primitive, and a normal corresponding.
// Calculates the intensity of the light at that point.
pub fn compute_light(
    source_to_point: Vec3,
    shininess_factor: f32,
    point: Vec3,
    normal: Vec3,
    light_source: &LightSource,
) -> f32 {
    let direction: Vec3;
    let mut i: f32 = 0.0;
    let tmp;
    match light_source {
        LightSource::Ambient(intensity) => {
            i += *intensity;
            return i;
        }
        LightSource::Point(intensity, light_pos) => {
            direction = *light_pos - point;
            i += calc_intensity(direction, normal, *intensity);
            tmp = *intensity;
        }
        LightSource::Directional(intensity, dir) => {
            direction = *dir;
            i += calc_intensity(direction, normal, *intensity);
            tmp = *intensity;
        }
    }
    // On top of the regular light, we also calculate the shininess of the primitive at a given
    // intersection point, based on the primitive's `shininess_factor`.
    if shininess_factor != 0.0 {
        // If a ray of light hits an intersection point, with a normal (`normal`), `refl` would be
        // the reflection of that ray, kind of like if a laser hits a smooth mirror, it will bounce
        // off a reflect back onto us, this calculates that vector that comes back after the hit.
        let refl = 2.0 * normal * normal.dot(direction) - direction;
        // How much "shine" the observor will see depends on his position, kind of like how you can
        // angle something shiny to reflect sunlight in different directions.
        let refl_dot_ray = refl.dot(source_to_point);
        if refl_dot_ray > 0.0 {
            i += tmp
                * (refl_dot_ray / (refl.length() * source_to_point.length()))
                    .powf(shininess_factor);
        }
    }
    i
}

fn calc_intensity(direction_vector: Vec3, normal: Vec3, intensity: f32) -> f32 {
    let dot = normal.dot(direction_vector);
    return ((dot / direction_vector.length()) * intensity).max(0.0);
    // if the light comes from behind the point, the intensity will end up being negative (we dont
    // want that, so we use .max(0.0))
}

// Function checks given a point, a light source and a collection (Iterator) of possible obstacles,
// wether the point is seen by the light source, aka is the point in shade.
pub fn is_visible_by_light<'a>(
    point: Vec3,
    light: &LightSource,
    obstacles: impl Iterator<Item = &'a Primitive>,
) -> bool {
    let direction: Vec3;
    let min = 0.001;
    let max;
    match light {
        LightSource::Ambient(_) => return true,
        LightSource::Point(_, light_pos) => {
            direction = *light_pos - point;
            max = 1.0;
        }
        LightSource::Directional(_, dir) => {
            max = f32::MAX;
            direction = *dir;
        }
    }
    for obst in obstacles {
        if let Some((t, s)) = intersect_ray(point, direction, obst) {
            if min < t && t < max || min < s && s < max {
                return false;
            }
        }
    }
    true
}
