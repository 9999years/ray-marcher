extern crate vek::{vec};
use vek::quaternion::Quaternion;
use vek::vec::{Vec2, Vec3, Vec4};

use distance::{DistanceEstimator};


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
