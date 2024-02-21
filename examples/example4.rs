use raytracer::material::PointLight;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::{point, tuple::Tuple, Canvas, Color};

fn main() {
    let ray_origin = point!(0.0, 0.0, -5.0);
    let wall_position_z = 5.0;
    let wall_size = 10.0;

    let canvas_size = 1200;
    let canvas_pixel_world_size = wall_size / canvas_size as f64;

    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let mut sphere = Sphere::default();
    sphere.material.color = Color {
        red: 1.6,
        blue: 0.94,
        green: 0.125,
    };

    let light_position = point!(-10., 10., -10.);
    let light_color = Color {
        red: 1.,
        blue: 1.,
        green: 1.,
    };

    for y in 0..canvas_size {
        for x in 0..canvas_size {
            let half = wall_size / 2.0;
            let world_x = -half + (x as f64) * canvas_pixel_world_size;
            let world_y = half - (y as f64) * canvas_pixel_world_size;

            let wall_point = point!(world_x, world_y, wall_position_z);
            let ray = Ray {
                origin: ray_origin,
                direction: (wall_point - ray_origin).normalize(),
            };

            let xs = sphere.intersect(ray);

            if xs.hit() != None {
                let light = PointLight::new(light_position, light_color);

                let hit_object = xs.hit().unwrap();
                let point = ray.position(hit_object.t);
                let normal = hit_object.object.normal_at(point);
                let eye = -ray.direction;
                let color = hit_object
                    .object
                    .material
                    .lightning(light, point, eye, normal);
                canvas.write_pixel(x, y, color);
            }
        }
    }
    canvas.save();
}
