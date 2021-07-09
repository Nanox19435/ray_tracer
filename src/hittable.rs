use super::structs::{vec3::Vector3, ray::Ray};

pub struct HitRecord {
    pub p: Vector3,
    pub normal: Vector3,
    t: f64,
    front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vector3) {
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {outward_normal} else {-outward_normal}
    }
}

impl std::default::Default for HitRecord {
    fn default() -> Self {
        HitRecord {p:Vector3::new(0.0,0.0,0.0), normal:Vector3::new(0.0,0.0,0.0), t:0.0, front_face:false}
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

//Hittable objects:
pub mod sphere;

pub mod hittable_list;
