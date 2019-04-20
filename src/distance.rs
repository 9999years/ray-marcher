use std::iter::Sum;

use num::Float;
use vek::{Quaternion, Vec3, Vec4};

pub trait Estimator<T>: Sized
where
    T: Float + Sum,
{
    fn estimate(&self, pos: Vec3<T>) -> T;
}

pub struct Geometry<T, E>
where
    T: Float + Sum,
    E: Estimator<T>,
{
    pub max_steps: usize,
    /// values smaller than ε are considered part of the geometry
    pub epsilon: T,
    /// rays which exceed this distance are assumed to be lost
    pub cutoff: T,
    /// sample size for estimating normals
    pub sample_size: T,
    pub de: E,
}

impl<T, E> Geometry<T, E>
where
    T: Float + Sum,
    E: Estimator<T>,
{
    pub fn estimate(&self, pos: Vec3<T>, rot: Vec3<T>) -> Option<Vec3<T>> {
        let mut total_dist = T::from(0).unwrap();
        for _ in 0..self.max_steps {
            let measure_pos = pos + rot * total_dist;
            let dist = self.de.estimate(measure_pos);
            total_dist = total_dist + dist;

            if dist <= self.epsilon {
                return Some(measure_pos);
            } else if total_dist >= self.cutoff || total_dist.is_infinite() {
                return None;
            }
        }
        None
    }

    pub fn normal(&self, pos: Vec3<T>) -> Vec3<T>
    where
        T: Float + Sum,
    {
        let zero = T::zero();
        let x = Vec3::new(self.sample_size, zero, zero);
        let y = Vec3::new(zero, self.sample_size, zero);
        let z = Vec3::new(zero, zero, self.sample_size);
        Vec3::new(
            self.de.estimate(pos + x) - self.de.estimate(pos - x),
            self.de.estimate(pos + y) - self.de.estimate(pos - y),
            self.de.estimate(pos + z) - self.de.estimate(pos - z),
        )
        .normalized()
    }
}

pub struct Julia<T: Float + Sum> {
    c: Quaternion<T>,
    iterations: usize,
}

impl<T> Julia<T>
where
    T: Float + Sum,
{
    pub fn new(c: Quaternion<T>, iterations: usize) -> Self {
        Self { c, iterations }
    }
}

impl<T> Estimator<T> for Julia<T>
where
    T: Float + Sum,
{
    fn estimate(&self, pos: Vec3<T>) -> T {
        // keep one component fixed to view a 3d "slice" of the 4d fractal
        let mut q = Quaternion::from(Vec4::from(pos));
        // q', running derviative of q
        let mut qp: Quaternion<T> = Quaternion::from(Vec4::right());

        let t2 = T::from(2).unwrap();
        let t16 = T::from(16).unwrap();

        for _ in 0..self.iterations {
            qp = (q * qp) * t2;
            q = q * q + self.c;
            if q.magnitude_squared() > t16 {
                break;
            }
        }

        //            |q| log |q|
        // distance = ───────────
        //               2 |q′|
        let mag_q = q.magnitude();
        mag_q * mag_q.ln() / (t2 * qp.magnitude())
    }
}
