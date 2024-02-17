use crate::intersection::Intersections;
use crate::matrix::Matrix4;
use crate::tuple::Tuple;
use crate::{intersection::Intersection, point};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Matrix4::identity())
    }
}

impl Sphere {
    fn new(transform: Matrix4) -> Self {
        Self { transform }
    }

    pub fn intersect(&self, mut r: Ray) -> Intersections {
        let ray = r.transform(self.transform.inverse().unwrap());
        let sphere_to_ray = ray.origin - point!(0., 0., 0.);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            Intersections(vec![])
        } else {
            let t1 = Intersection {
                t: (-b - discriminant.sqrt()) / (2.0 * a),
                object: *self,
            };
            let t2 = Intersection {
                t: (-b + discriminant.sqrt()) / (2.0 * a),
                object: *self,
            };

            Intersections(vec![t1, t2])
        }
    }

    fn set_transform(&mut self, t: Matrix4) {
        self.transform = t;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn position(&self, time: f64) -> Tuple {
        self.origin + self.direction * time
    }

    pub fn transform(&mut self, matrix: Matrix4) -> Self {
        Self {
            origin: matrix * self.origin,
            direction: matrix * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ matrix::Matrix4, point, tuple::Tuple, vector};

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

        let xs = s.intersect(r);

        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].t, 4.);
        assert_eq!(xs.0[1].t, 6.);
    }

    #[test]
    fn test_ray_intersects_at_tangent() {
        let r = Ray {
            origin: point!(0., 1., -5.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].t, 5.);
        assert_eq!(xs.0[1].t, 5.);
    }

    #[test]
    fn test_ray_misses_a_sphere() {
        let r = Ray {
            origin: point!(0., 2., -5.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 0);
    }

    #[test]
    fn test_ray_originates_inside_sphere() {
        let r = Ray {
            origin: point!(0., 0., 0.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].t, -1.);
        assert_eq!(xs.0[1].t, 1.);
    }

    #[test]
    fn test_sphere_behind_ray() {
        let r = Ray {
            origin: point!(0., 0., 5.),
            direction: vector!(0., 0., 1.),
        };
        let s = Sphere::default();

        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].t, -6.);
        assert_eq!(xs.0[1].t, -4.);
    }

    #[test]
    fn test_translating_ray() {
        let mut r = Ray {
            origin: point!(1., 2., 3.),
            direction: vector!(0., 1., 0.),
        };
        let m = Matrix4::translate(3., 4., 5.);
        let r = r.transform(m);
        assert_eq!(r.origin, point!(4., 6., 8.));
        assert_eq!(r.direction, vector!(0., 1., 0.));
    }

    #[test]
    fn test_scaling_ray() {
        let mut r = Ray {
            origin: point!(1., 2., 3.),
            direction: vector!(0., 1., 0.),
        };
        let m = Matrix4::scaling(2., 3., 4.);
        let r = r.transform(m);
        assert_eq!(r.origin, point!(2., 6., 12.));
        assert_eq!(r.direction, vector!(0., 3., 0.));
    }
    #[test]
    fn sphere_default_transformation() {
        let s = Sphere::default();
        assert_eq!(s.transform, Matrix4::identity());
    }

    #[test]
    fn changing_sphere_transformation() {
        let mut s = Sphere::default();
        let t = Matrix4::translate(2., 3., 4.);
        s.set_transform(t);
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_scaled_sphere_ray() {
        let r = Ray {
            origin: point!(0., 0., -5.),
            direction: vector!(0., 0., 1.),
        };

        let mut s = Sphere::default();
        s.set_transform(Matrix4::scaling(2., 2., 2.));
        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].t, 3.);
        assert_eq!(xs.0[1].t, 7.);
    }

    #[test]
    fn intersecting_translated_sphere_with_ray() {
        let r = Ray {
            origin: point!(0., 0., -5.),
            direction: vector!(0., 0., 1.),
        };
        let mut s = Sphere::default();
        s.set_transform(Matrix4::translate(5., 0., 0.));
        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 0);
    }
}
