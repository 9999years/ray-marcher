use std::ops::{Div, Range};

extern crate vek;
use self::vek::{Vec2, Vec3};

extern crate num;
use self::num::{Float, Num};
use std::iter::Sum;

/// if `val` is in `domain`, put it in a proportional spot in `codomain`
fn scale<T>(val: T, domain: Range<T>, codomain: Range<T>) -> T
where
    T: Num + Copy,
{
    let scale = (val - domain.start) / (domain.end - domain.start);
    scale * (codomain.end - codomain.start) + codomain.start
}

pub struct Camera<T> {
    pub rot: Vec3<T>,
    pub pos: Vec3<T>,
    pub focal_len: T,
}

pub struct Viewport<T> {
    width: T,
    height: T,
    /// a right-angle with the camera to define orientation; normalized
    right: Vec3<T>,
    /// position and facing of the center of the viewport
    camera: Camera<T>,
}

impl<T: Num + Copy> Viewport<T> {
    fn aspect(&self) -> T
    where
        T: Div,
    {
        self.width / self.height
    }

    /// location.x and .y are fractions from 0 to 1 of how far left/bottom in the viewport the
    /// ray should originate at
    /// Returns: position, orientation of the ray
    fn ray(&self, location: Vec2<T>) -> (Vec3<T>, Vec3<T>)
    where
        T: Float + Sum,
    {
        // w and h scaled to -0.5, 0.5
        let width = location.x - T::from(0.5).unwrap();
        let height = location.y - T::from(0.5).unwrap();

        // vectors pointing from the center of the viewport to the width coord and height
        // coord on the viewport
        let ray_on_viewport = self.right * (width * self.width)
            + self.right.cross(self.camera.rot) * (height * self.height);

        // vector from the center of the viewport to the origin of the rays
        let camera = self.camera.rot * -self.camera.focal_len;

        // ray orientation; normalized version of vector from origin of rays to viewport
        // coords
        let ray_rot = (ray_on_viewport - camera).normalized();

        (self.camera.pos + ray_on_viewport, ray_rot)
    }
}
