use crate::point;
use crate::tuple::Tuple;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
struct Sphere {
    id: u64,
    center: Tuple,
    radius: f64,
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(point!(0., 0., 0.), 1.)
    }
}

impl Sphere {
    fn new(center: Tuple, radius: f64) -> Self {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self { id, center, radius }
    }
}

#[derive(Debug, Clone, Copy)]
struct Ray {
    origin: Tuple,
    direction: Tuple,
}

impl Ray {
    fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    fn intersects(&self, s: Sphere) -> Vec<f64> {
        let sphere_to_ray = self.origin - s.center;
        let a = self.direction.dot(self.direction);
        let b = 2.0 * sphere_to_ray.dot(self.direction);
        let c = sphere_to_ray.dot(sphere_to_ray) - s.radius * s.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![t1, t2]
        }
    }
}

fn sphere() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as f64
}

#[cfg(test)]
mod tests {
    use crate::{point, tuple::Tuple, vector};

    use super::{Ray, Sphere};

    #[test]
    fn test_point_from_distance() {
        let r = Ray {
            origin: point!(2., 3., 4.),
            direction: vector!(1., 0., 0.),
        };
        assert_eq!(r.position(0.), point!(2., 3., 4.));
        assert_eq!(r.position(1.), point!(3., 3., 4.));
        assert_eq!(r.position(-1.), point!(1., 3., 4.));
        assert_eq!(r.position(2.5), point!(4.5, 3., 4.));
    }

    #[test]
    fn test_ray_intersects_at_two_points() {
        let r = Ray {
            origin: point!(0., 0., -5.),
            direction: vector!(0., 0., 1.),
        };

        let s = Sphere::default();

        let xs = r.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 4.);
        assert_eq!(xs[1], 6.);
    }

    #[test]
    fn test_ray_intersects_at_tangent() {
        let r = Ray {
            origin: point!(0., 1., -5.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();
        let xs = r.intersects(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], 5.);
        assert_eq!(xs[1], 5.);
    }

    #[test]
    fn test_ray_misses_a_sphere() {
        let r = Ray {
            origin: point!(0., 2., -5.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();
        let xs = r.intersects(s);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        let r = Ray {
            origin: point!(0., 0., 0.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();
        let xs = r.intersects(s);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -1.);
        assert_eq!(xs[1], 1.);
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray {
            origin: point!(0., 0., 5.),
            direction: vector!(0., 0., 1.),
        };

        let xs = r.intersects(Sphere::default());
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0], -6.);
        assert_eq!(xs[1], -4.);
    }
}
