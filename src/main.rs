use std::rc::Rc;
use rand::Rng;

mod structs;

//redename for convinience
use structs::vec3::Vector3;
use structs::vec3::Vector3 as Color;

use structs::ray::Ray;

mod hittable;
use hittable::{hittable_list::HittableList, sphere::Sphere};

mod camera;

fn ray_color(ray: &Ray, world: &dyn hittable::Hittable) -> Color {
    if let Some(hit) = world.hit(ray, 0.0, f64::INFINITY) {
        0.5*(hit.normal + Color::new(1.0, 1.0, 1.0))
    } else {
        let unit_direction = ray.direction.normalized();
        let t = 0.5*(unit_direction.y + 1.0);

        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}


fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16.0/9.0;
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / ASPECT_RATIO) as u32;
    let samples_per_pixel: u8 = 75;

    //World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vector3::new(0.0,0.0,-1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vector3::new(0.0,-100.5,-1.0), 100.0)));

    //Camera
    let camera = camera::Camera::default();

    //Render
    println!("P3\n{} {}\n255", image_width, image_height);

    for j in (0..image_height).rev() {
        eprint!("\x1B[2J");
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let mut rng = rand::thread_rng();
                let u: f64 = (i as f64 + rng.gen_range(0.0..0.9))/ (image_width - 1) as f64;
                let v: f64 = (j as f64 + rng.gen_range(0.0..0.9))/ (image_height - 1) as f64;

                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }

            structs::vec3::write_color(pixel_color, samples_per_pixel);


        }
    }
}
