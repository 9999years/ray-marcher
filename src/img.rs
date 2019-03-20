extern crate palette;
use self::palette::Pixel;

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
        where C: Pixel<u8> {
        let idx = y * self.width + x;
        let val = Pixel::into_raw_slice(&[color]);
        &self.data[idx..idx + val.len()].copy_from_slice(val);
    }
}
