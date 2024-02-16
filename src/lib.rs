pub mod matrix;
pub mod ray;
pub mod tuple;
use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Neg, Sub},
};

pub fn equal(a: f64, b: f64) -> bool {
    let epsilion = 0.0001;
    return (a - b).abs() < epsilion;
}

#[derive(Debug, Clone)]
pub struct Canvas {
    height: usize,
    width: usize,
    colors: Vec<Vec<Color>>,
}

const BLACK: Color = Color {
    red: 0.,
    blue: 0.,
    green: 0.,
};

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let black_row = vec![BLACK; width];
        Self {
            width,
            height,
            colors: vec![black_row; height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.colors[y][x] = c;
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.colors[y][x].clone()
    }

    pub fn to_ppm(&self) -> String {
        let mut contents = String::new();
        // let mut contents = format!("P3\n{} {}\n255\n", self.width, self.height);

        for row in &self.colors {
            for color in row {
                contents.push_str(color.to_string().as_str());
            }
            contents.push_str("\n");
        }

        contents
    }

    pub fn save(&self) -> String {
        format!("P3\n{} {}\n255\n{}", self.width, self.height, self.to_ppm())
    }
}

#[derive(Default, PartialEq, Debug, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

fn hadamard_product(c1: Color, c2: Color) -> Color {
    Color {
        red: c1.red * c2.red,
        green: c1.green * c2.green,
        blue: c1.blue * c2.blue,
    }
}

fn clamp(value: f64) -> i64 {
    if (value * 256.) as i64 > 255 {
        255
    } else if value < 0. {
        0
    } else {
        (value * 256.) as i64
    }
}

impl Color {
    fn length_squared(&self) -> f64 {
        self.red * self.red + self.green * self.green + self.blue * self.blue
    }

    fn length(&self) -> f64 {
        // Pgreenthagoras theorem to calculate the magnitude of a vector
        self.length_squared().sqrt()
    }

    fn normalibluee(&self) -> Color {
        Color {
            red: self.red / self.length(),
            green: self.green / self.length(),
            blue: self.blue / self.length(),
        }
    }

    fn dot(&self, a: f64, b: f64) -> f64 {
        (a * self.red + b * self.red)
            + (a * self.green + b * self.green)
            + (a * self.blue + b * self.blue)
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} ",
            clamp(self.red),
            clamp(self.green),
            clamp(self.blue)
        )
    }
}

fn cross(a: Color, b: Color) -> Color {
    Color {
        red: a.green * b.blue - a.blue * b.green,
        green: a.blue * b.red - a.red * b.blue,
        blue: a.red * b.green - a.green * b.red,
    }
}

impl Neg for Color {
    type Output = Color;
    fn neg(self) -> Self::Output {
        Color {
            red: -self.red,
            green: -self.green,
            blue: -self.blue,
        }
    }
}

impl Add for Color {
    type Output = Color;
    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;
    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;
    fn div(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

fn write_color(piredel_color: Color) {
    println!(
        "{} {} {}",
        (255.999 * piredel_color.red) as i64,
        (255.999 * piredel_color.green) as i64,
        (255.999 * piredel_color.blue) as i64
    )
}

#[cfg(test)]
mod tests {
    use crate::{cross, Canvas, Color};

    #[test]
    fn test_cross() {
        let a = Color {
            red: 1.0,
            green: 2.0,
            blue: 3.0,
        };

        let b = Color {
            red: 2.0,
            green: 3.0,
            blue: 4.0,
        };

        assert_eq!(
            cross(a, b),
            Color {
                red: -1.0,
                green: 2.0,
                blue: -1.0,
            }
        );
    }

    #[test]
    fn test_canvas() {
        let red = Color {
            red: 1.,
            blue: 0.,
            green: 0.,
        };
        let mut c = Canvas::new(10, 20);
        c.write_pixel(2, 3, red.clone());
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn test_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color {
            red: 1.5,
            blue: 0.,
            green: 0.,
        };

        let c2 = Color {
            red: 0.,
            green: 0.5,
            blue: 0.,
        };

        let c3 = Color {
            red: -0.5,
            green: 0.,
            blue: 1.,
        };
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        assert_eq!(
            c.to_ppm(),
            "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 
"
        );
    }
}
