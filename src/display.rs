// This file contains the code for the display module
extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowBuildError};

use crate::vector::Vec2;

// Render Methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderMethod {
    Wireframe,             // default
    WireframeVertex,       // wireframe with vertex
    FillTriangle,          // fill triangle
    FillTriangleWireframe, // fill triangle with wireframe
}

// CULL METHODS
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CullMethod {
    None, // default
    CullBackface,
}

pub const WINDOW_WIDTH: u32 = 1080;
pub const WINDOW_HEIGHT: u32 = 720;
pub const FRAMES_PER_SECOND: u32 = 60;

const WINDOW_TITLE: &str = "Renderer Learning";

pub fn initialize_window(sdl_context: &sdl2::Sdl) -> Result<Window, WindowBuildError> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .borderless()
        .build()?;
    Ok(window)
}

pub fn clear_color_buffer(color_buffer: &mut Vec<u8>) {
    *color_buffer = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize];
}

pub fn render_color_buffer(canvas: &mut Canvas<Window>, color_buffer: &Vec<u8>) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .unwrap();

    texture
        .update(None, &color_buffer, (WINDOW_WIDTH * 3) as usize)
        .unwrap();
    canvas.copy(&texture, None, None).unwrap();
}

pub fn draw_pixel(color_buffer: &mut [u8], x: u32, y: u32, color: sdl2::pixels::Color) {
    if x >= WINDOW_WIDTH || y >= color_buffer.len() as u32 / (WINDOW_WIDTH * 3) {
        return;
    }
    let pixel_index = (y * WINDOW_WIDTH + x) as usize * 3;
    color_buffer[pixel_index] = color.r;
    color_buffer[pixel_index + 1] = color.g;
    color_buffer[pixel_index + 2] = color.b;
}

pub fn draw_rect(
    color_buffer: &mut Vec<u8>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: sdl2::pixels::Color,
) {
    if x >= WINDOW_WIDTH || y >= WINDOW_HEIGHT {
        return;
    }

    for row in y..y + height {
        for col in x..x + width {
            draw_pixel(color_buffer, col, row, color);
        }
    }
}

#[allow(dead_code)]
pub fn draw_grid(color_buffer: &mut Vec<u8>, size: usize) {
    for y in (0..WINDOW_HEIGHT).step_by(size) {
        for x in (0..WINDOW_WIDTH).step_by(size) {
            draw_pixel(
                color_buffer,
                x,
                y,
                sdl2::pixels::Color::RGBA(255, 255, 255, 255),
            );
        }
    }
}

pub fn draw_triangle(
    color_buffer: &mut Vec<u8>,
    points: [Vec2; 3],
    color: sdl2::pixels::Color,
    allow_drawing_vertex: bool,
) {
    for i in 0..3 {
        let p0 = points[i];
        let p1 = points[(i + 1) % 3];
        draw_line(
            color_buffer,
            p0.x as i32,
            p0.y as i32,
            p1.x as i32,
            p1.y as i32,
            color,
        );
        if allow_drawing_vertex {
            draw_rect(
                color_buffer,
                p0.x as u32 - 2,
                p0.y as u32 - 2,
                4,
                4,
                sdl2::pixels::Color::RGBA(255, 0, 0, 255),
            );
        }
    }
}

pub fn draw_line(
    color_buffer: &mut Vec<u8>,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: sdl2::pixels::Color,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut e2;

    loop {
        draw_pixel(color_buffer, x0 as u32, y0 as u32, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        e2 = 2 * err;

        if e2 > dy {
            err += dy;
            x0 += sx;
        }

        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}
