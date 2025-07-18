use raylib::prelude::*;
use crate::framebuffer_src::framebuffer::Framebuffer;
use crate::framebuffer_src::line::line;

pub fn fill_polygon(
    framebuffer: &mut Framebuffer,
    window: &mut RaylibHandle,
    raylib_thread: &RaylibThread,
    vertices: &Vec<Vector2>,
) {
    for i in 0..vertices.len() {
        let a = vertices[i];
        let b = vertices[(i + 1) % vertices.len()];
        line(framebuffer, window, raylib_thread, a, b);
    }

    let min_y = vertices.iter().map(|v| v.y as i32).min().unwrap_or(0);
    let max_y = vertices.iter().map(|v| v.y as i32).max().unwrap_or(0);

    for y in min_y..=max_y {
        let mut intersections = vec![];

        for i in 0..vertices.len() {
            let v1 = vertices[i];
            let v2 = vertices[(i + 1) % vertices.len()];

            let (x0, y0) = (v1.x, v1.y);
            let (x1, y1) = (v2.x, v2.y);

            if (y0 as i32 <= y && y1 as i32 > y) || (y1 as i32 <= y && y0 as i32 > y) {
                let x = x0 + (y as f32 - y0) * (x1 - x0) / (y1 - y0);
                intersections.push(x as i32);
            }
        }

        intersections.sort();
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let x_start = intersections[i];
                let x_end = intersections[i + 1];
                for x in x_start..=x_end {
                    framebuffer.set_pixel(window, raylib_thread, x, y);
                }
            }
        }
    }
}
