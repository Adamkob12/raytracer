use crate::*;

pub fn trace_ray(
    source: Vec3,
    dest: Vec3,
    t_min: i32,
    t_max: i32,
    scene: &Scene,
    current_reflection: u32,
) -> Option<Color> {
    let mut closest = INF as f32;
    let mut closest_prim = None;
    for prim in scene.iter_primitives() {
        if let Some((t1, t2)) = intersect_ray(source, dest, prim) {
            if (t_min as f32) < t1 && t1 < t_max as f32 && t1 < closest {
                closest = t1;
                closest_prim = Some(prim);
            }
            if (t_min as f32) < t2 && t2 < t_max as f32 && t2 < closest {
                closest = t2;
                closest_prim = Some(prim);
            }
        }
    }
    // Calculate light
    if let Some(p) = closest_prim {
        let intersection_point = source + closest * dest;
        let normal = p.compute_normal(intersection_point);
        let mut intensity = 0.0;
        for light in scene.iter_lights() {
            // Calculate if we are even visible by the light (aka shadows)
            // if we are not (we = intersection_point) visible by the light,
            // continue to the next light
            if is_visible_by_light(intersection_point, light, scene.iter_primitives()) {
                intensity += compute_light(
                    -closest * dest,
                    p.get_shininess(),
                    intersection_point,
                    normal,
                    light,
                );
            }
        }
        let color = p.color().unwrap().apply_light_intensity(intensity);
        if current_reflection == MAX_REFLCT {
            return Some(color);
        }
        let refl_factor = p.get_reflectiveness();
        let refl_ray = 2.0 * normal * normal.dot(-closest * dest) + closest * dest;
        return Some(
            color.apply_light_intensity(1.0 - refl_factor)
                + trace_ray(
                    intersection_point,
                    refl_ray,
                    t_min,
                    t_max,
                    scene,
                    current_reflection + 1,
                )
                .unwrap_or(BACKGROUND_COLOR)
                .apply_light_intensity(refl_factor),
        );
    }
    None
}

pub fn intersect_ray(source: Vec3, dest: Vec3, primitive: &Primitive) -> Option<(f32, f32)> {
    match primitive {
        Primitive::Sphere(center, radius, _, _, _) => {
            let co = source - *center;
            let a = dest.dot(dest);
            let b = 2.0 * co.dot(dest);
            let c = co.dot(co) - radius.powi(2);
            return quadratic(a, b, c);
        }
        #[allow(unreachable_patterns)]
        _ => todo!("intersect_ray is not implemented for this primitve"),
    }
}
