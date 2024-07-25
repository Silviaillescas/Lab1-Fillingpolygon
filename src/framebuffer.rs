use std::fs::File;
use std::io::{self, Write};

mod color {
    #[derive(Clone)]
    pub struct Color {
        pub r: u8,
        pub g: u8,
        pub b: u8,
    }

    impl Color {
        pub const BLACK: Color = Color { r: 0, g: 0, b: 0 };
        pub const WHITE: Color = Color { r: 255, g: 255, b: 255 };
    }
}

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

    // Método para guardar el framebuffer como un archivo BMP
    pub fn save_as_bmp(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;

        // Encabezado BMP
        let file_size = 14 + 40 + (self.width * self.height * 3) as u32;
        let header = [
            0x42, 0x4D, // Firma 'BM'
            (file_size & 0xFF) as u8,
            ((file_size >> 8) & 0xFF) as u8,
            ((file_size >> 16) & 0xFF) as u8,
            ((file_size >> 24) & 0xFF) as u8,
            0, 0, // Reservado
            0, 0, // Reservado
            54, 0, 0, 0, // Desplazamiento de archivo a PixelArray
        ];
        file.write_all(&header)?;

        // Encabezado DIB
        let dib_header = [
            40, 0, 0, 0, // Tamaño del encabezado DIB
            (self.width & 0xFF) as u8,
            ((self.width >> 8) & 0xFF) as u8,
            ((self.width >> 16) & 0xFF) as u8,
            ((self.width >> 24) & 0xFF) as u8,
            (self.height & 0xFF) as u8,
            ((self.height >> 8) & 0xFF) as u8,
            ((self.height >> 16) & 0xFF) as u8,
            ((self.height >> 24) & 0xFF) as u8,
            1, 0, // Número de planos de color
            24, 0, // Bits por píxel
            0, 0, 0, 0, // Compresión
            0, 0, 0, 0, // Tamaño de la imagen (puede ser 0 para BI_RGB)
            0, 0, 0, 0, // Resolución horizontal (píxeles por metro)
            0, 0, 0, 0, // Resolución vertical (píxeles por metro)
            0, 0, 0, 0, // Colores en la tabla de colores
            0, 0, 0, 0, // Recuento de colores importantes
        ];
        file.write_all(&dib_header)?;

        // Datos de los píxeles
        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.get_pixel(x, y);
                file.write_all(&[pixel.b, pixel.g, pixel.r])?;
            }
            // Relleno para cada fila para que sea múltiplo de 4 bytes
            let padding = (4 - (self.width * 3) % 4) % 4;
            file.write_all(&vec![0; padding])?;
        }

        Ok(())
    }
}
