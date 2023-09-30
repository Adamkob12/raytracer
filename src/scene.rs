use crate::{LightSource, Primitive};

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
        match primitive {
            Primitive::Sphere(_center, _radius, _color, _s, refl) => {
                if refl < 0.0 || refl > 1.0 {
                    panic!("Reflectiveness value should be between 0.0 and 1.0 inclusive");
                }
            }
            #[allow(unreachable_patterns)]
            _ => todo!("implement unimplemented primitive case for `get_shininess`"),
        }
        self.primitives.push(primitive);
    }

    pub fn insert_light_source(&mut self, light: LightSource) {
        self.lights.push(light);
        let mut light_intensity_total = 0.0;
        for light in self.iter_lights() {
            match light {
                LightSource::Point(intensity, _) => light_intensity_total += *intensity,
                LightSource::Directional(intensity, _) => light_intensity_total += *intensity,
                LightSource::Ambient(intensity) => light_intensity_total += *intensity,
            }
        }
        if light_intensity_total > 1.0 {
            println!("The total light intensity is over 1.0, some points may be overexposed");
        }
    }

    pub fn new() -> Self {
        Scene {
            primitives: vec![],
            lights: vec![],
        }
    }
}
