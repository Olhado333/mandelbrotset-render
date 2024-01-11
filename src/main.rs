mod complex;
mod mandelbrot;
mod color;

use image::ImageBuffer;
use complex::Complex;
use mandelbrot::escape_time;
use color::color;

fn main() {
    let precision = 500;
    let (width, hieght) = (10000, 10000);
    let scalar = 4.0;

    let img = ImageBuffer::from_fn(width, hieght, |x, y| {
        let (x_alpha, y_alpha): (f64, f64) = (x as f64 / width as f64, y as f64 / hieght as f64);

        let (x_c, y_c): (f64, f64) = (
            x_alpha * scalar - scalar * 0.5,
            y_alpha * scalar - scalar * 0.5,
        );

        let c = Complex::new(x_c, y_c);
        let i = escape_time(&c, precision);
        color(i)
    });

    img.save("test.png").expect("error");
}
