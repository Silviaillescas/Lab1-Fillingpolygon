use std::fs::File;
use std::io::Write;
use crate::framebuffer::Framebuffer;

impl Framebuffer {
    pub fn export_as_bmp(&self, file_path: &str) {
        let mut file = File::create(file_path).expect("No se pudo crear el archivo");

        let bmp_file_header = generate_bmp_file_header(self.width, self.height);
        let bmp_info_header = generate_bmp_info_header(self.width, self.height);

        file.write_all(&bmp_file_header).expect("No se pudo escribir la cabecera del archivo BMP");
        file.write_all(&bmp_info_header).expect("No se pudo escribir la cabecera de información del BMP");

        for y in (0..self.height).rev() {
            for x in 0..self.width {
                let pixel = self.get_pixel(x, y);
                file.write_all(&[pixel.b, pixel.g, pixel.r]).expect("No se pudo escribir los datos del píxel");
            }
        }
    }
}

fn generate_bmp_file_header(width: usize, height: usize) -> [u8; 14] {
    let file_size = 14 + 40 + (width * height * 3) as u32;
    [
        0x42, 0x4D, // Número mágico para BMP
        (file_size & 0xFF) as u8, ((file_size >> 8) & 0xFF) as u8, ((file_size >> 16) & 0xFF) as u8, ((file_size >> 24) & 0xFF) as u8, // Tamaño del archivo BMP
        0x00, 0x00, 0x00, 0x00, // Bytes reservados sin uso
        0x36, 0x00, 0x00, 0x00, // Desplazamiento al inicio de los datos de píxeles
    ]
}

fn generate_bmp_info_header(width: usize, height: usize) -> [u8; 40] {
    [
        0x28, 0x00, 0x00, 0x00, // Tamaño de esta cabecera (40 bytes)
        (width & 0xFF) as u8, ((width >> 8) & 0xFF) as u8, ((width >> 16) & 0xFF) as u8, ((width >> 24) & 0xFF) as u8, // Ancho de la imagen
        (height & 0xFF) as u8, ((height >> 8) & 0xFF) as u8, ((height >> 16) & 0xFF) as u8, ((height >> 24) & 0xFF) as u8, // Alto de la imagen
        0x01, 0x00, // Número de planos de color (debe ser 1)
        0x18, 0x00, // Bits por píxel (24 para RGB)
        0x00, 0x00, 0x00, 0x00, // Sin compresión
        0x00, 0x00, 0x00, 0x00, // Tamaño de la imagen (puede ser 0 si no hay compresión)
        0x13, 0x0B, 0x00, 0x00, // Resolución horizontal (píxeles por metro)
        0x13, 0x0B, 0x00, 0x00, // Resolución vertical (píxeles por metro)
        0x00, 0x00, 0x00, 0x00, // Número de colores en la paleta
        0x00, 0x00, 0x00, 0x00, // Colores importantes (0 significa que todos son importantes)
    ]
}