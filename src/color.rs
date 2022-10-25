#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn red() -> Color {
        Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0.0,
            g: 1.0,
            b: 0.0,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 1.0,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn white() -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        }
    }

    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Color {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl std::ops::Sub for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Color {
        Color::new(self.r() - rhs.r(), self.g() - rhs.g(), self.b() - rhs.b())
    }
}

impl std::ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Color {
        Color::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_red_green_and_blue() {
        let c = Color::new(-0.5, 0.4, 1.7);
        assert_eq!(c.r(), -0.5);
        assert_eq!(c.g(), 0.4);
        assert_eq!(c.b(), 1.7);
    }

    #[test]
    fn adding_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 + c2;
        assert_eq!(c3.r(), 1.6);
        assert_eq!(c3.g(), 0.7);
        assert_eq!(c3.b(), 1.0);
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let c3 = c1 - c2;
        assert!(crate::utils::equal(c3.r(), 0.2));
        assert_eq!(c3.g(), 0.5);
        assert_eq!(c3.b(), 0.5);
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        let c2 = c * 2.0;
        assert_eq!(c2.r(), 0.4);
        assert_eq!(c2.g(), 0.6);
        assert_eq!(c2.b(), 0.8);
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let c3 = c1 * c2;
        assert_eq!(c3.r(), 0.9);
        assert_eq!(c3.g(), 0.2);
        assert!(crate::utils::equal(c3.b(), 0.04));
    }
}
