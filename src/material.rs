use crate::{tuple::Tuple, Color};

const BLACK: Color = Color {
    red: 0.,
    blue: 0.,
    green: 0.,
};

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

pub struct PointLight {
    position: Tuple,
    intensity: Color,
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Material {
    pub fn lightning(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = BLACK;
        let mut specular = BLACK;

        if light_dot_normal < 0. {
            diffuse = BLACK;
            specular = BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = -lightv.reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye <= 0. {
                specular = BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color {
                red: 1.,
                blue: 1.,
                green: 1.,
            },
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{equal, point, test_color, tuple::Tuple, vector, Color};

    use super::{Material, PointLight};

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::default();
        let position = point!(0., 0., 0.);
        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight {
            position: point!(0., 0., -10.),
            intensity: Color {
                red: 1.,
                blue: 1.,
                green: 1.,
            },
        };

        let result = m.lightning(light, position, eyev, normalv);
        assert_eq!(
            result,
            Color {
                red: 1.9,
                blue: 1.9,
                green: 1.9
            }
        );
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_offset_45() {
        let m = Material::default();
        let position = point!(0., 0., 0.);
        let eyev = vector!(0., 2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight {
            position: point!(0., 0., -10.),
            intensity: Color {
                red: 1.,
                blue: 1.,
                green: 1.,
            },
        };

        let result = m.lightning(light, position, eyev, normalv);
        assert_eq!(
            result,
            Color {
                red: 1.,
                blue: 1.,
                green: 1.
            }
        );
    }

    #[test]
    fn lightning_with_eye_opposite_surface_offset45() {
        let m = Material::default();
        let position = point!(0., 0., 0.);
        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight {
            position: point!(0., 10., -10.),
            intensity: Color {
                red: 1.,
                blue: 1.,
                green: 1.,
            },
        };
        let result = m.lightning(light, position, eyev, normalv);
        test_color!(
            result,
            Color {
                red: 0.7364,
                blue: 0.7364,
                green: 0.7364
            }
        );
    }

    #[test]
    fn lighting_with_eye_in_the_path_of_reflection_vector() {
        let m = Material::default();
        let position = point!(0., 0., 0.);
        let eyev = vector!(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(
            point!(0., 10., -10.),
            Color {
                red: 1.,
                blue: 1.,
                green: 1.,
            },
        );

        let result = m.lightning(light, position, eyev, normalv);
        test_color!(
            result,
            Color {
                red: 1.6364,
                blue: 1.6364,
                green: 1.6364
            }
        );
    }

    #[test]
    fn test_lighting_with_light_behind_the_surface() {
        let m = Material::default();
        let position = point!(0., 0., 0.);
        let eyev = vector!(0., 0., -1.);
        let normalv = vector!(0., 0., -1.);
        let light = PointLight::new(
            point!(0., 0., 10.),
            Color {
                red: 1.,
                blue: 1.,
                green: 1.,
            },
        );

        let result = m.lightning(light, position, eyev, normalv);
        assert_eq!(
            result,
            Color {
                red: 0.1,
                blue: 0.1,
                green: 0.1
            }
        );
    }
}
