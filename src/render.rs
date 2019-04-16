use std::collections::HashMap;

use crate::distance::Geometry;
use crate::light::{BlinnPhong, Material};
use crate::camera::{Render, Viewport};

use num::Float;

#[derive(Default)]
pub struct Scene<'a, T, C>
where
    T: Float,
    C: Default,
{
    // TODO geometry
    materials: HashMap<&'a str, Material<T>>,
    lights: Vec<Light<T>>,
    cameras: HashMap<&'a str, Viewport<T>>,
    renders: Vec<Render<T>>,
}
