use crate::{display::draw_line, vector::Vec2};

#[derive(Debug, Copy, Clone)]
pub struct Triangle {
    pub(crate) points: [Vec2; 3],
}

#[derive(Debug, Copy, Clone)]
pub struct Face {
    pub(crate) a: usize,
    pub(crate) b: usize,
    pub(crate) c: usize,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(points: [Vec2; 3]) -> Triangle {
        Triangle { points }
    }
}

#[allow(dead_code)]
impl Face {
    pub fn new(a: usize, b: usize, c: usize) -> Face {
        Face { a, b, c }
    }
}

pub fn draw_filled_triangle(
    color_buffer: &mut Vec<u8>,
    points: [Vec2; 3],
    color: sdl2::pixels::Color,
) {
    let mut points = points;
    points.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

    let x0 = points[0].x as i32;
    let y0 = points[0].y as i32;
    let x1 = points[1].x as i32;
    let y1 = points[1].y as i32;
    let x2 = points[2].x as i32;
    let y2 = points[2].y as i32;

    if y1 == y2 {
        fill_flat_bottom_triangle(color_buffer, x0, y0, x1, y1, x2, y2, color);
    } else if y0 == y1 {
        fill_flat_top_triangle(color_buffer, x0, y0, x1, y1, x2, y2, color);
    } else {
        let mx = (((x2 - x0) * (y1 - y0)) / (y2 - y0)) + x0; // find the middle point
        let my = y1;
        fill_flat_bottom_triangle(color_buffer, x0, y0, x1, y1, mx, my, color);
        fill_flat_top_triangle(color_buffer, x1, y1, mx, my, x2, y2, color);
    }
}

pub fn fill_flat_top_triangle(
    color_buffer: &mut Vec<u8>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: sdl2::pixels::Color,
) {
    // Find the two slopes (two triangle legs)
    let inv_slope_1 = (x2 - x0) as f32 / (y2 - y0) as f32;
    let inv_slope_2 = (x2 - x1) as f32 / (y2 - y1) as f32;

    // Start x_start and x_end from the bottom vertex (x2, y2)
    let mut x_start = x2 as f32;
    let mut x_end = x2 as f32;

    // Loop all the scanlines from bottom to top
    for y in (y0..=y2).rev() {
        draw_line(color_buffer, x_start as i32, y, x_end as i32, y, color);
        x_start -= inv_slope_1;
        x_end -= inv_slope_2;
    }
}

pub fn fill_flat_bottom_triangle(
    color_buffer: &mut Vec<u8>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    color: sdl2::pixels::Color,
) {
    // Find the two slopes (two triangle legs)
    let inv_slope_1 = (x1 - x0) as f32 / (y1 - y0) as f32;
    let inv_slope_2 = (x2 - x0) as f32 / (y2 - y0) as f32;

    // Start x_start and x_end from the top vertex (x0, y0)
    let mut x_start = x0 as f32;
    let mut x_end = x0 as f32;

    // Loop all the scanlines from top to bottom
    for y in y0..=y2 {
        draw_line(color_buffer, x_start as i32, y, x_end as i32, y, color);
        x_start += inv_slope_1;
        x_end += inv_slope_2;
    }
}
