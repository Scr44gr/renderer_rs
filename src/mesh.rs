use crate::triangle::Face;
use crate::vector;

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
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    // right
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    // back
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    // left
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    // top
    Face { a: 7, b: 5, c: 3 },
    Face { a: 7, b: 3, c: 2 },
    // bottom
    Face { a: 8, b: 1, c: 4 },
    Face { a: 8, b: 4, c: 6 },
];

pub struct Mesh {
    pub vertices: Vec<vector::Vec3>,
    pub faces: Vec<Face>,
    // mesh attributes
    pub rotation: vector::Vec3,
}

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
            vertices,
            faces,
            rotation: vector::Vec3::new(0.0, 0.0, 0.0),
        }
    }
}
