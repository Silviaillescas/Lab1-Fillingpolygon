use crate::color::Color;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::BLACK; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            self.pixels[y * self.width + x] = color.clone();
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> &Color {
        &self.pixels[y * self.width + x]
    }

    pub fn clear(&mut self, color: Color) {
        self.pixels.iter_mut().for_each(|pixel| *pixel = color.clone());
    }
}

