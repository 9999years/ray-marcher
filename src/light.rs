extern crate vek::{vec};
use vek::vec::{Vec2, Vec3, Vec4};

use distance::{DistanceEstimator};
use camera::{Camera};

struct BlinnPhong<T> {
    camera: Camera<T>,
    lights: Vec<Light<T>>,
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

