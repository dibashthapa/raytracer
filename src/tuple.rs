use std::ops::{Add, Div, Mul, Neg, Sub};

#[macro_export]
macro_rules! vector {
    ($x: expr, $y: expr, $z: expr) => {
        Tuple {
            x: $x,
            y: $y,
            z: $z,
            w: 0.,
        }
    };
}

#[macro_export]
macro_rules! point {
    ($x: literal, $y: literal, $z: literal) => {
        Tuple {
            x: $x,
            y: $y,
            z: $z,
            w: 1.,
        }
    };
}
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn magnitude(&self) -> f64 {
        ((self.x * self.x) + (self.y * self.y) + (self.z * self.z) + (self.w * self.w)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.magnitude(),
            y: self.y / self.magnitude(),
            z: self.y / self.magnitude(),
            w: self.w / self.magnitude(),
        }
    }

    fn dot(a: Tuple, b: Tuple) -> f64 {
        (a.x * b.x) + (a.y * b.y) + (a.z * b.z) + (a.w * b.w)
    }

    fn cross(a: Tuple, b: Tuple) -> Tuple {
        vector!(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x
        )
    }
}

impl Add for Tuple {
    type Output = Tuple;
    fn add(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: Self) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: 0.0,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, rhs: f64) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Tuple;

    #[test]
    fn test_magnitude() {
        assert_eq!(vector!(0., 0., 1.).magnitude(), 1.);
        assert_eq!(vector!(1., 2., 3.).magnitude(), 14_f64.sqrt());
    }

    #[test]
    fn test_dot() {
        let a = Tuple {
            x: 1.,
            y: 2.,
            z: 3.,
            w: 0.,
        };

        let b = Tuple {
            x: 2.,
            y: 3.,
            z: 4.,
            w: 0.,
        };

        assert_eq!(Tuple::dot(a, b), 20.);
    }

    #[test]
    fn test_normalize() {
        let v = Tuple {
            x: 4.,
            y: 0.,
            z: 0.,
            w: 0.,
        };

        assert_eq!(
            v.normalize(),
            Tuple {
                x: 1.,
                y: 0.,
                z: 0.,
                w: 0.
            }
        );
    }
}
