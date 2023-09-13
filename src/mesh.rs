use std::io::Read;

use sdl2::pixels::Color;

use crate::triangle::Face;
use crate::vector;

pub struct Mesh {
    pub vertices: Vec<vector::Vec3>,
    pub faces: Vec<Face>,
    // mesh attributes
    pub rotation: vector::Vec3,
    pub scale: vector::Vec3,
    pub translation: vector::Vec3,
}

pub const N_CUBE_VERTICES: usize = 8;
pub const N_CUBE_FACES: usize = 6 * 2;

pub const CUBE_VERTICES: [vector::Vec3; N_CUBE_VERTICES] = [
    vector::Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    }, // 1
    vector::Vec3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    }, // 2
    vector::Vec3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    }, // 3
    vector::Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    }, // 4
    vector::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    }, // 5
    vector::Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    }, // 6
    vector::Vec3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    }, // 7
    vector::Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    }, // 8
];

pub const CUBE_FACES: [Face; N_CUBE_FACES] = [
    // front
    Face {
        a: 1,
        b: 2,
        c: 3,
        color: Color::RED,
    },
    Face {
        a: 1,
        b: 3,
        c: 4,
        color: Color::RED,
    },
    // right
    Face {
        a: 4,
        b: 3,
        c: 5,
        color: Color::GREEN,
    },
    Face {
        a: 4,
        b: 5,
        c: 6,
        color: Color::GREEN,
    },
    // back
    Face {
        a: 6,
        b: 5,
        c: 7,
        color: Color::BLUE,
    },
    Face {
        a: 6,
        b: 7,
        c: 8,
        color: Color::BLUE,
    },
    // left
    Face { a: 8, b: 7, c: 2, color: Color::YELLOW },
    Face { a: 8, b: 2, c: 1, color: Color::YELLOW },
    // top
    Face { a: 7, b: 5, c: 3, color: Color::CYAN },
    Face { a: 7, b: 3, c: 2, color: Color::CYAN },
    // bottom
    Face { a: 8, b: 1, c: 4, color: Color::MAGENTA },
    Face { a: 8, b: 4, c: 6, color: Color::MAGENTA },
];
impl Mesh {
    pub fn new_cube() -> Mesh {
        let mut vertices: Vec<vector::Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for i in 0..N_CUBE_VERTICES {
            vertices.push(CUBE_VERTICES[i]);
        }

        for i in 0..N_CUBE_FACES {
            faces.push(CUBE_FACES[i]);
        }
        Mesh {
            vertices: vertices,
            faces: faces,
            rotation: vector::Vec3::new(0.0, 0.0, 0.0),
            scale: vector::Vec3::new(1.0, 1.0, 1.0),
            translation: vector::Vec3::new(0.0, 0.0, 0.0),
        }
    }

    #[allow(dead_code)]
    pub fn load_from_file(filename: &str) -> Mesh {
        let mut vertices: Vec<vector::Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let mut file = std::fs::File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let lines = contents.lines();

        for line in lines {
            let mut words = line.split_whitespace();
            let result = words.next();
            if result.is_none() {
                continue;
            }

            match result.unwrap() {
                "v" => {
                    // v 0.000000 2.000000 2.000000
                    let x: f32 = words.next().unwrap().parse().unwrap();
                    let y: f32 = words.next().unwrap().parse().unwrap();
                    let z: f32 = words.next().unwrap().parse().unwrap();
                    vertices.push(vector::Vec3::new(x, y, z));
                }
                "f" => {
                    // f 1/1/1 5/2/1 4/3/1
                    let mut face: Face = Face::new(0, 0, 0, Color::RGBA(240, 240, 240, 255));
                    let mut i = 0;
                    for word in words {
                        let mut indices = word.split('/');
                        let index: usize = indices.next().unwrap().parse().unwrap();
                        match i {
                            0 => face.a = index,
                            1 => face.b = index,
                            2 => face.c = index,
                            _ => {}
                        }
                        i += 1;
                    }
                    faces.push(face);
                }
                _ => {}
            }
        }

        Mesh {
            vertices,
            faces,
            rotation: vector::Vec3::new(0.0, 0.0, 0.0),
            scale: vector::Vec3::new(1.0, 1.0, 1.0),
            translation: vector::Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
