use raytracer::{
    intersection::Intersections,
    point,
    ray::{Ray, Sphere},
    tuple::Tuple,
    Canvas, Color,
};

fn main() {
    let ray_origin = point!(0., 0., -5.);
    let wall_position_z = 5.;
    let wall_size = 10.;

    let canvas_size = 100;
    let canvas_pixel_world_size = wall_size / canvas_size as f64;
    let mut canvas = Canvas::new(canvas_size, canvas_size);
    let color = Color {
        red: 1.,
        blue: 0.,
        green: 0.,
    };

    let sphere = Sphere::default();

    for y in 0..canvas_size {
        for x in 0..canvas_size {
            let half = wall_size / 2.;
            let world_x = -half + (x as f64) * canvas_pixel_world_size;
            let world_y = half - (y as f64) * canvas_pixel_world_size;
            let wall_point = point!(world_x, world_y, wall_position_z);
            let mut ray = Ray {
                origin: ray_origin,
                direction: (wall_point - ray_origin).normalize(),
            };
            let xs = Intersections(sphere.intersect(ray));

            if xs.hit().is_some() {
                canvas.write_pixel(x, y, color);
            }
        }
    }

    canvas.save();
}
