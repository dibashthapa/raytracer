use crate::{
    intersection::{Intersection, Intersections},
    material::Material,
    matrix::Matrix4,
    point,
    ray::Ray,
    tuple::Tuple,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sphere {
    pub transform: Matrix4,
    pub material: Material,
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new(Matrix4::identity())
    }
}

impl Sphere {
    fn new(transform: Matrix4) -> Self {
        Self {
            transform,
            material: Material::default(),
        }
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

    pub fn set_transform(&mut self, t: Matrix4) {
        self.transform = t;
    }

    pub fn normal_at(&self, world_point: Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * world_point;
        let object_normal = object_point - point!(0., 0., 0.);

        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = 0.;
        world_normal.normalize()
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{equal, matrix::Matrix4, point, test_point, tuple::Tuple, vector};

    use super::Sphere;

    #[test]
    fn normal_sphere_on_x_axis() {
        let s = Sphere::default();
        let n = s.normal_at(point!(1., 0., 0.));
        assert_eq!(n, vector!(1., 0., 0.));
    }

    #[test]
    fn normal_is_normalized_vector() {
        let s = Sphere::default();
        let n = s.normal_at(point!(
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.,
            3_f64.sqrt() / 3.
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn compute_normal_translated_sphere() {
        let mut s = Sphere::default();
        s.set_transform(Matrix4::translate(0., 1., 0.));
        let n = s.normal_at(point!(0., 1.70711, -0.70711));
        test_point!(n, vector!(0., 0.70711, -0.70711));
    }

    #[test]
    fn compute_normal_transformed_sphere() {
        let mut s = Sphere::default();
        let m = Matrix4::scaling(1., 0.5, 1.) * Matrix4::rotation_z(PI / 5.);
        s.set_transform(m);
        let n = s.normal_at(point!(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.));
        test_point!(n, vector!(0., 0.97014, -0.24254));
    }
}
