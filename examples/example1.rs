use raytracer::{point, tuple::Tuple, vector};

#[derive(Debug, Copy, Clone)]
struct Projectile {
    position: Tuple,
    velocity: Tuple,
}

struct Environment {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Environment, proj: &Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity;
    Projectile { position, velocity }
}

fn main() {
    let mut proj = Projectile {
        position: point!(0., 1., 0.),
        velocity: vector!(1., 1., 0.).normalize() * 15.2,
    };

    let env = Environment {
        gravity: vector!(0., -0.1, 0.),
        wind: vector!(-0.01, 0., 0.),
    };

    let mut count = 0;

    while proj.position.y > 0. {
        proj = tick(&env, &proj);
        count += 1;
        println!("Tick {} {:#?}", count, proj.position);
    }
}
