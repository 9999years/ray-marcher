use std::iter::Sum;

use num::Float;

use crate::camera::{Render, Viewport};
use crate::distance::Geometry;
use crate::light::{Light, Material};

pub struct RenderGeometry<'a, T>
where
    T: Float + Sum + Default,
{
    mat: &'a Material<T>,
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

pub struct Scene<'a, T, C>
where
    T: Float + Sum + Default,
    C: Default,
{
    geometry: Vec<RenderGeometry<'a, T>>,
    materials: Vec<Material<T>>,
    lights: Vec<Light<T, C>>,
    cameras: Vec<Viewport<T>>,
    renders: Vec<Render<'a, T>>,
}
