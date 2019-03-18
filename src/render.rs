extern crate vek::{vec};
use vek::quaternion::Quaternion;
use vek::vec::{Vec2, Vec3, Vec4};

use distance::{DistanceEstimator};

struct Camera<T> {
    rot: Vec3<T>,
    pos: Vec3<T>,
}

struct BlinnPhong<T> {
     camera: Vec3<T>,
     pos: Vec3<T>,
}

struct MaterialProperties<T> {
     specular: T,
     diffuse: T,
     ambient: T,
}

struct Light<T> {
     rot: Vec3<T>,
     color: (u8, u8, u8),

     shininess: T,
     constants: MaterialProperties<T>,
     intensity: MaterialProperties<T>,
}


fn normal<T>(pos: Vec3<T>, T sample_size, de: DistanceEstimator<T>) -> Vec3<T> {
    let x = Vec3::new(sample_size, 0, 0);
    let y = Vec3::new(0, sample_size, 0);
    let z = Vec3::new(0, 0, sample_size);
    Vec3::new(
        de(pos + x) - de(pos - x),
        de(pos + y) - de(pos - y),
        de(pos + z) - de(pos - z)
    ).normalize()
}
