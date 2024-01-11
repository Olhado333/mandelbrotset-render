use crate::complex::Complex;

pub fn escape_time(c: &Complex, precision: u32) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);

    for i in 0..precision {
        z = z.square().add(&c);

        if z.magnitude() > 3.0 {
            return Some(i);
        }
    }

    None
}