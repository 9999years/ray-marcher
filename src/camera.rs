extern crate vek;
use vek::vec::{Vec2, Vec3};

/// if `val` is in `domain`, put it in a proportional spot in `codomain`
fn scale<T>(val: T, domain: Range<T>, codomain: Range<T>) -> T {
    let scale = (val - domain.start) / (domain.end - domain.start);
    scale * (codomain.end - codomain.start) + codomain.start
}

struct Camera<T> {
    rot: Vec3<T>,
    pos: Vec3<T>,
    focal_len: T,
}

struct Viewport<T> {
    width: T,
    height: T,
    /// a right-angle with the camera to define orientation; normalized
    right: Vec3<T>,
    /// position and facing of the center of the viewport
    camera: Camera<T>,
}

impl Viewport<T> {
    fn aspect(&self) -> T {
        self.width / self.height
    }

    /// location.x and .y are fractions from 0 to 1 of how far left/bottom in the viewport the
    /// ray should originate at
    /// Returns: position, orientation of the ray
    fn ray(&self, location: Vec2<T>) -> (Vec3<T>, Vec3<T>) {
        // w and h scaled to -0.5, 0.5
        let width  = location.x - 0.5;
        let height = location.y - 0.5;

        // vectors pointing from the center of the viewport to the width coord and height
        // coord on the viewport
        let ray_on_viewport =
              width  * self.width  * self.right
            + height * self.height * self.right.cross(self.camera.rot);

        // vector from the center of the viewport to the origin of the rays
        let camera = self.camera.rot * -self.camera.focal_length;


        // ray orientation; normalized version of vector from origin of rays to viewport
        // coords
        let ray_rot = (ray_on_viewport - camera).normalized();

        (self.camera.pos + ray_on_viewport, ray_rot)
    }
}
