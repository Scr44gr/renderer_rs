extern crate sdl2;

use display::FRAMES_PER_SECOND;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;
use vector::Vec3;
mod display;
mod mesh;
mod triangle;
mod vector;

struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    color_buffer: Vec<u8>,
    is_running: bool,
    fov_factor: f32,
    camera_position: Vec3,
    triangles_to_render: Vec<triangle::Triangle>,
    mesh: mesh::Mesh,
}

impl Renderer {
    pub fn new(window: Window, sdl_context: Sdl) -> Renderer {
        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let color_buffer = vec![0; (display::WINDOW_WIDTH * display::WINDOW_HEIGHT * 3) as usize];
        let mesh = mesh::Mesh::new_cube();

        Renderer {
            sdl_context,
            canvas,
            color_buffer,
            is_running: true,
            fov_factor: 700.0,
            camera_position: Vec3::new(0.0, 5.0, -5.0),
            triangles_to_render: Vec::new(),
            mesh,
        }
    }

    pub fn project(&mut self, point: vector::Vec3) -> vector::Vec2 {
        let projected_point = vector::Vec2 {
            x: (self.fov_factor * point.x) / point.z,
            y: (self.fov_factor * point.y) / point.z,
        };
        projected_point
    }

    pub fn process_input(&mut self) {
        let mut events = self.sdl_context.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                Event::MouseWheel { y, .. } => {
                    self.camera_position.z += y as f32;
                }

                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }
    pub fn update(&mut self) {
        self.mesh.rotation.x += 0.02;
        self.mesh.rotation.y += 0.02;
        self.mesh.rotation.z += 0.02;

        for i in 0..mesh::N_CUBE_FACES {
            let cube_face = &mesh::CUBE_FACES[i];

            let mut face_vertices: [Vec3; 3] = [Vec3::new(0.0, 0.0, 0.0); 3];
            face_vertices[0] = mesh::CUBE_VERTICES[cube_face.a - 1];
            face_vertices[1] = mesh::CUBE_VERTICES[cube_face.b - 1];
            face_vertices[2] = mesh::CUBE_VERTICES[cube_face.c - 1];


            let mut projected_triangle: triangle::Triangle = triangle::Triangle {
                points: [vector::Vec2 { x: 0.0, y: 0.0 }; 3],
            };

            for j in 0..3 {
                let mut transformed_vertex = face_vertices[j];
                transformed_vertex = transformed_vertex.rotate_x(self.mesh.rotation.x);
                transformed_vertex = transformed_vertex.rotate_y(self.mesh.rotation.y);
                transformed_vertex = transformed_vertex.rotate_z(self.mesh.rotation.z);

                transformed_vertex.z -= self.camera_position.z;
                let mut projected_point = self.project(transformed_vertex);

                // Scale and translate the projected point to the center of the screen
                projected_point.x += display::WINDOW_WIDTH as f32 / 2.0;
                projected_point.y += display::WINDOW_HEIGHT as f32 / 2.0;
                projected_triangle.points[j] = projected_point;
            }
            self.triangles_to_render.push(projected_triangle);
        }
    }

    pub fn render(&mut self) {
        let num_triangles = self.triangles_to_render.len();

        for i in 0..num_triangles {
            let triangle = &self.triangles_to_render[i];
            display::draw_triangle(
                &mut self.color_buffer,
                triangle.points,
                sdl2::pixels::Color::RGBA(0, 150, 0, 255),
            );
        }

        self.triangles_to_render.clear();
        display::render_color_buffer(&mut self.canvas, &mut self.color_buffer);
        display::clear_color_buffer(&mut self.color_buffer);
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SECOND));
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window = display::initialize_window(&sdl_context);
    let mut renderer = Renderer::new(window.unwrap(), sdl_context);

    while renderer.is_running {
        renderer.process_input();
        renderer.update();
        renderer.render();
    }
}