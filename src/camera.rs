use crate::types::Vec3;
#[derive(Debug)]
pub struct Camera {
    pub pos: Vec3,
    pub fov: f64,
    pub aspect_ratio: f64,

    pub near_plane: f64,
    pub far_plane: f64,

    pub yaw: f64,
    pub pitch: f64,

    pub move_speed: f64,
    pub rotate_speed: f64,
}
impl Camera {
    pub fn default() -> Self {
        Self {
            pos: Vec3::new(0.0, 0.0, -1.0),
            fov: 45.0,
            aspect_ratio: 16.0 / 9.0,

            near_plane: 0.1,
            far_plane: 100.0,

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

        let x = (view_space.x / (scale * view_space.z)) * self.aspect_ratio;
        let y = view_space.y / (scale * view_space.z);
        (x, y)
    }

    pub fn apply_view_transform(&self, point: Vec3) -> Vec3 {
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

    pub fn _clip_line_to_near_plane(&self, v1: Vec3, v2: Vec3) -> Option<(Vec3, Vec3)> {
        // Check if both points are behind the camera
        if v1.z <= self.near_plane && v2.z <= self.near_plane {
            return None;
        }

        // If both points are in front of the near plane, no clipping needed
        if v1.z > self.near_plane && v2.z > self.near_plane {
            return Some((v1, v2));
        }

        // One point is behind, one is in front - we need to clip
        let t = (self.near_plane - v1.z) / (v2.z - v1.z);
        let intersection = Vec3 {
            x: v1.x + t * (v2.x - v1.x),
            y: v1.y + t * (v2.y - v1.y),
            z: self.near_plane,
        };

        if v1.z > self.near_plane {
            return Some((v1, intersection));
        } else {
            return Some((intersection, v2));
        }
    }

    pub fn cast_ray(&self, u: f64, v: f64) -> Vec3 {
        let fov_rad = (self.fov.to_radians() / 2.0).tan();

        let x = u * fov_rad * self.aspect_ratio;
        let y = v * fov_rad;
        let ray_camera = Vec3::new(x, y, 1.0).normalize();

        self.camera_to_world(ray_camera)
    }

    pub fn check_ray_aabb_intersections(&self, ray_direction: Vec3, min: Vec3, max: Vec3) -> bool {
        let inv_dir = Vec3 {
            x: 1.0 / ray_direction.x,
            y: 1.0 / ray_direction.y,
            z: 1.0 / ray_direction.z,
        };

        let mut tmin = (min.x - self.pos.x) * inv_dir.x;
        let mut tmax = (max.x - self.pos.x) * inv_dir.x;
        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (min.y - self.pos.y) * inv_dir.y;
        let mut tymax = (max.y - self.pos.y) * inv_dir.y;
        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        if tymin > tmin {
            tmin = tymin;
        }
        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (min.z - self.pos.z) * inv_dir.z;
        let mut tzmax = (max.z - self.pos.z) * inv_dir.z;
        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        true
    }

    pub fn camera_to_world(&self, dir: Vec3) -> Vec3 {
        let forward = self.forward().normalize();
        let right = self.right().normalize();
        let up = forward.cross(right).normalize();

        Vec3 {
            x: dir.x * right.x + dir.y * up.x + dir.z * forward.x,
            y: dir.x * right.y + dir.y * up.y + dir.z * forward.y,
            z: dir.x * right.z + dir.y * up.z + dir.z * forward.z,
        }
    }
}
