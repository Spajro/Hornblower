pub struct Buffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

pub trait Paintable {
    fn paint(self, buffer: &mut Buffer);
}

impl Buffer {
    pub fn new(width: usize, height: usize) -> Self {
        Buffer {
            width,
            height,
            buffer: vec![0; width * height],
        }
    }
}