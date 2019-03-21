use num::Float;
use palette::{Pixel, Srgba};

use crate::distance::Estimator;
use crate::render::Scene;

// 8-bit rgba image data
struct ImageData<T> {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl <T> ImageData {
    /// number of channels per pixel; rgba
    const CHANNELS: usize = 4;

    fn new(width: usize, height: usize) -> Self {
        ImageData {
            width: width,
            height: height,
            data: Vec::with_capacity(width * height * Self::CHANNELS),
        }
    }

    fn coords_to_inx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn inx_to_coords(&self, inx: usize) -> (usize, usize) {
        let y = inx / self.width;
        let x = inx % self.width;
        (x, y)
    }

    fn set<C>(&mut self, x: usize, y: usize, color: C)
    where
        C: Pixel<u8>,
    {
        self.set_inx(self.coords_to_inx(x, y), color);
    }

    fn set_inx(&mut self, inx: usize, color: C)
    where
        C: Pixel<u8>
    {
        let color_slice = &[color];
        let val = Pixel::into_raw_slice(color_slice);
        &self.data[idx..idx + val.len()].copy_from_slice(val);
    }

    /// returns an iterator giving a usize for the start of each pixel in the image data
    fn indexes(&self) -> Iterator<Item = usize> {
        (0..self.data.len()).step_by(Self::CHANNELS)
    }

    fn render_fn(&mut self, func: F)
    where
        F: Fn(usize, usize) -> C
        C: Srgba<_>
    {
        for inx in self.indexes() {
            let (x, y) = self.inx_to_coords(inx);
            self.set_inx(inx, func(x, y));
        }
    }

    fn render<T, C>(&mut self, scene: Scene<T, C>)
    where
        T: Float,
    {
    }
}
