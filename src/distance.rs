use std::iter::{Sum};

extern crate vek;
use self::vek::{Vec3, Vec4, Quaternion};

extern crate num;
use self::num::{Num, Float, One};

pub type DistanceEstimator<T> = Fn(Vec3<T>) -> T;

fn julia<T: Float + Sum>(pos: Vec3<T>, c: Quaternion<T>, iterations: i32) -> T {
    // keep one component fixed to view a 3d "slice" of the 4d fractal
    let mut q = Quaternion::from(Vec4::from(pos));
    // q', running derviative of q
    let mut qp: Quaternion<T> = Quaternion::from_xyzw(1, 0, 0, 0);

    for i in 0..iterations {
        qp = (q * qp) * 2;
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
    // sample size for estimating normals
    sample_size: T,
    de: DistanceEstimator<T>,
}

impl <T: Num> Estimator<T> {
    fn estimate(&self, pos: Vec3<T>, rot: Vec3<T>) -> Option<Vec3<T>> {
        let mut total_dist: T = 0;
        for i in 0..self.max_steps {
            let measure_pos = pos + rot * total_dist;
            let dist = self.de(measure_pos);
            total_dist += dist;

            if dist <= self.min_dist {
                Some(measure_pos)
            } else if total_dist >= self.max_dist || total_dist.is_infinite() {
                None()
            }
        }
    }

    fn normal(&self, pos: Vec3<T>) -> Vec3<T>
        where T: Float + Sum {
        let x = Vec3::new(self.sample_size, 0, 0);
        let y = Vec3::new(0, self.sample_size, 0);
        let z = Vec3::new(0, 0, self.sample_size);
        Vec3::new(
            self.de(pos + x) - self.de(pos - x),
            self.de(pos + y) - self.de(pos - y),
            self.de(pos + z) - self.de(pos - z)
        ).normalized()
    }

}

