extern crate num;
use self::num::Float;

extern crate palette;
use self::palette::Pixel;

use self::distance::Estimator;

struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ImageData {
    fn new(width: usize, height: usize) -> Self {
        ImageData {
            width: width,
            height: height,
            data: Vec::with_capacity(width * height),
        }
    }

    fn set<C>(&mut self, x: usize, y: usize, color: C)
    where
        C: Pixel<u8>,
    {
        let idx = y * self.width + x;
        let color_slice = &[color];
        let val = Pixel::into_raw_slice(color_slice);
        &self.data[idx..idx + val.len()].copy_from_slice(val);
    }

    fn <T> render<C>(&mut self, scene: Scene<T, C>)
    where
        T: Float,
    {
    }
}
