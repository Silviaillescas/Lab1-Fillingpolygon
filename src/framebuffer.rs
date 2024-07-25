use crate::bmp;

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<u32>,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, background_color: u32) -> Framebuffer {
        Framebuffer {
            width,
            height,
            buffer: vec![background_color; width * height],
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            self.buffer[(self.height - y - 1) * self.width + x] = color;
        }
    }

    pub fn render_buffer(&self, file_path: &str) -> std::io::Result<()> {
        bmp::write_bmp_file(file_path, &self.buffer, self.width, self.height)
    }

    pub fn display(&self) {
        use minifb::{Window, WindowOptions, Key};
        let mut window = Window::new(
            "Framebuffer - Press ESC to exit",
            self.width,
            self.height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Convertir el buffer a un formato que minifb pueda usar
        let buffer: Vec<u32> = self.buffer.iter().map(|&color| {
            // Convertir de 0xAARRGGBB a 0x00RRGGBB
            let r = (color >> 16) & 0xFF;
            let g = (color >> 8) & 0xFF;
            let b = color & 0xFF;
            (r << 16) | (g << 8) | b
        }).collect();

        // Mostrar la ventana hasta que el usuario presione ESC
        while window.is_open() && !window.is_key_down(Key::Escape) {
            window.update_with_buffer(&buffer, self.width, self.height).unwrap();
        }
    }
}
