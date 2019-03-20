use std::iter::Sum;

extern crate vek;
use self::vek::Vec3;

extern crate num;
use self::num::Float;

extern crate palette;
use self::palette::Mix;

use super::camera::Camera;

struct BlinnPhong<T, C> {
    camera: Camera<T>,
    lights: Vec<Light<T, C>>,
}

struct MaterialProperties<T> {
    specular: T,
    diffuse: T,
    ambient: T,
}

/// C being the color type
struct Light<T, C> {
    rot: Vec3<T>,
    color: C,

    shininess: T,
    constants: MaterialProperties<T>,
    intensity: MaterialProperties<T>,
}

impl<T, C> BlinnPhong<T, C>
where
    T: Float,
    C: Default + Mix<Scalar = T>,
{
    pub fn lighting(&self, normal: Vec3<T>) -> C
    where
        T: Sum,
    {
        let cam = self.camera.rot;
        let mut color = C::default();
        for light in &self.lights {
            let halfway = (cam + light.rot).normalized();
            color = color.mix(
                &light.color,
                light.rot.dot(normal) * light.intensity.diffuse
                    + normal.dot(halfway).powf(light.shininess) * light.intensity.specular,
            );
        }
        color
    }
}
