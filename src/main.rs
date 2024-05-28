extern crate sdl2;

use display::FRAMES_PER_SECOND;
use matrix::Matrix;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::{f32::consts::PI, time::Duration};
use vector::{Vec2, Vec3, Vec4};
mod display;
mod matrix;
mod mesh;
mod triangle;
mod vector;

const LIGHT_DIRECTION: Vec3 = Vec3 {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    is_running: bool,
    color_buffer: Vec<u8>,
    triangles_to_render: Vec<triangle::Triangle>,
    camera_position: Vec3,
    mesh: mesh::Mesh,
    render_method: display::RenderMethod,
    cull_method: display::CullMethod,
    apply_light: bool,
    projection_matrix: Matrix,
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

        let color_buffer = vec![0; (display::WINDOW_WIDTH * display::WINDOW_HEIGHT * 3) as usize];
        let mesh = mesh::Mesh::load_from_file("./assets/f22.obj");
        //let mesh = mesh::Mesh::new_cube();

        // Initialize projection matrix
        let fov = PI / 3.0; // 60 degrees
        let aspect_ratio = display::WINDOW_HEIGHT as f32 / display::WINDOW_WIDTH as f32;
        let near = 1.0;
        let far = 100.0;

        let projection_matrix = Matrix::make_perspetive(fov, aspect_ratio, near, far);
        Renderer {
            sdl_context,
            canvas,
            color_buffer,
            is_running: true,
            camera_position: Vec3::new(0.0, 0.0, 0.0),
            triangles_to_render: Vec::new(),
            mesh: mesh,
            render_method: display::RenderMethod::Wireframe,
            cull_method: display::CullMethod::None,
            apply_light: true,
            projection_matrix: projection_matrix,
        }
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
                    Keycode::Num1 => self.render_method = display::RenderMethod::Wireframe,
                    Keycode::Num2 => self.render_method = display::RenderMethod::WireframeVertex,
                    Keycode::Num3 => self.render_method = display::RenderMethod::FillTriangle,
                    Keycode::Num4 => {
                        self.render_method = display::RenderMethod::FillTriangleWireframe
                    }
                    // Cull methods
                    Keycode::Num5 => self.cull_method = display::CullMethod::None,
                    Keycode::Num6 => self.cull_method = display::CullMethod::CullBackface,
                    Keycode::L => self.apply_light = !self.apply_light,
                    // to move the camera
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
        self.mesh.translation.z = 5.0;

        // Create Scale matrix that will be used to multiply the mesh vertices

        let mut scale_matrix =
            Matrix::new().scale(self.mesh.scale.x, self.mesh.scale.y, self.mesh.scale.z);

        let mut translation_matrix = Matrix::new().translate(
            self.mesh.translation.x,
            self.mesh.translation.y,
            self.mesh.translation.z,
        );
        let mut rotation_matrix_x = Matrix::new().rotate_x(self.mesh.rotation.x);
        let mut rotation_matrix_y = Matrix::new().rotate_y(self.mesh.rotation.y);
        let mut rotation_matrix_z = Matrix::new().rotate_z(self.mesh.rotation.z);

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
                // Use a matrix to scale, rotate, and translate the mesh
                transformed_vertex = scale_matrix.multiply(&mut transformed_vertex);
                transformed_vertex = rotation_matrix_x.multiply(&mut transformed_vertex);
                transformed_vertex = rotation_matrix_y.multiply(&mut transformed_vertex);
                transformed_vertex = rotation_matrix_z.multiply(&mut transformed_vertex);
                transformed_vertex = translation_matrix.multiply(&mut transformed_vertex);
                // Store transformed vertex
                transformed_vertices[j] = transformed_vertex;
            }
            let vector_a = Vec3::from_vec4(transformed_vertices[0]); //     A
            let vector_b = Vec3::from_vec4(transformed_vertices[1]); //   /   \
            let vector_c = Vec3::from_vec4(transformed_vertices[2]); //  C-----B

            // Calculate Normal
            let vector_ab = (vector_b - vector_a).normalize();
            let vector_ac = (vector_c - vector_a).normalize();
            let normal = vector_ab.cross(vector_ac).normalize();
            let mut light_color = cube_face.color;

            if self.apply_light {
                let light_direction = LIGHT_DIRECTION.normalize();
                let light_intensity = -normal.dot(light_direction);

                // clamp light intensity to make sure it's between 0 and 1
                let light_intensity = light_intensity.clamp(0.0, 1.0);
                light_color = self.light_apply_intensity(light_intensity, cube_face.color);
            }
            if self.cull_method == display::CullMethod::CullBackface {
                // Calculate Camera Ray
                let camera_ray = self.camera_position - vector_a;

                //  Calculate Camera Ray Dot Normal
                let dot_normal_camera = normal.dot(camera_ray);
                if dot_normal_camera < 0.0 {
                    continue;
                }
            }
            // Projecting 3D points to 2D
            let mut projected_points = [Vec4::new(0.0, 0.0, 0.0, 0.0); 3];

            for j in 0..3 {
                projected_points[j] = self
                    .projection_matrix
                    .multiply_vec4_projection(&transformed_vertices[j]);
                // Scaling projected point
                projected_points[j].x *= display::WINDOW_WIDTH as f32 / 2.0;
                projected_points[j].y *= display::WINDOW_HEIGHT as f32 / 2.0;

                // Transforming projected point to screen space
                projected_points[j].x += display::WINDOW_WIDTH as f32 / 2.0;
                projected_points[j].y += display::WINDOW_HEIGHT as f32 / 2.0;
            }
            // Calculating average depth of triangle
            let avg_depth =
                (transformed_vertices[0].z + transformed_vertices[1].z + transformed_vertices[2].z)
                    / transformed_vertices.len() as f32;

            let projected_triangle: triangle::Triangle = triangle::Triangle {
                points: [
                    Vec2::new(projected_points[0].x, projected_points[0].y),
                    Vec2::new(projected_points[1].x, projected_points[1].y),
                    Vec2::new(projected_points[2].x, projected_points[2].y),
                ],
                color: light_color,
                avg_depth: avg_depth,
            };
            // apply light intensity to triangle color
            self.triangles_to_render.push(projected_triangle);
        }
        // Sorting triangles by depth
        self.triangles_to_render
            .sort_by(|a, b| b.avg_depth.partial_cmp(&a.avg_depth).unwrap());
    }

    pub fn light_apply_intensity(
        &mut self,
        intensity: f32,
        color: sdl2::pixels::Color,
    ) -> sdl2::pixels::Color {
        let r = (color.r as f32 * intensity) as u8;
        let g = (color.g as f32 * intensity) as u8;
        let b = (color.b as f32 * intensity) as u8;
        sdl2::pixels::Color::RGBA(r, g, b, 255)
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
