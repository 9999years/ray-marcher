use std::iter::Sum;
use std::ops::{Div, Range};

use serde::{Serialize, Deserialize};
use num::{Float, Num};
use vek::{Vec2, Vec3, Ray, Extent2, Lerp};

/// if `val` is in `domain`, put it in a proportional spot in `codomain`
fn scale<T>(val: T, domain: Range<T>, codomain: Range<T>) -> T
where
    T: Num + Copy + Lerp,
{
    let scale = (val - domain.start) / (domain.end - domain.start);
    T::lerp_unclamped(codomain.start, codomain.end, scale)
}

#[derive(Serialize, Deserialize, Default)]
pub struct Viewport<T: Default> {
    /// position and facing of the center of the viewport
    cam: Ray<T>,
    right: Vec3<T>,
    /// the viewport's size in world units
    size: Extent2<T>,
    /// the viewport's focal length; higher means more zoomed in
    pub focal_len: T,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Render<'a, T: Default> {
    width: usize,
    pub view: &'a Viewport<T>,
}

impl<'a, T> Viewport<'a, T>
where
    T: Default,
{
    pub fn up(&self) -> Vec3<T>
    where
        T: Num + Copy,
    {
        self.cam.direction.cross(self.right)
    }

    pub fn aspect(&self) -> T
    where
        T: Div,
    {
        self.width / self.height
    }

    /// location.x and .y are fractions from 0 to 1 of how far left/bottom in the viewport the
    /// ray should originate at
    /// Returns: position, orientation of the ray
    pub fn ray(&self, location: Vec2<T>) -> (Vec3<T>, Vec3<T>)
    where
        T: Float + Sum,
    {
        // w and h scaled to -0.5, 0.5
        let width = location.x - T::from(0.5).unwrap();
        let height = location.y - T::from(0.5).unwrap();

        // vectors pointing from the center of the viewport to the width coord and height
        // coord on the viewport
        let ray_on_viewport = self.right * (width * self.width)
            + self.right.cross(self.cam.direction) * (height * self.height);

        // vector from the center of the viewport to the origin of the rays
        let camera = self.cam.direction * -self.focal_len;

        // ray orientation; normalized version of vector from origin of rays to viewport
        // coords
        let ray_rot = (ray_on_viewport - camera).normalized();

        (self.cam.origin + ray_on_viewport, ray_rot)
    }
}

impl <T> Render<T> {
    pub fn aspect(&self) -> T
    where
        T: Num + Copy,
    {
        self.view.aspect()
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize
    where
        T: Float + Into<usize>
    {
        (T::from(self.width).unwrap() / self.aspect()).into()
    }
}
