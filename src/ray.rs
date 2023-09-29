use crate::{quadratic, scene::*, Color, Vec3, BACKGROUND_COLOR, INF};

pub fn trace_ray(source: Vec3, dest: Vec3, t_min: i32, t_max: i32, scene: &Scene) -> &Color {
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
    if let Some(p) = closest_prim {
        return p.color().unwrap();
    }
    &BACKGROUND_COLOR
}

pub fn intersect_ray(source: Vec3, dest: Vec3, primitive: &Primitive) -> Option<(f32, f32)> {
    match primitive {
        Primitive::Sphere(center, radius, _) => {
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
