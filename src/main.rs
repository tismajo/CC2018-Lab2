mod framebuffer_src;

use raylib::prelude::*;

use crate::framebuffer_src::{framebuffer::Framebuffer, line::{line}};

fn main() {
    let width = 800;
    let height = 600;

    let framebuffer_width = 800;
    let framebuffer_height = 600;

    let (mut window, raylib_thread) = raylib::init()
        .size(width, height)
        .title("Game of life")
        .resizable()
        .build();

    let mut framebuffer = Framebuffer::new(
        &mut window,
        &raylib_thread,
        framebuffer_width,
        framebuffer_height,
        Color::WHITE,
    );

    framebuffer.set_background_color(Color::new(44, 44, 127, 255));

    let mut x = 0.0;
    let mut y = 0.0;
    let mut dx = 1.5;
    let mut dy = 1.0;

    while !window.window_should_close() {
        if window.is_key_pressed(KeyboardKey::KEY_S) {
            framebuffer.save_screenshot(&mut window, &raylib_thread, "screenshot.png");
            println!("📸 Screenshot saved!");
        }

        x += dx;
        y += dy;

        // Rebote
        if x <= 0.0 || x + 350.0 >= framebuffer.width as f32 {
            dx = -dx;
        }
        if y <= 0.0 || y + 350.0 >= framebuffer.height as f32 {
            dy = -dy;
        }

        framebuffer.clear(&mut window, &raylib_thread);
        framebuffer.render(x, y, &mut window, &raylib_thread);
        framebuffer.swap_buffers(&mut window, &raylib_thread);

        // Simula 60fps
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
