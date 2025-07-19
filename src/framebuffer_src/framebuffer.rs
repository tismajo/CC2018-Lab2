use raylib::prelude::*;

pub struct Framebuffer {
    pub render_texture: RenderTexture2D,
    pub width: u32,
    pub height: u32,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new(
        window: &mut RaylibHandle,
        raylib_thread: &RaylibThread,
        width: u32,
        height: u32,
        background_color: Color,
    ) -> Self {
        let render_texture = window.load_render_texture(&raylib_thread, width, height)
            .expect("No se pudo crear render texture");

        Framebuffer {
            render_texture,
            width,
            height,
            background_color,
            current_color: Color::BLACK,
        }
    }

    pub fn clear(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        let mut d = window.begin_texture_mode(&raylib_thread, &mut self.render_texture);
        d.clear_background(self.background_color);
    }

    pub fn set_pixel(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread, x: i32, y: i32) {
        let mut d = window.begin_texture_mode(&raylib_thread, &mut self.render_texture);
        d.draw_pixel(x, y, self.current_color);
    }

    pub fn set_cell(&mut self, window: &mut RaylibHandle, raylib_thread: &RaylibThread, x: i32, y: i32, alive: bool) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }
        self.current_color = if alive { Color::WHITE } else { Color::BLACK };
        self.set_pixel(window, raylib_thread, x, y);
    }

    pub fn swap_buffers(&self, window: &mut RaylibHandle, raylib_thread: &RaylibThread) {
        let screen_width = window.get_screen_width() as f32;
        let screen_height = window.get_screen_height() as f32;

        let scale_x = screen_width / self.width as f32;
        let scale_y = screen_height / self.height as f32;
        let scale = scale_x.min(scale_y);

        let pos_x = (screen_width - (self.width as f32 * scale)) / 2.0;
        let pos_y = (screen_height - (self.height as f32 * scale)) / 2.0;

        let mut d = window.begin_drawing(&raylib_thread);
        d.clear_background(Color::BLACK);
        d.draw_texture_ex(
            &self.render_texture,
            Vector2::new(pos_x, pos_y),
            0.0,
            scale,
            Color::YELLOW,
        );
    }
}
