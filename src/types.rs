#[derive(Debug, Clone, Copy)]

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn length(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Vec3 {
        let len = self.length();
        if len == 0.0 {
            Vec3::new(0.0, 0.0, 0.0)
        } else {
            Vec3::new(self.x / len, self.y / len, self.z / len)
        }
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.z,
        }
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

#[derive(Debug, Clone, Copy)]

pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
pub struct MyShapes {
    pub _cube_vertices: [Vec3; 8],
    pub _cube_edges: [(usize, usize); 12],

    pub _pyramid_vertices: [Vec3; 5],
    pub _pyramid_edges: [(usize, usize); 8],

    pub _prism_vertices: [Vec3; 6],
    pub _prism_edges: [(usize, usize); 9],

    pub _tesseract_vertices: [Vec3; 16],
    pub _tesseract_edges: [(usize, usize); 32],
}
impl MyShapes {
    pub fn create_shapes() -> Self {
        let _cube_vertices = [
            Vec3::new(-0.5, -0.5, 3.5),
            Vec3::new(0.5, -0.5, 3.5),
            Vec3::new(0.5, 0.5, 3.5),
            Vec3::new(-0.5, 0.5, 3.5),
            Vec3::new(-0.5, -0.5, 2.5),
            Vec3::new(0.5, -0.5, 2.5),
            Vec3::new(0.5, 0.5, 2.5),
            Vec3::new(-0.5, 0.5, 2.5),
        ];

        let _cube_edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
        ];

        let _pyramid_vertices = [
            Vec3::new(-0.5, -0.5, 3.0), // base 0
            Vec3::new(0.5, -0.5, 3.0),  // base 1
            Vec3::new(0.5, 0.5, 3.0),   // base 2
            Vec3::new(-0.5, 0.5, 3.0),  // base 3
            Vec3::new(0.0, 0.0, 2.0),   // apex 4
        ];

        let _pyramid_edges = [
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (0, 4),
            (1, 4),
            (2, 4),
            (3, 4),
        ];

        let _prism_vertices = [
            Vec3::new(-0.5, -0.5, 3.5), // base triangle
            Vec3::new(0.5, -0.5, 3.5),
            Vec3::new(0.0, 0.5, 3.5),
            Vec3::new(-0.5, -0.5, 2.5), // top triangle
            Vec3::new(0.5, -0.5, 2.5),
            Vec3::new(0.0, 0.5, 2.5),
        ];

        let _prism_edges = [
            (0, 1),
            (1, 2),
            (2, 0),
            (3, 4),
            (4, 5),
            (5, 3),
            (0, 3),
            (1, 4),
            (2, 5),
        ];

        let _tesseract_vertices = [
            // Front cube
            Vec3::new(-0.5, -0.5, 3.5),
            Vec3::new(0.5, -0.5, 3.5),
            Vec3::new(0.5, 0.5, 3.5),
            Vec3::new(-0.5, 0.5, 3.5),
            Vec3::new(-0.5, -0.5, 2.5),
            Vec3::new(0.5, -0.5, 2.5),
            Vec3::new(0.5, 0.5, 2.5),
            Vec3::new(-0.5, 0.5, 2.5),
            // Back cube (offset in 4D, projected in 3D)
            Vec3::new(-0.8, -0.8, 3.8),
            Vec3::new(0.8, -0.8, 3.8),
            Vec3::new(0.8, 0.8, 3.8),
            Vec3::new(-0.8, 0.8, 3.8),
            Vec3::new(-0.8, -0.8, 2.2),
            Vec3::new(0.8, -0.8, 2.2),
            Vec3::new(0.8, 0.8, 2.2),
            Vec3::new(-0.8, 0.8, 2.2),
        ];

        let _tesseract_edges = [
            // Inner cube
            (0, 1),
            (1, 2),
            (2, 3),
            (3, 0),
            (4, 5),
            (5, 6),
            (6, 7),
            (7, 4),
            (0, 4),
            (1, 5),
            (2, 6),
            (3, 7),
            // Outer cube
            (8, 9),
            (9, 10),
            (10, 11),
            (11, 8),
            (12, 13),
            (13, 14),
            (14, 15),
            (15, 12),
            (8, 12),
            (9, 13),
            (10, 14),
            (11, 15),
            // Connections between cubes
            (0, 8),
            (1, 9),
            (2, 10),
            (3, 11),
            (4, 12),
            (5, 13),
            (6, 14),
            (7, 15),
        ];

        Self {
            _cube_vertices,
            _cube_edges,
            _pyramid_vertices,
            _pyramid_edges,
            _prism_vertices,
            _prism_edges,
            _tesseract_vertices,
            _tesseract_edges,
        }
    }
}
