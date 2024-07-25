mod framebuffer;
mod bmp;
mod color;
mod line;

use framebuffer::Framebuffer;
use color::Color;
use line::draw_line;

// Agregar esta función para rellenar el polígono
fn fill_polygon(fb: &mut Framebuffer, points: &[(i32, i32)], color: &Color) {
    let mut nodes = Vec::new();
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    // Encuentra el mínimo y el máximo de Y
    for point in points {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    // Escanear línea por línea desde min_y hasta max_y
    for y in min_y..=max_y {
        nodes.clear();

        // Construir una lista de nodos
        let mut j = points.len() - 1;
        for i in 0..points.len() {
            if (points[i].1 < y && points[j].1 >= y) || (points[j].1 < y && points[i].1 >= y) {
                let x = points[i].0 + (y - points[i].1) * (points[j].0 - points[i].0) / (points[j].1 - points[i].1);
                nodes.push(x);
            }
            j = i;
        }

        // Ordenar nodos
        nodes.sort();

        // Rellenar entre pares de nodos
        for n in (0..nodes.len()).step_by(2) {
            if n + 1 < nodes.len() {
                for x in nodes[n]..=nodes[n + 1] {
                    fb.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }
}


fn main() {
    let mut fb = Framebuffer::new(800, 600);

    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    let points2 = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut fb, &points1, &Color::YELLOW);

    // Dibujar la orilla del primer polígono con color blanco
    for i in 0..points1.len() {
        let (x0, y0) = points1[i];
        let (x1, y1) = points1[(i + 1) % points1.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut fb, &points2, &Color::BLUE);

    // Dibujar la orilla del segundo polígono con color blanco
    for i in 0..points2.len() {
        let (x0, y0) = points2[i];
        let (x1, y1) = points2[(i + 1) % points2.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    fb.save_as_bmp("polygon2.bmp");
}