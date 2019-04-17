use std::iter::Sum;

use num::Float;

use crate::distance::{Estimator, Geometry};
use crate::light::{BlinnPhong, Material, Light};
use crate::camera::{Render, Viewport};

pub struct RenderGeometry<'a, T, E>
where
    T: Float + Sum + Default,
    E: Estimator<T>,
{
    mat: &'a Material<T>,
    geom: Geometry<T, E>,
}

pub struct Scene<'a, T, C, E>
where
    T: Float + Sum + Default,
    C: Default,
    E: Estimator<T>,
{
    geometry: Vec<RenderGeometry<'a, T, E>>,
    materials: Vec<Material<T>>,
    lights: Vec<Light<T, C>>,
    cameras: Vec<Viewport<T>>,
    renders: Vec<Render<'a, T>>,
}
