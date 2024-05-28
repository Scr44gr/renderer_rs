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
        m.data[3][3] = 1.0;
        m
    }

    pub fn multiply(&mut self, vector: &mut vector::Vec4) -> vector::Vec4 {
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
    // Translate matrix
    pub fn translate(&mut self, tx: f32, ty: f32, tz: f32) -> Matrix {
        // | 1   0   0   tx |
        // | 0   1   0   ty |
        // | 0   0   1   tz |
        // | 0   0   0   1  |
        let mut m = Matrix::identity();
        m.data[3][0] = tx;
        m.data[3][1] = ty;
        m.data[3][2] = tz;
        m
    }
    /// Transform a vector by this matrix
    pub fn transform(&self, v: &vector::Vec3) -> vector::Vec3 {
        let mut result = vector::Vec3::new(0.0, 0.0, 0.0);

        result.x =
            v.x * self.data[0][0] + v.y * self.data[1][0] + v.z * self.data[2][0] + self.data[3][0];
        result.y =
            v.x * self.data[0][1] + v.y * self.data[1][1] + v.z * self.data[2][1] + self.data[3][1];
        result.z =
            v.x * self.data[0][2] + v.y * self.data[1][2] + v.z * self.data[2][2] + self.data[3][2];

        let w =
            v.x * self.data[0][3] + v.y * self.data[1][3] + v.z * self.data[2][3] + self.data[3][3];

        if w != 0.0 {
            result.x /= w;
            result.y /= w;
            result.z /= w;
        }
        result
    }

    pub fn rotate_x(&mut self, angle: f32) -> Matrix {
        // | 1   0       0       0 |
        // | 0   cos(a)  sin(a) 0 |
        // | 0   -sin(a)  cos(a)  0 |
        // | 0   0       0       1 |
        let mut m = Matrix::identity();
        m.data[1][1] = angle.cos();
        m.data[1][2] = angle.sin();
        m.data[2][1] = -angle.sin();
        m.data[2][2] = angle.cos();
        m
    }

    pub fn rotate_y(&mut self, angle: f32) -> Matrix {
        // | cos(a)  0   -sin(a)  0 |
        // | 0       1   0       0 |
        // | sin(a) 0   cos(a)  0 |
        // | 0       0   0       1 |
        let mut m = Matrix::identity();
        m.data[0][0] = angle.cos();
        m.data[0][2] = -angle.sin();
        m.data[2][0] = angle.sin();
        m.data[2][2] = angle.cos();
        m
    }

    pub fn rotate_z(&mut self, angle: f32) -> Matrix {
        // | cos(a)  sin(a) 0   0 |
        // | -sin(a)  cos(a)  0   0 |
        // | 0       0       1   0 |
        // | 0       0       0   1 |
        let mut m = Matrix::identity();
        m.data[0][0] = angle.cos();
        m.data[0][1] = angle.sin();
        m.data[1][0] = -angle.sin();
        m.data[1][1] = angle.cos();
        m
    }

    pub fn make_perspetive(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Matrix {
        let mut m = Matrix {
            data: [[0.0; 4]; 4],
        };
        m.data[0][0] = aspect_ratio * (1.0 / (fov / 2.0).tan());
        m.data[1][1] = 1.0 / (fov / 2.0).tan();
        m.data[2][2] = far / (far - near);
        m.data[2][3] = (-far * near) / (far - near);
        m.data[3][2] = 1.0;

        m
    }

    pub fn multiply_vec4_projection(&mut self, v: &vector::Vec4) -> vector::Vec4 {
        let mut result = self.multiply(&mut vector::Vec4::new(v.x, v.y, v.z, v.w));
        if result.w != 0.0 {
            result.x /= result.w;
            result.y /= result.w;
            result.z /= result.w;
        }
        result
    }
}
