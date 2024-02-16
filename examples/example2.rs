use std::f64::consts::PI;

use raytracer::matrix::Matrix4;
use raytracer::{Canvas, Color};
use raytracer::{point, tuple::Tuple};

fn main() {
    let origin = point!(0., 0., 0.);
    let radius = 10.;
    let h = 200.;
    let w = 200.;
    let color = Color {
        red: 1.,
        green: 1.,
        blue: 1.,
    };
    let mut canvas = Canvas::new(w as usize, h as usize);
    let origin_x = w / 2.;
    let origin_y = h / 2.;

    canvas.write_pixel(origin_x as usize, origin_y as usize, color);

    let mut point = Matrix4::translate(0., 0., radius) * origin;


    canvas.write_pixel((origin_x + point.x) as usize, (origin_y - point.z) as usize, color.clone());

    for _ in 0..12 {
        point = Matrix4::rotation_y(PI / 6.) * point;
        canvas.write_pixel((origin_x + point.x) as usize, (origin_y - point.z) as usize, color.clone());
    }

    

    println!("{}", canvas.save());
}
