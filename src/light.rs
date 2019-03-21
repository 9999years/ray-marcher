use std::iter::Sum;

use vek::Vec3;
use num::Float;
use palette::{Blend, Alpha};

use crate::camera::Camera;

struct BlinnPhong<T, C> {
    camera: Camera<T>,
    lights: Vec<Light<T, C>>,
}

struct Material<T> {
    specular: T,
    diffuse: T,
    ambient: T,

    // α
    shininess: T,
}

/// C being the color type
struct Light<T, C> {
    // L
    rot: Vec3<T>,

    // i_s, i_d, i_a
    intensity: Material<C>,
    // k_s, k_d, k_a in a material
}

impl<T, C> BlinnPhong<T, C>
where
    T: Float,
    C: Default + Blend<Scalar = T>,
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
    pub fn lighting(&self, normal: Vec3<T>, mat: Material<T>) -> C
    where
        T: Sum,
    {
        let mut color = C::default();
        for light in &self.lights {
            let halfway = (self.camera.rot + light.rot).normalized();
            // add the new light to the total light so far
            // note: light.ambient, light.diffuse, and light.specular
            // can be completely different colors
            color = color.screen(light.ambient * mat.ambient)
                .screen(light.diffuse * mat.diffuse * light.rot.dot(normal))
                .screen(light.specular * mat.specular
                        * normal.dot(halfway).powf(light.shininess));
        }
        color
    }
}
