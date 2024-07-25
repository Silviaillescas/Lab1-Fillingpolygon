mod framebuffer;
mod bmp;
mod color;
mod line;

use framebuffer::Framebuffer;
use color::Color;
use line::draw_line;

// Agregar esta función para rellenar el polígono
fn fill_polygon(fb: &mut Framebuffer, points: &[(isize, isize)], color: u32) {
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

fn point_in_polygon(point: (isize, isize), polygon: &[(isize, isize)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn fill_polygon_with_hole(fb: &mut Framebuffer, outer: &[(isize, isize)], hole: &[(isize, isize)], color: u32) {
    let mut nodes = Vec::new();
    let mut min_y = outer[0].1;
    let mut max_y = outer[0].1;

    // Encuentra el mínimo y el máximo de Y
    for point in outer {
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
        let mut j = outer.len() - 1;
        for i in 0..outer.len() {
            if (outer[i].1 < y && outer[j].1 >= y) || (outer[j].1 < y && outer[i].1 >= y) {
                let x = outer[i].0 + (y - outer[i].1) * (outer[j].0 - outer[i].0) / (outer[j].1 - outer[i].1);
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
                    if !point_in_polygon((x, y), hole) {
                        fb.set_pixel(x as usize, y as usize, color);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut fb = Framebuffer::new(800, 600, Color::BLACK.to_u32()); // Fondo negro

    let points1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];

    let points2 = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let points3 = [
        (377, 249), (411, 197), (436, 249),
    ];

    let points4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    let points5 = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];

    // Rellenar el primer polígono con color amarillo
    fill_polygon(&mut fb, &points1, Color::YELLOW.to_u32());

    // Dibujar la orilla del primer polígono con color blanco
    for i in 0..points1.len() {
        let (x0, y0) = points1[i];
        let (x1, y1) = points1[(i + 1) % points1.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    // Rellenar el segundo polígono con color azul
    fill_polygon(&mut fb, &points2, Color::BLUE.to_u32());

    // Dibujar la orilla del segundo polígono con color blanco
    for i in 0..points2.len() {
        let (x0, y0) = points2[i];
        let (x1, y1) = points2[(i + 1) % points2.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    // Rellenar el tercer polígono con color rojo
    fill_polygon(&mut fb, &points3, Color::RED.to_u32());

    // Dibujar la orilla del tercer polígono con color blanco
    for i in 0..points3.len() {
        let (x0, y0) = points3[i];
        let (x1, y1) = points3[(i + 1) % points3.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    // Rellenar el cuarto polígono con color verde
    fill_polygon_with_hole(&mut fb, &points4, &points5, Color::GREEN.to_u32());

    // Dibujar la orilla del cuarto polígono con color blanco
    for i in 0..points4.len() {
        let (x0, y0) = points4[i];
        let (x1, y1) = points4[(i + 1) % points4.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    // Dibujar la orilla del agujero con color blanco
    for i in 0..points5.len() {
        let (x0, y0) = points5[i];
        let (x1, y1) = points5[(i + 1) % points5.len()];
        draw_line(&mut fb, x0, y0, x1, y1, &Color::WHITE);
    }

    fb.render_buffer("polygon4.bmp").expect("No se pudo guardar el BMP");
    fb.display();
}
