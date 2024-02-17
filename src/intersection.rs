use crate::ray::Sphere;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

#[derive(Clone, Debug)]
pub struct Intersections(pub Vec<Intersection>);

impl Intersections {
        pub fn hit(&self) -> Option<&Intersection> {
            self.0
                .iter()
                .filter(|i| i.t >= 0.)
                .min_by(|a, b| a.t.partial_cmp(&b.t).unwrap())
        }
}

#[cfg(test)]
mod tests {
    use crate::{
        point,
        ray::{Ray, Sphere},
        tuple::Tuple,
        vector,
    };

    use super::{Intersection, Intersections};

    #[test]
    fn aggregate_intersection() {
        let s = Sphere::default();
        let i1 = Intersection { t: 1., object: s };

        let i2 = Intersection { t: 2., object: s };

        let xs = Intersections(vec![i1, i2]);

        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].t, 1.);
        assert_eq!(xs.0[1].t, 2.);
    }

    #[test]
    fn intersect_sets_the_obejct() {
        let r = Ray {
            origin: point!(0., 0., -5.),
            direction: vector!(0., 0., 1.),
        };
        let s= Sphere::default();
        let xs = s.intersect(r);
        assert_eq!(xs.0.len(), 2);
        assert_eq!(xs.0[0].object, Sphere::default());
        assert_eq!(xs.0[1].object, Sphere::default());
    }

    #[test]
    fn hit_positive_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: 1., object: s };
        let i2 = Intersection { t: 2., object: s };
        let xs = Intersections(vec![i1, i2]);
        assert_eq!(xs.hit(), Some(&i1));
    }

    #[test]
    fn hit_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: 1., object: s };
        let xs = Intersections(vec![i2, i1]);
        assert_eq!(xs.hit(), Some(&i2));
    }

    #[test]
    fn hit_intersections_negative_t() {
        let s = Sphere::default();
        let i1 = Intersection { t: -2., object: s };
        let i2 = Intersection { t: -1., object: s };
        let xs = Intersections(vec![i2, i1]);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn hit_lowest_intersection() {
        let s = Sphere::default();
        let i1 = Intersection { t: 5., object: s };
        let i2 = Intersection { t: 7., object: s };
        let i3 = Intersection { t: -3., object: s };
        let i4 = Intersection { t: 2., object: s };
        let xs = Intersections(vec![i1, i2, i3, i4]);
        assert_eq!(xs.hit(), Some(&i4));
    }
}
