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
