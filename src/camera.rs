use super::structs::{vec3::Vector3, ray::Ray};

pub struct Camera {
    origin: Vector3,
    lower_left_corner: Vector3,
    horizontal: Vector3,
    vertical: Vector3
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin
        }
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let aspect_ratio: f64 = 16.0/9.0;
        let viewport_height: f64 = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        Camera {
            origin: Vector3::new(0.0, 0.0, 0.0),
            horizontal: Vector3::new(viewport_width, 0.0, 0.0),
            vertical: Vector3::new(0.0, viewport_height, 0.0),
            lower_left_corner: Vector3::new(-viewport_width/2.0, -viewport_height/2.0, -focal_length)
        }
    }
}