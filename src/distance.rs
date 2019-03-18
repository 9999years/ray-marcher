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

struct Estimator<T> {
    max_steps: i32,
    min_dist: T,
    max_dist: T,
    de: DistanceEstimator<T>,
}

impl Estimator<T> {
    fn estimate<T>(&self, pos: Vec3<T>, rot: Vec3<T>) -> Option<Vec3<T>> {
        let mut total_dist: T = 0;
        for i in 0..self.max_steps {
            let measure_pos = pos + rot * total_dist;
            let dist = de(measure_pos);
            total_dist += dist;

            if dist <= self.min_dist {
                Some(measure_pos)
            } else if total_dist >= self.max_dist || total_dist.is_infinite() {
                None()
            }
        }
    }
}

