use std::io::Read;

use crate::triangle::Face;
use crate::vector;

pub struct Mesh {
    pub vertices: Vec<vector::Vec3>,
    pub faces: Vec<Face>,
    // mesh attributes
    pub rotation: vector::Vec3,
}

impl Mesh {
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
                    let mut face: Face = Face::new(0, 0, 0);
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
        }
    }
}
