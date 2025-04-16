use crate::types::Vec3;
#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub fov: f64,

    pub yaw: f64,
    pub pitch: f64,

    pub move_speed: f64,
    pub rotate_speed: f64,
}
impl Camera {
    pub fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -1.0),
            fov: 90.0,

            yaw: 0.0,
            pitch: 0.0,

            move_speed: 0.1,
            rotate_speed: 5.0,
        }
    }

    fn forward(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        let pitch_rad = self.pitch.to_radians();

        Vec3 {
            x: pitch_rad.cos() * yaw_rad.sin(),
            y: pitch_rad.sin(),
            z: pitch_rad.cos() * yaw_rad.cos(),
        }
    }

    pub fn right(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();
        Vec3 {
            x: yaw_rad.cos(),
            y: 0.0,
            z: -yaw_rad.sin(),
        }
    }
    pub fn forward_movement(&self) -> Vec3 {
        let yaw_rad = self.yaw.to_radians();

        Vec3 {
            x: yaw_rad.sin(),
            y: 0.0,
            z: yaw_rad.cos(),
        }
    }

    pub fn project_vertex(&self, vertex: &Vec3) -> (f64, f64) {
        // first apply view transformation
        let view_space = self.apply_view_transform(*vertex - self.pos);

        // we dont want to project points behind the camera
        if view_space.z <= 0.0 {
            return (10.0, 10.0); // Place points behind camera off-screen
        }

        // Now do perspective projection
        let scale = (self.fov / 2.0).to_radians().tan();
        let aspect_ratio = 9.0 / 16.0;
        let x = (view_space.x / (scale * view_space.z)) * aspect_ratio;
        let y = view_space.y / (scale * view_space.z);
        (x, y)
    }

    fn apply_view_transform(&self, point: Vec3) -> Vec3 {
        // create rotation matrices for yaw and pitch
        let yaw_rad = -self.yaw.to_radians();
        let pitch_rad = -self.pitch.to_radians();

        // First rotate around Y axis (yaw)
        let mut result = Vec3 {
            x: point.x * yaw_rad.cos() + point.z * yaw_rad.sin(),
            y: point.y,
            z: -point.x * yaw_rad.sin() + point.z * yaw_rad.cos(),
        };

        // Then rotate around X axis (pitch)
        result = Vec3 {
            x: result.x,
            y: result.y * pitch_rad.cos() - result.z * pitch_rad.sin(),
            z: result.y * pitch_rad.sin() + result.z * pitch_rad.cos(),
        };

        result
    }

    pub fn clip_line_to_near_plane(&self, v1: Vec3, v2: Vec3, near_z: f64) -> Option<(Vec3, Vec3)> {
        // Check if both points are behind the camera
        if v1.z <= near_z && v2.z <= near_z {
            return None;
        }

        // If both points are in front of the near plane, no clipping needed
        if v1.z > near_z && v2.z > near_z {
            return Some((v1, v2));
        }

        // One point is behind, one is in front - we need to clip
        let t = (near_z - v1.z) / (v2.z - v1.z);
        let intersection = Vec3 {
            x: v1.x + t * (v2.x - v1.x),
            y: v1.y + t * (v2.y - v1.y),
            z: near_z,
        };

        if v1.z > near_z {
            return Some((v1, intersection));
        } else {
            return Some((intersection, v2));
        }
    }
}
