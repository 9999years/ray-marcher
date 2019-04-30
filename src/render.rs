use std::iter::Sum;

use num::Float;

use crate::camera::Render;
use crate::distance::Geometry;
use crate::light::{Light, Material};

pub struct RenderGeometry<T>
where
    T: Float + Sum + Default,
{
    pub mat: Material<T>,
    pub geom: Geometry<T>,
}

//impl RenderGeometry<'a, T, E>
//where
//T: Float + Sum + Default,
//E: Estimator<T>,
//{
//pub fn new(mat: &'a Material<T>, geom: Geometry<T, E>) -> Self {

//}
//}

pub struct Scene<T, C>
where
    T: Float + Sum + Default + Clone,
    C: Default + Clone,
{
    pub geometry: Vec<RenderGeometry<T>>,
    pub lights: Vec<Light<T, C>>,
    pub renders: Vec<Render<T>>,
}
