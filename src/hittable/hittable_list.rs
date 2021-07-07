use std::rc::Rc;

pub struct HittableList {
    objects: Vec<Rc<dyn super::Hittable>>
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {objects: Vec::<Rc<dyn super::Hittable>>::new()}
    }

    pub fn clear(mut self) { self.objects.clear(); }
    pub fn add(&mut self, object: Rc<dyn super::Hittable>) {self.objects.push(object);}

}

impl super::Hittable for HittableList {
    fn hit(&self, ray: &super::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let mut temp_record: Option<super::HitRecord> = None;
        let mut closest_so_far = t_max;

        //Iterates over all physical objects searching for ray collisions.
        for object in self.objects.iter() {
            if let Some(hit) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hit.t;
                temp_record = Some(hit);
            }
        }

        temp_record
    }
}
