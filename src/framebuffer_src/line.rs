use raylib::prelude::*;
use framebuffer::Framebuffer;

use crate::framebuffer_src::framebuffer;

// Method for writing a line
pub fn line(
    framebuffer: &mut Framebuffer,
    window: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    start: Vector2,
    end: Vector2,
) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        framebuffer.set_pixel(window, raylib_thread, x0, y0);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}
