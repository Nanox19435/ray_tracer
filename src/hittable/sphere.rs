use super::{
    super::structs::{vec3::Vector3, ray::Ray},
    HitRecord};


pub struct Sphere {
    center: Vector3,
    radius: f64
}

impl Sphere {
    pub fn new(center: Vector3, radius: f64) -> Sphere {
        Sphere {center, radius}
    }
}

impl super::Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord>{
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {return None;}

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd)/a;
            if root < t_min || t_max < root {return None;}
        }

        //If something does not work, maybe the problem is here.
        // hit = Some(super::HitRecord {p: ray.at(1.0)})
        let t = root;
        let p = ray.at(t);
        let normal = (p - self.center) / self.radius;

        let mut hit = super::HitRecord {t, normal, p, front_face:false};
        hit.set_face_normal(ray, normal);

        Some(hit)
        // let outward_normal = (rec.p - self.center) / self.radius;
        // rec.set_face_normal(ray, outward_normal);

    }
}
