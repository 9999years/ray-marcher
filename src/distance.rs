use std::iter::Sum;
use std::ops::Range;

use num::Float;
use vek::{Quaternion, Vec3, Vec4, Clamp};

pub type DistanceEstimator<T> = Fn(Vec3<T>) -> T;

pub struct Estimator<T> {
    max_steps: i32,
    /// values smaller than ε are considered part of the geometry
    epsilon: T,
    /// rays which exceed this distance are assumed to be lost
    cutoff: T,
    // sample size for estimating normals
    sample_size: T,
    de: DistanceEstimator<T>,
}

impl<T: Float> Estimator<T> {
    fn estimate(&self, pos: Vec3<T>, rot: Vec3<T>) -> Option<Vec3<T>> {
        let mut total_dist = T::from(0).unwrap();
        for _ in 0..self.max_steps {
            let measure_pos = pos + rot * total_dist;
            let dist = (self.de)(measure_pos);
            total_dist = total_dist + dist;

            if dist <= self.epsilon {
                return Some(measure_pos);
            } else if total_dist >= self.cutoff || total_dist.is_infinite() {
                return None;
            }
        }
        None
    }

    fn normal(&self, pos: Vec3<T>) -> Vec3<T>
    where
        T: Float + Sum,
    {
        let zero = T::zero();
        let x = Vec3::new(self.sample_size, zero, zero);
        let y = Vec3::new(zero, self.sample_size, zero);
        let z = Vec3::new(zero, zero, self.sample_size);
        Vec3::new(
            (self.de)(pos + x) - (self.de)(pos - x),
            (self.de)(pos + y) - (self.de)(pos - y),
            (self.de)(pos + z) - (self.de)(pos - z),
        )
        .normalized()
    }
}

fn julia<'a, T: Float + Sum>(c: Quaternion<T>, iterations: i32) -> &'a DistanceEstimator<T> {
    |pos| {
        // keep one component fixed to view a 3d "slice" of the 4d fractal
        let mut q = Quaternion::from(Vec4::from(pos));
        // q', running derviative of q
        let mut qp: Quaternion<T> = Quaternion::from(Vec4::right());

        for _ in 0..iterations {
            qp = (q * qp) * T::from(2).unwrap();
            q = q * q + c;
            if q.magnitude_squared() > T::from(16).unwrap() {
                break;
            }
        }

        //            |q| log |q|
        // distance = ───────────
        //               2 |q′|
        let mag_q = q.magnitude();
        mag_q * mag_q.ln() / (T::from(2).unwrap() * qp.magnitude())
    }
}
