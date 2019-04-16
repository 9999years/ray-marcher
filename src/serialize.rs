use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;

use crate::camera::{Viewport};

#[derive(Serialize, Deserialize)]
struct Render {
    camera: String,
    width: usize,
}

#[derive(Serialize, Deserialize)]
struct Camera<T> {
    facing: Vec3<T>,
    right: Vec3<T>,
    pos: Vec3<T>,
    focal_len: T,
    width: T,
    height: T,
}

impl Into<Viewport<T>> for Camera<T> {
    fn into(self) -> Viewport<T> {
        Viewport<T> {
            cam = Ray<T>::new(self.pos, self.facing),
            right = self.right,
            size = Extent2<T>::new(self.width, self.height),
            focal_len = self.focal_len,
        }
    }
}

enum Geometry<T> {
    distance: Estimator<T>,
}

impl TryFrom<HashMap<>> for Geometry<T> {
}
