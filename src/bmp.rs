use std::fs::File;
use std::io::{self, Write};

pub fn write_bmp_file(file_path: &str, buffer: &[u32], width: usize, height: usize) -> io::Result<()> {
    let mut file = File::create(file_path)?;

    // Encabezado BMP
    let file_size = 14 + 40 + (width * height * 3) as u32;
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
        (width & 0xFF) as u8,
        ((width >> 8) & 0xFF) as u8,
        ((width >> 16) & 0xFF) as u8,
        ((width >> 24) & 0xFF) as u8,
        (height & 0xFF) as u8,
        ((height >> 8) & 0xFF) as u8,
        ((height >> 16) & 0xFF) as u8,
        ((height >> 24) & 0xFF) as u8,
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
    for y in (0..height).rev() {  // Invertir las filas
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let r = (pixel >> 16) & 0xFF;
            let g = (pixel >> 8) & 0xFF;
            let b = pixel & 0xFF;
            file.write_all(&[b as u8, g as u8, r as u8])?;
        }
        // Relleno para cada fila para que sea múltiplo de 4 bytes
        let padding = (4 - (width * 3) % 4) % 4;
        file.write_all(&vec![0; padding])?;
    }

    Ok(())
}
