use raytracer::intersection::Intersections;
use raytracer::ray::{Ray, Sphere};
use raytracer::{point, tuple::Tuple, Canvas, Color};

fn main() {
    let ray_origin = point!(0.0, 0.0, -5.0);
    let wall_position_z = 5.0;
    let wall_size = 10.0;

    let canvas_size = 400;
    let canvas_pixel_world_size = wall_size / canvas_size as f64;

    let yellow = Color {
        red: 1.0,
        blue: 1.0,
        green: 0.0,
    };

    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let sphere = Sphere::default();

    println!(
        "Raytracing {} pixels. Please be patient...",
        canvas_size.pow(2)
    );

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
                canvas.write_pixel(x, y, yellow);
            }
        }
    }
    canvas.save();
}
