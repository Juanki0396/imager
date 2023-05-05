#[derive(Debug)]
pub struct Dims {
    width: usize,
    height: usize,
}

impl Dims {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}
