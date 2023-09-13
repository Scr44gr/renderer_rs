extern crate sdl2;

use display::FRAMES_PER_SECOND;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;
use vector::{Vec2, Vec3, Vec4};
use matrix::Matrix;
mod display;
mod mesh;
mod triangle;
mod vector;
mod matrix;


struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    color_buffer: Vec<u8>,
    is_running: bool,
    fov_factor: f32,
    camera_position: Vec3,
    triangles_to_render: Vec<triangle::Triangle>,
    mesh: mesh::Mesh,
    render_method: display::RenderMethod,
    cull_method: display::CullMethod,
}

impl Renderer {
    pub fn new(window: Window, sdl_context: Sdl) -> Renderer {
        let canvas = window
            .into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let color_buffer = vec![0; (display::WINDOW_WIDTH * display::WINDOW_HEIGHT * 5) as usize];
        //let mesh = mesh::Mesh::load_from_file("./assets/f22.obj");
        let mesh = mesh::Mesh::new_cube();

        Renderer {
            sdl_context,
            canvas,
            color_buffer,
            is_running: true,
            fov_factor: 700.0,
            camera_position: Vec3::new(0.0, 0.0, 0.0),
            triangles_to_render: Vec::new(),
            mesh: mesh,
            render_method: display::RenderMethod::Wireframe,
            cull_method: display::CullMethod::None,
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
                Event::KeyDown {
                    keycode: Some(keycode),
                    ..
                } => match keycode {
                    Keycode::Escape => self.is_running = false,
                    // Render methods
                    Keycode::Kp1 => self.render_method = display::RenderMethod::Wireframe,
                    Keycode::Kp2 => self.render_method = display::RenderMethod::WireframeVertex,
                    Keycode::Kp3 => self.render_method = display::RenderMethod::FillTriangle,
                    Keycode::Kp4 => self.render_method = display::RenderMethod::FillTriangleWireframe,
                    // Cull methods
                    Keycode::Kp5 => self.cull_method = display::CullMethod::None,
                    Keycode::Kp6 => self.cull_method = display::CullMethod::CullBackface,
                    _ => {}
                },
                _ => {}
            }
        }
    }
    pub fn update(&mut self) {
        // change the mesh roration/scale values per animation frame
        self.mesh.rotation.x += 0.02;
        self.mesh.rotation.y += 0.02;
        self.mesh.rotation.z += 0.01;

        self.mesh.scale.x += 0.02;
        self.mesh.scale.y += 0.02;
        // Create Scale matrix that will be used to multiply the mesh vertices

        let mut scale_matrix = Matrix::new().scale(
            self.mesh.scale.x,
            self.mesh.scale.y,
            self.mesh.scale.z,
        );

        let num_faces = self.mesh.faces.len();
        for i in 0..num_faces {
            let cube_face = self.mesh.faces[i];

            let mut face_vertices: [Vec3; 3] = [Vec3::new(0.0, 0.0, 0.0); 3];
            face_vertices[0] = self.mesh.vertices[cube_face.a - 1];
            face_vertices[1] = self.mesh.vertices[cube_face.b - 1];
            face_vertices[2] = self.mesh.vertices[cube_face.c - 1];

            let mut transformed_vertices: [Vec4; 3] = [Vec4::new(0.0, 0.0, 0.0, 0.0); 3];

            // Transforming vertices
            for j in 0..3 {
                let mut transformed_vertex = Vec4::from_vec3(face_vertices[j]);
                // use a matrix to scale the vertices
                transformed_vertex = scale_matrix.multiply(&mut transformed_vertex);
                transformed_vertex.z += 5.0; 
                transformed_vertices[j] = transformed_vertex; 
            }

            if self.cull_method == display::CullMethod::CullBackface {
                // Applying backface culling
                // Getting vectors
                let vector_a = Vec3::from_vec4(transformed_vertices[0]); //     A
                let vector_b = Vec3::from_vec4(transformed_vertices[1]); //   /   \
                let vector_c = Vec3::from_vec4(transformed_vertices[2]); //  C-----B

                // Calculate Normal
                let vector_ab = (vector_b - vector_a).normalize();
                let vector_ac = (vector_c - vector_a).normalize();
                let normal = vector_ab.cross(vector_ac).normalize();

                // Calculate Camera Ray
                let camera_ray = self.camera_position - vector_a;

                //  Calculate Camera Ray Dot Normal
                let dot_normal_camera = normal.dot(camera_ray);
                if dot_normal_camera < 0.0 {
                    continue;
                }
            }
            // Projecting 3D points to 2D
            let mut projected_points = [Vec2::new(0.0, 0.0); 3];
            for j in 0..3 {
                let mut projected_point = self.project(Vec3::from_vec4(transformed_vertices[j]));

                projected_point.x += display::WINDOW_WIDTH as f32 / 2.0;
                projected_point.y += display::WINDOW_HEIGHT as f32 / 2.0;
                projected_points[j] = projected_point;
            }
            // Calculating average depth of triangle
            let avg_depth =
                (transformed_vertices[0].z + transformed_vertices[1].z + transformed_vertices[2].z)
                    / transformed_vertices.len() as f32;

            let projected_triangle: triangle::Triangle = triangle::Triangle {
                points: projected_points,
                color: self.mesh.faces[i].color,
                avg_depth: avg_depth,
            };
            self.triangles_to_render.push(projected_triangle);
        }
        // Sorting triangles by depth
        self.triangles_to_render.sort_by(|a, b| b.avg_depth.partial_cmp(&a.avg_depth).unwrap());
    }

    pub fn render(&mut self) {
        let num_triangles = self.triangles_to_render.len();

        for i in 0..num_triangles {
            let triangle = &self.triangles_to_render[i];

            match self.render_method {
                // Draw wireframe triangle
                display::RenderMethod::Wireframe => {
                    display::draw_triangle(
                        &mut self.color_buffer,
                        triangle.points,
                        sdl2::pixels::Color::RGBA(255, 255, 255, 255),
                        false,
                    );
                }
                // Draw wireframe with vertex points
                display::RenderMethod::WireframeVertex => {
                    display::draw_triangle(
                        &mut self.color_buffer,
                        triangle.points,
                        sdl2::pixels::Color::RGBA(255, 255, 255, 255),
                        true,
                    );
                }
                // Draw filled triangle
                display::RenderMethod::FillTriangle => {
                    triangle::draw_filled_triangle(
                        &mut self.color_buffer,
                        triangle.points,
                        triangle.color,
                    );
                }
                // Draw filled triangle and then draw wireframe on top
                display::RenderMethod::FillTriangleWireframe => {
                    triangle::draw_filled_triangle(
                        &mut self.color_buffer,
                        triangle.points,
                        triangle.color,
                    );
                    display::draw_triangle(
                        &mut self.color_buffer,
                        triangle.points,
                        sdl2::pixels::Color::RGBA(0, 0, 0, 255),
                        false,
                    );
                }
            }
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
