use std::iter::Sum;

use num::Float;

use crate::camera::{Render, Viewport};
use crate::distance::Geometry;
use crate::light::{Light, Material};

pub struct RenderGeometry<T>
where
    T: Float + Sum + Default,
{
    mat: Material<T>,
    geom: Geometry<T>,
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
    geometry: Vec<RenderGeometry<T>>,
    lights: Vec<Light<T, C>>,
    renders: Vec<Render<T>>,
}
