use super::{Ray, Vector3, HitRecord};
use super::super::structs::vec3;
//All materials will be defined in this module

use Vector3 as Color;

pub trait Material {
    //The book returns a boolean and uses two references to external variables to pass the
    //scattered ray and color. Both Rust and I do not like this approach, and as such, it is
    //replaced by returning a tuple and the Option enum.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)>;
}


pub struct Lambertian {
    pub albedo: Color
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = hit_record.normal + vec3::random_in_unit_sphere();

        //it doesn't make sense for a something as specific as this to be a function, specially if
        //it's only being used once.
        if (scatter_direction.x < 1e-8) && (scatter_direction.y < 1e-8) && (scatter_direction.z < 1e-8) {
            scatter_direction = hit_record.normal;
        }

        Some((Ray::new(hit_record.p, scatter_direction), self.albedo))
    }
}

pub struct Metal {
    pub albedo: Color
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * n.dot(v) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vector3 = reflect(ray.direction.normalized(), hit_record.normal);
        if reflected.dot(hit_record.normal) > 0.0 {
            Some((Ray::new(hit_record.p, reflected), self.albedo))
        } else {
            None
        }
    }
}
