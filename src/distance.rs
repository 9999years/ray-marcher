extern crate vek::{vec};
use vek::quaternion::Quaternion;
use vek::vec::{Vec2, Vec3, Vec4};

type DistanceEstimator<T> = Fn(Vec3<T>) -> T;

fn julia<T>(pos: Vec3<T>, c: Quaternion<T>, i32 iterations) -> T {
    // keep one component fixed to view a 3d "slice" of the 4d fractal
    let mut q = Quaternion::from(Vec4::from(pos));
    // q', running derviative of q
    let mut qp == Quaternion::from_xyzw(1, 0, 0, 0);

    for i in (0..iterations) {
        qp = 2 * (q * qp);
        q = q * q + c;
        if q.magnitude_squared() > 16 {
            break;
        }
    }

    //            |q| log |q|
    // distance = ───────────
    //               2 |q′|
    let mag_q = q.magnitude();
    mag_q * mag_q.ln() / (2 * qp.magnitude())
}
