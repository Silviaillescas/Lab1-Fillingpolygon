mod framebuffer;
mod bmp;
mod color;
mod line;

use framebuffer::Framebuffer;
use color::Color;
use line::draw_line;

/// Rellena un polígono en el framebuffer
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

/// Desplaza las coordenadas del polígono por dx en x y dy en y
fn move_polygon(points: &[(i32, i32)], dx: i32, dy: i32) -> Vec<(i32, i32)> {
    points.iter().map(|&(x, y)| (x + dx, y + dy)).collect()
}

fn main() {
    let mut fb = Framebuffer::new(800, 600);

    let points = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    // Mueve el polígono a una nueva posición (ejemplo: 50 píxeles a la derecha y 20 píxeles hacia abajo)
    let moved_points = move_polygon(&points, 50, 20);

    // Rellenar el polígono movido
    fill_polygon(&mut fb, &moved_points, &Color::YELLOW);

    // Dibujar la orilla del polígono movido
    for i in 0..moved_points.len() {
        let (x0, y0) = moved_points[i];
        let (x1, y1) = moved_points[(i + 1) % moved_points.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    fb.export_as_bmp("polygon1.bmp");
}
