use raytracer::matrix::Matrix4;
fn main() {
    let a = Matrix4([
        [1., 2., 3., 4.],
        [2., 4., 1., 0.],
        [0., 0., 1., 6.],
        [0., 4., 2., 1.],
    ]);

    dbg!(a * a.inverse().unwrap());
}
