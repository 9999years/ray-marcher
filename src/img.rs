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

    fn set(&self, x: usize, y: usize, val: &[u8]) {
        let idx = y * self.width + x;
        &self.data[idx..idx + val.len()].copy_from_slice(val);
    }
}
