use std::iter::Sum;
use std::ops::Mul;

use num::Float;
use palette::{Alpha, Blend, Component, ComponentWise};
use vek::Vec3;

use crate::camera::Camera;

pub struct BlinnPhong<T, C> {
    camera: Camera<T>,
    lights: Vec<Light<T, C>>,
}

pub struct Material<T> {
    specular: T,
    diffuse: T,
    ambient: T,

    // α
    shininess: T,
}

/// C being the color type
pub struct Light<T, C> {
    // L
    rot: Vec3<T>,

    // i_s, i_d, i_a
    // col(or)
    col: Material<C>,
    // k_s, k_d, k_a in a material
}

impl<T, C> BlinnPhong<T, Alpha<C, T>>
where
    T: Float + Sum + Component,
    C: Default + Copy + Blend<Color = C> + ComponentWise<Scalar = T> + Mul<T, Output = C>,
{
    /// lighting for a given normal and material
    /// Possible optimization: a cache
    ///
    /// Blinn-Phong is calculated with:
    ///     I_p: lighting at a surface point
    ///     N: surface normal
    ///     V: vector towards camera
    ///     Per-material:
    ///         k_s: specular refl. constant
    ///         k_d: diffuse refl. constant
    ///         k_a: ambient refl. constant
    ///         α: shininess constant
    ///     Per-light m:
    ///         L: vector from surface towards light source
    ///         H: half-way vector, L + V normalized
    ///         i_s: specular light intensity constant
    ///         i_d: diffuse light intensity constant
    ///         i_a: ambient light intensity constant
    ///     I_p = ∑_lights (k_a i_a
    ///                   + k_d i_d (L ⋅ N)
    ///                   + k_s i_s (N ⋅ H)^α)
    pub fn lighting(&self, normal: Vec3<T>, mat: Material<T>) -> Alpha<C, T> {
        let mut color: Alpha<C, T> = Alpha::default();
        for light in &self.lights {
            let halfway = (self.camera.rot + light.rot).normalized();
            // add the new light to the total light so far
            // note: light.ambient, light.diffuse, and light.specular
            // can be completely different colors
            color = color
                .plus(light.col.ambient * mat.ambient)
                .plus(light.col.diffuse * mat.diffuse * light.rot.dot(normal))
                .plus(light.col.specular * mat.specular * normal.dot(halfway).powf(mat.shininess));
        }
        color
    }
}
