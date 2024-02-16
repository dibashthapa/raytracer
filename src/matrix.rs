use crate::tuple::Tuple;
use core::ops::{Add, Div, Mul, Sub};
use std::f64::consts::PI;

#[derive(Debug, PartialEq)]
pub struct Matrix2([[f64; 2]; 2]);

impl Matrix2 {
    fn determinant(&self) -> f64 {
        self.0[0][0] * self.0[1][1] - self.0[1][0] * self.0[0][1]
    }
}

#[derive(Debug, PartialEq)]
pub struct Matrix3([[f64; 3]; 3]);

impl Matrix3 {
    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        if (row + column) % 2 != 0 {
            return -self.minor(row, column);
        }

        self.minor(row, column)
    }

    fn submatrix(&self, row: usize, column: usize) -> Matrix2 {
        let mut output = Matrix2([[0.; 2]; 2]);

        let mut out_row = 0;
        for (r_index, r) in self.0.iter().enumerate() {
            if r_index == row {
                continue;
            };
            let mut out_col = 0;
            for (c_index, c) in r.iter().enumerate() {
                if c_index == column {
                    continue;
                };

                output.0[out_row][out_col] = *c;
                out_col += 1;
            }
            out_row += 1;
        }

        output
    }

    fn determinant(&self) -> f64 {
        let mut det = 0.;

        for column in 0..3 {
            det += self.0[0][column] * self.cofactor(0, column);
        }

        det
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Matrix4(pub [[f64; 4]; 4]);

impl Matrix4 {
    pub fn inverse(&self) -> Result<Self, ()> {
        let mut m = Matrix4([[0.; 4]; 4]);

        if self.determinant() == 0. {
            return Err(());
        }

        for row in 0..self.0.len() {
            for col in 0..self.0.len() {
                let c = self.cofactor(row, col);
                m.0[col][row] = c / self.determinant();
            }
        }

        Ok(m)
    }

    fn translate(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4([
            [1.0, 0., 0., x],
            [0., 1., 0., y],
            [0., 0., 1., z],
            [0., 0., 0., 1.],
        ])
    }

    fn scaling(x: f64, y: f64, z: f64) -> Matrix4 {
        Matrix4([
            [x, 0., 0., 0.],
            [0., y, 0., 0.],
            [0., 0., z, 0.],
            [0., 0., 0., 1.],
        ])
    }

    fn shearing(a: f64, b: f64, c: f64, d: f64, e: f64, f: f64) -> Matrix4 {
        Matrix4([
            [1., a, b, 0.],
            [c, 1., d, 0.],
            [e, f, 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    fn transpose(&self) -> Self {
        let mut output = Self([[0.; 4]; 4]);
        for row in 0..4 {
            for col in 0..4 {
                output.0[col][row] = self.0[row][col];
            }
        }

        output
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        self.submatrix(row, column).determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        if (row + column) % 2 != 0 {
            return -self.minor(row, column);
        }

        self.minor(row, column)
    }

    fn rotation_x(r: f64) -> Matrix4 {
        Matrix4([
            [1., 0., 0., 0.],
            [0., r.cos(), -r.sin(), 0.],
            [0., r.sin(), r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    fn rotation_y(r: f64) -> Matrix4 {
        Matrix4([
            [r.cos(), 0., r.sin(), 0.],
            [0., 1., 0., 0.],
            [-r.sin(), 0., r.cos(), 0.],
            [0., 0., 0., 1.],
        ])
    }

    fn rotation_z(r: f64) -> Matrix4 {
        Matrix4([
            [r.cos(), -r.sin(), 0., 0.],
            [r.sin(), r.cos(), 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        ])
    }

    fn submatrix(&self, row: usize, column: usize) -> Matrix3 {
        let mut output = Matrix3([[0.; 3]; 3]);

        let mut out_row = 0;
        for (r_index, r) in self.0.iter().enumerate() {
            if r_index == row {
                continue;
            };
            let mut out_col = 0;
            for (c_index, c) in r.iter().enumerate() {
                if c_index == column {
                    continue;
                };

                output.0[out_row][out_col] = *c;
                out_col += 1;
            }
            out_row += 1;
        }

        output
    }

    fn determinant(&self) -> f64 {
        let mut det = 0.;

        for column in 0..4 {
            det += self.0[0][column] * self.cofactor(0, column);
        }

        det
    }
}

impl Mul for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut output = Matrix4([[0.; 4]; 4]);
        for row in 0..4 {
            for col in 0..4 {
                output.0[row][col] = self.0[row][0] * rhs.0[0][col]
                    + self.0[row][1] * rhs.0[1][col]
                    + self.0[row][2] * rhs.0[2][col]
                    + self.0[row][3] * rhs.0[3][col]
            }
        }
        output
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        Tuple {
            x: self.0[0][0] * rhs.x
                + self.0[0][1] * rhs.y
                + self.0[0][2] * rhs.z
                + self.0[0][3] * rhs.w,
            y: self.0[1][0] * rhs.x
                + self.0[1][1] * rhs.y
                + self.0[1][2] * rhs.z
                + self.0[1][3] * rhs.w,
            z: self.0[2][0] * rhs.x
                + self.0[2][1] * rhs.y
                + self.0[2][2] * rhs.z
                + self.0[2][3] * rhs.w,
            w: self.0[3][0] * rhs.x
                + self.0[3][1] * rhs.y
                + self.0[3][2] * rhs.z
                + self.0[3][3] * rhs.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::equal;
    use std::f64::consts::PI;

    use crate::{
        matrix::{Matrix2, Matrix3, Matrix4},
        point, test_point,
        tuple::Tuple,
        vector,
    };

    #[test]
    fn multiply4x4() {
        assert_eq!(
            Matrix4([
                [1., 2., 3., 4.],
                [5., 6., 7., 8.],
                [9., 8., 7., 6.],
                [5., 4., 3., 2.]
            ]) * Matrix4([
                [-2., 1., 2., 3.],
                [3., 2., 1., -1.],
                [4., 3., 6., 5.],
                [1., 2., 7., 8.]
            ]),
            Matrix4([
                [20., 22., 50., 48.],
                [44., 54., 114., 108.],
                [40., 58., 110., 102.],
                [16., 26., 46., 42.]
            ])
        );
    }

    #[test]
    fn multply_tuple() {
        assert_eq!(
            Matrix4([
                [1., 2., 3., 4.],
                [2., 4., 4., 2.],
                [8., 6., 4., 1.],
                [0., 0., 0., 1.]
            ]) * Tuple {
                x: 1.,
                y: 2.,
                z: 3.,
                w: 1.
            },
            Tuple {
                x: 18.,
                y: 24.,
                z: 33.,
                w: 1.
            }
        );
    }

    #[test]
    fn identity_matrix() {
        assert_eq!(
            Matrix4([
                [0., 1., 2., 4.],
                [1., 2., 4., 8.],
                [2., 4., 8., 16.],
                [4., 8., 16., 32.]
            ]) * Matrix4([
                [1., 0., 0., 0.],
                [0., 1., 0., 0.],
                [0., 0., 1., 0.],
                [0., 0., 0., 1.]
            ]),
            Matrix4([
                [0., 1., 2., 4.],
                [1., 2., 4., 8.],
                [2., 4., 8., 16.],
                [4., 8., 16., 32.]
            ])
        );
    }

    #[test]
    fn transpose_matrix() {
        assert_eq!(
            Matrix4([
                [0., 9., 3., 0.],
                [9., 8., 0., 8.],
                [1., 8., 5., 3.],
                [0., 0., 5., 8.]
            ])
            .transpose(),
            Matrix4([
                [0., 9., 1., 0.],
                [9., 8., 8., 0.],
                [3., 0., 5., 5.],
                [0., 8., 3., 8.]
            ])
        )
    }
    #[test]
    fn submatrix3x3() {
        assert_eq!(
            Matrix3([[1., 5., 0.], [-3., 2., 7.], [0., 6., -3.]]).submatrix(0, 2),
            Matrix2([[-3., 2.], [0., 6.]])
        );
    }

    #[test]
    fn submatrix4x4() {
        assert_eq!(
            Matrix4([
                [-6., 1., 1., 6.],
                [-8., 5., 8., 6.],
                [-1., 0., 8., 2.],
                [-7., 1., -1., 1.]
            ])
            .submatrix(2, 1),
            Matrix3([[-6., 1., 6.], [-8., 8., 6.], [-7., -1., 1.]])
        );
    }

    #[test]
    fn test_minor() {
        assert_eq!(
            Matrix3([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]).minor(1, 0),
            25.
        );
    }

    #[test]
    fn test_determinant() {
        assert_eq!(Matrix2([[1., 5.], [-3., 2.]]).determinant(), 17.);
    }

    #[test]
    fn test_cofactor() {
        let a = Matrix3([[3., 5., 0.], [2., -1., -7.], [6., -1., 5.]]);
        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);
        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn test_3x3() {
        let a = Matrix3([[1., 2., 6.], [-5., 8., -4.], [2., 6., 4.]]);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn test_4x4() {
        let a = Matrix4([
            [-2., -8., 3., 5.],
            [-3., 1., 7., 3.],
            [1., 2., -9., 6.],
            [-6., 7., 7., -9.],
        ]);
        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn test_inverse() {
        let a = Matrix4([
            [8., -5., 9., 2.],
            [7., 5., 6., 1.],
            [-6., 0., 9., 6.],
            [-3., 0., -9., -4.],
        ]);

        assert_eq!(
            a.inverse(),
            Ok(Matrix4([
                [
                    -0.15384615384615385,
                    -0.15384615384615385,
                    -0.28205128205128205,
                    -0.5384615384615384
                ],
                [
                    -0.07692307692307693,
                    0.12307692307692308,
                    0.02564102564102564,
                    0.03076923076923077
                ],
                [
                    0.358974358974359,
                    0.358974358974359,
                    0.4358974358974359,
                    0.9230769230769231
                ],
                [
                    -0.6923076923076923,
                    -0.6923076923076923,
                    -0.7692307692307693,
                    -1.9230769230769231
                ]
            ]))
        );
    }

    #[test]
    fn inverse_by_matrix() {
        let a = Matrix4([
            [3., -9., 7., 3.],
            [3., -8., 2., -9.],
            [-4., 4., 4., 1.],
            [-6., 5., -1., 1.],
        ]);
        let b = Matrix4([
            [8., 2., 2., 2.],
            [3., -1., 7., 0.],
            [7., 0., 5., 4.],
            [6., -2., 0., 5.],
        ]);

        let c = a * b;

        let b_inverse = b.inverse().unwrap();

        let product = c * b_inverse;

        let epsilon = 1e-6;

        for i in 0..4 {
            for j in 0..4 {
                assert!(
                    (product.0[i][j] - a.0[i][j]).abs() < epsilon,
                    "Element ({}, {}) differs more than epsilon: {}",
                    i,
                    j,
                    epsilon
                );
            }
        }
    }

    #[test]
    fn test_translation() {
        let transform = Matrix4::translate(5., -3., 2.);
        let p = point!(-3., 4., 5.);

        assert_eq!(transform * p, point!(2., 1., 7.));
    }

    #[test]
    fn test_inverse_translation() {
        let transform = Matrix4::translate(5., -3., 2.);
        let inv = transform.inverse();
        let p = point!(-3., 4., 5.);
        assert_eq!(inv.unwrap() * p, point!(-8., 7., 3.));
    }

    #[test]
    fn test_translation_doesnt_affect_vector() {
        let transform = Matrix4::translate(5., -3., 2.);
        let v = vector!(-3., 4., 5.);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn test_scaling_matrix_applied_point() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let p = point!(-4., 6., 8.);
        assert_eq!(transform * p, point!(-8., 18., 32.));
    }

    #[test]
    fn test_scaling_matrix_applied_vector() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let v = vector!(-4., 6., 8.);
        assert_eq!(transform * v, vector!(-8., 18., 32.));
    }

    #[test]
    fn test_scaling_inverse_scaling_matrix() {
        let transform = Matrix4::scaling(2., 3., 4.);
        let inv = transform.inverse().unwrap();
        let v = vector!(-4., 6., 8.);
        assert_eq!(inv * v, vector!(-2., 2., 2.));
    }

    #[test]
    fn test_reflection_is_scaling() {
        let transform = Matrix4::scaling(-1., 1., 1.);
        let p = point!(2., 3., 4.);
        assert_eq!(transform * p, point!(-2., 3., 4.));
    }

    #[test]
    fn test_rotation_around_x_axis() {
        let point = point!(0., 1., 0.);
        let half_quarter = Matrix4::rotation_x(PI / 4.);
        let full_quarter = Matrix4::rotation_x(PI / 2.);
        let p = full_quarter * point;
        assert!(equal(p.x, 0.));
        assert!(equal(p.y, 0.));
        assert!(equal(p.z, 1.));

        let p = half_quarter * point;
        assert!(equal(p.x, 0.));
        assert!(equal(p.y, 2_f64.sqrt() / 2.));
        assert!(equal(p.z, 2_f64.sqrt() / 2.));
    }

    #[test]
    fn test_rotation_opposite_direction() {
        let point = point!(0., 1., 0.);
        let half_quarter = Matrix4::rotation_x(PI / 4.);
        let inv = half_quarter.inverse().unwrap();

        let v = inv * point;
        assert!(equal(v.x, 0.));
        assert!(equal(v.y, 2_f64.sqrt() / 2.));
        assert!(equal(v.z, -2_f64.sqrt() / 2.));
    }

    #[test]
    fn test_rotation_around_y_axis() {
        let point = point!(0., 0., 1.);
        let half_quarter = Matrix4::rotation_y(PI / 4.);
        let full_quarter = Matrix4::rotation_y(PI / 2.);

        let p = half_quarter * point;
        assert!(equal(p.x, 2_f64.sqrt() / 2.));
        assert!(equal(p.y, 0.));
        assert!(equal(p.z, 2_f64.sqrt() / 2.));
        let p = full_quarter * point;
        assert!(equal(p.x, 1.));
        assert!(equal(p.y, 0.));
        assert!(equal(p.z, 0.));
    }

    #[test]
    fn test_rotation_around_z_axis() {
        let point = point!(0., 1., 0.);
        let half_quarter = Matrix4::rotation_z(PI / 4.);
        let full_quarter = Matrix4::rotation_z(PI / 2.);

        let p = half_quarter * point;
        assert!(equal(p.x, -2_f64.sqrt() / 2.));
        assert!(equal(p.y, 2_f64.sqrt() / 2.));
        assert!(equal(p.z, 0.));
        let p = full_quarter * point;
        assert!(equal(p.x, -1.));
        assert!(equal(p.y, 0.));
        assert!(equal(p.z, 0.));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix4::shearing(1., 0., 0., 0., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(transform * p, point!(5., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix4::shearing(0., 1., 0., 0., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(transform * p, point!(6., 3., 4.));
    }

    #[test]
    fn shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix4::shearing(0., 0., 0., 1., 0., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(transform * p, point!(2., 7., 4.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix4::shearing(0., 0., 0., 0., 1., 0.);
        let p = point!(2., 3., 4.);
        assert_eq!(transform * p, point!(2., 3., 6.));
    }

    #[test]
    fn shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix4::shearing(0., 0., 0., 0., 0., 1.);
        let p = point!(2., 3., 4.);
        assert_eq!(transform * p, point!(2., 3., 7.));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = point!(1., 0., 1.);
        let a = Matrix4::rotation_x(PI / 2.);
        let b = Matrix4::scaling(5., 5., 5.);
        let c = Matrix4::translate(10., 5., 7.);

        let p2 = a * p;
        test_point!(p2, point!(1., -1., 0.));

        let p3 = b * p2;
        test_point!(p3, point!(5., -5., 0.));

        let p4 = c * p3;
        test_point!(p4, point!(15., 0., 7.));
    }

    #[test]
    fn chained_transformations_must_be_in_reverse_order() {
        let p = point!(1., 0., 1.);
        let a = Matrix4::rotation_x(PI / 2.);
        let b = Matrix4::scaling(5., 5., 5.);
        let c = Matrix4::translate(10., 5., 7.);
        let t = c * b * a;

        test_point!(t * p, point!(15., 0., 7.));
    }
}
