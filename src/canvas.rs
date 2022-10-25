use crate::color::Color;

pub struct Canvas {
    width: u32,
    height: u32,
    buffer: Vec<Color>,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        Canvas {
            width,
            height,
            buffer: vec![Color::black(); (width * height) as usize],
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn write_pixel(&mut self, x: u32, y: u32, color: Color) {
        let index = (y * self.width + x) as usize;
        self.buffer[index] = color;
    }

    pub fn pixel_at(&self, x: u32, y: u32) -> Color {
        let index = (y * self.width + x) as usize;
        self.buffer[index]
    }

    pub fn to_ppm(&self) -> String {
        let mut ppm = String::new();
        ppm.push_str(&format!("P3\n{} {}\n255\n", self.width, self.height));
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                if x == 50 {
                    line.push_str("\n");
                }
                let color = self.pixel_at(x, y);
                let r = (color.r() * 255.0).round() as u8;
                let g = (color.g() * 255.0).round() as u8;
                let b = (color.b() * 255.0).round() as u8;
                line.push_str(&format!("{} {} {} ", r, g, b));
            }
            ppm.push_str(&format!("{}\n", line.trim()));
        }
        ppm.push_str("\n");
        return ppm;
    }

    pub fn save_to_ppm(&self, filename: &str) {
        let ppm = self.to_ppm();
        std::fs::write(filename, ppm).unwrap();
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width(), 10);
        assert_eq!(c.height(), 20);
        assert_eq!(c.buffer.len(), 200);
        assert_eq!(c.buffer[0], Color::black());
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);
        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), Color::red());
    }

    #[test]
    fn constructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[0], "P3");
        assert_eq!(lines[1], "5 3");
        assert_eq!(lines[2], "255");
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm = c.to_ppm();

        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);
        let color = Color::new(1.0, 0.8, 0.6);
        for y in 0..2 {
            for x in 0..10 {
                c.write_pixel(x, y, color);
            }
        }

        let ppm = c.to_ppm();

        let lines: Vec<&str> = ppm.lines().collect();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[4],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert_eq!(ppm.chars().last(), Some('\n'));
    }
}
