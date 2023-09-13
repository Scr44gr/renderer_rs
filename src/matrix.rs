#![allow(dead_code)]
use crate::vector;


pub struct Matrix {
    pub data: [[f32; 4]; 4],
}


impl Matrix {
    
    pub fn new() -> Matrix {
        Matrix {
            data: [[0.0; 4]; 4],
        }
    }
    
    pub fn identity() -> Matrix {
        let mut m = Matrix::new();
        m.data[0][0] = 1.0;
        m.data[1][1] = 1.0;
        m.data[2][2] = 1.0;
        m.data[3][3] = 1.0;
        m
    }
    pub fn scale(&mut self, sx: f32, sy: f32, sz: f32) -> Matrix {
        // | sx  0   0   0 |
        // | 0   sy  0   0 |
        // | 0   0   sz  0 |
        // | 0   0   0   1 |
        let mut m = Matrix::identity();
        m.data[0][0] = sx;
        m.data[1][1] = sy;
        m.data[2][2] = sz;
        m
    }

    pub fn multiply(&mut self, vector: &mut vector::Vec4) -> vector::Vec4{
        let mut result = vector::Vec4::new(0.0, 0.0, 0.0, 0.0);

        result.x = vector.x * self.data[0][0]
            + vector.y * self.data[1][0]
            + vector.z * self.data[2][0]
            + vector.w * self.data[3][0];
        result.y = vector.x * self.data[0][1]
            + vector.y * self.data[1][1]
            + vector.z * self.data[2][1]
            + vector.w * self.data[3][1];
        result.z = vector.x * self.data[0][2]
            + vector.y * self.data[1][2]
            + vector.z * self.data[2][2]
            + vector.w * self.data[3][2];
        result.w = vector.x * self.data[0][3]
            + vector.y * self.data[1][3]
            + vector.z * self.data[2][3]
            + vector.w * self.data[3][3];

        *vector = result;

        result
    }
    
    /// Transform a vector by this matrix
    pub fn transform(&self, v: &vector::Vec3) -> vector::Vec3 {
        let mut result = vector::Vec3::new(0.0, 0.0, 0.0);

        result.x = v.x * self.data[0][0]
            + v.y * self.data[1][0]
            + v.z * self.data[2][0]
            + self.data[3][0];
        result.y = v.x * self.data[0][1]
            + v.y * self.data[1][1]
            + v.z * self.data[2][1]
            + self.data[3][1];
        result.z = v.x * self.data[0][2]
            + v.y * self.data[1][2]
            + v.z * self.data[2][2]
            + self.data[3][2];

        let w = v.x * self.data[0][3]
            + v.y * self.data[1][3]
            + v.z * self.data[2][3]
            + self.data[3][3];

        if w != 0.0 {
            result.x /= w;
            result.y /= w;
            result.z /= w;
        }
        result
    }
}