use raytracer::intersection::Intersections;
use raytracer::ray::{Ray, Sphere};
use raytracer::{point, tuple::Tuple, Canvas, Color};

fn main() {
    let canvas_pixels = 100;
    let mut canvas = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color {
        red: 1.,
        blue: 0.,
        green: 0.,
    };

    let sphere = Sphere::default();
    let ray_origin = point!(0., 0., -5.);
    let wall_z = 10.;
    let wall_size = 7.;
    let pixels_size = wall_size / canvas_pixels as f64;

    let half = wall_size / 2.;
    for y in 0..canvas_pixels {
        let world_y = half - pixels_size * y as f64;
        for x in 0..canvas_pixels {
            let world_x = -half + pixels_size * x as f64;
            let position = point!(world_x, world_y, wall_z);
            let ray = Ray {
                origin: ray_origin,
                direction: (position - ray_origin).normalize(),
            };
            let xs = sphere.intersect(ray);
            if Intersections(xs).hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas.save();
}
