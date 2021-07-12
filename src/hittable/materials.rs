use rand::Rng;
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
    fn scatter(&self, _ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
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
    pub albedo: Color,
    pub fuzz: f64
}

fn reflect(v: Vector3, n: Vector3) -> Vector3 {
    v - 2.0 * n.dot(v) * n
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let reflected: Vector3 = reflect(ray.direction.normalized(), hit_record.normal);
        if reflected.dot(hit_record.normal) > 0.0 {
            let scattered = Ray::new(hit_record.p, reflected + self.fuzz.abs() * vec3::random_in_unit_sphere());
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub refraction_index: f64
}

fn refract(uv: Vector3, n: Vector3, etai_over_etat: f64) -> Vector3 {
    let cos_theta = n.dot(-uv).min(1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = (1.0 - r_out_perp.length_squared()).sqrt() * -n;

    r_out_perp + r_out_parallel
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Ray, Color)> {
        let refraction_ratio = if hit_record.front_face {1.0/self.refraction_index} else {self.refraction_index};

        let unit_direction = ray.direction.normalized();
        let cos_theta = hit_record.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        //reflects if it's not possible to refract
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::thread_rng().gen_range(0.0..1.0) {
            let reflected = reflect(unit_direction, hit_record.normal);
            Some((Ray::new(hit_record.p, reflected), Color::new(1.0, 1.0, 1.0)))
        } else {
            let refracted = refract(unit_direction, hit_record.normal, refraction_ratio);
            Some((Ray::new(hit_record.p, refracted), Color::new(1.0, 1.0, 1.0)))
        }
    }
}
