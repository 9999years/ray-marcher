use palette::{Pixel, Srgba};

// 8-bit rgba image data
struct ImageData {
    width: usize,
    height: usize,
    data: Vec<u8>,
}

impl ImageData {
    /// number of channels per pixel; rgba
    const CHANNELS: usize = 4;

    fn new(width: usize, height: usize) -> Self {
        ImageData {
            width,
            height,
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

    fn set_inx<C>(&mut self, inx: usize, color: C)
    where
        C: Pixel<u8>
    {
        let color_slice = &[color];
        let val = Pixel::into_raw_slice(color_slice);
        &self.data[inx..inx + val.len()].copy_from_slice(val);
    }

    /// returns an iterator giving a usize for the start of each pixel in the image data
    fn indexes(&self) -> impl Iterator<Item = usize> {
        (0..self.data.len()).step_by(Self::CHANNELS)
    }

    fn render_fn<F, C>(&mut self, func: F)
    where
        F: Fn(usize, usize) -> Srgba<C>,
    {
        for inx in self.indexes() {
            let (x, y) = self.inx_to_coords(inx);
            self.set_inx(inx, func(x, y));
        }
    }
}
