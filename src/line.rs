use crate::framebuffer::Framebuffer;
use crate::color::Color;

/// Dibuja una línea en el framebuffer utilizando el algoritmo de Bresenham
pub fn draw_line(fb: &mut Framebuffer, x0: isize, y0: isize, x1: isize, y1: isize, color: &Color) {
    let color_u32 = color.to_u32();
    let mut x0 = x0;
    let mut y0 = y0;
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        fb.set_pixel(x0 as usize, y0 as usize, color_u32);
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = err * 2;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}
