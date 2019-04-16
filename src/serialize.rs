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

enum Geometry<T> {
    distance: Estimator<T>,
}

impl TryFrom<HashMap<>> for Geometry<T> {
}
