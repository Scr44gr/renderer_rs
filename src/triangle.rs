use crate::vector::Vec2;

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
