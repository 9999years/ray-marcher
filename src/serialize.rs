use std::collections::HashMap;
use std::iter::Sum;

use num::Float;
use serde::{Deserialize, Serialize};
use vek::{Extent2, Quaternion, Ray, Vec3};

use crate::camera::Viewport;
use crate::distance;
use crate::distance::Estimator;
use crate::light::{Light, Material};
use crate::render;

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

impl<T> Into<Viewport<T>> for Camera<T>
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
struct EstimatorBase<T> {
    material: String,
    epsilon: T,
    cutoff: T,
    max_steps: usize,
}

#[derive(Serialize, Deserialize)]
struct Julia<T> {
    c: Quaternion<T>,
    iterations: usize,

    #[serde(flatten)]
    est: EstimatorBase<T>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
enum Geometry<T> {
    Julia(Julia<T>),
}

impl <T> Into<distance::Geometry<T, distance::Julia<T>>> for Geometry<T>
where
    T: Float + Sum,
{
    fn into(self) -> distance::Geometry<T, distance::Julia<T>> {
        match self {
            Geometry::Julia(julia) => distance::Geometry {
                max_steps: julia.est.max_steps,
                epsilon: julia.est.epsilon,
                cutoff: julia.est.cutoff,
                sample_size: julia.est.epsilon,
                de: distance::Julia::new(julia.c, julia.iterations),
            },
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct Scene<T, C>
where
    T: Float + Sum + Default,
    C: Default,
{
    geometry: Vec<Geometry<T>>,
    materials: HashMap<String, Material<T>>,
    lights: Vec<Light<T, C>>,
    cameras: HashMap<String, Camera<T>>,
    renders: Vec<Render>,
}

impl <'a, T, C, E> Into<render::Scene<'a, T, C, E>> for Scene<T, C>
where
    T: Float + Sum + Default,
    C: Default,
    E: Estimator<T>,
{
    fn into(self) -> render::Scene<'a, T, C, E> {
        unimplemented!();
    }
}
