extern crate ray_tracer_challenge;
use ray_tracer_challenge::canvas::Canvas;
use ray_tracer_challenge::color::Color;

fn main() {
    let mut cv = Canvas::new(100, 100);

    for y in 0..cv.height() {
        for x in 0..cv.width() {
            let u = x as f64 / cv.width() as f64;
            let v = y as f64 / cv.height() as f64;
            let color = Color::new(u, v, 0.5);
            cv.write_pixel(x, y, color);
        }
    }

    cv.save_to_ppm("./images/projectile_chapter_2.ppm");
}
