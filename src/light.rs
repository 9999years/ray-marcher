extern crate vek;
use self::vek::{Vec3};

use super::camera::{Camera};

struct BlinnPhong<T, C> {
    camera: Camera<T>,
    lights: Vec<Light<T, C>>,
}

struct MaterialProperties<T> {
     specular: T,
     diffuse: T,
     ambient: T,
}

struct Light<T, C> {
     rot: Vec3<T>,
     color: C,

     shininess: T,
     constants: MaterialProperties<T>,
     intensity: MaterialProperties<T>,
}

