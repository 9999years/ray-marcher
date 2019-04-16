use std::convert::TryFrom;
use std::iter::Sum;

use serde::{Deserialize, Serialize};
use vek::{Extent2, Ray, Vec3, Quaternion};
use num::Float;

use crate::camera::Viewport;
use crate::distance;
use crate::distance::{Geometry, Estimator};

#[derive(Serialize, Deserialize)]
struct Render {
    camera: String,
    width: usize,
}

#[derive(Serialize, Deserialize)]
struct Camera<T> {
    facing: Vec3<T>,
    right: Vec3<T>,
    pos: Vec3<T>,
    focal_len: T,
    width: T,
    height: T,
}

impl <T> Into<Viewport<T>> for Camera<T>
where
    T: Float + Sum + Default,
{
    fn into(self) -> Viewport<T> {
        Viewport {
            cam: Ray::new(self.pos, self.facing.normalized()),
            right: self.right,
            size: Extent2::new(self.width, self.height),
            focal_len: self.focal_len,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Julia<T> {
    #[serde(alias = "type")]
    type_: String,
    c: Quaternion<T>,
    iterations: usize,
    material: String,
    epsilon: T,
    cutoff: T,
    max_steps: usize,
}

enum EstimatorErr {
    UnknownType(String),
}

impl <T, E> TryFrom<Julia<T>> for Geometry<T, E>
where
    T: Float + Sum,
    E: Estimator<T>,
{
    type Error = EstimatorErr;

    fn try_from(value: Julia<T>) -> Result<Self, Self::Error> {
        if value.type_ != "julia" {
            return Err(EstimatorErr::UnknownType(value.type_));
        }

        Ok(Geometry {
            max_steps: value.max_steps,
            epsilon: value.epsilon,
            cutoff: value.cutoff,
            sample_size: value.epsilon,
            de: distance::Julia {
                c: value.c,
                iterations: value.iterations,
            },
        })
    }
}
