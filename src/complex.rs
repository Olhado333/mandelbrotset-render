pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(a: f64, b: f64) -> Complex {
        Complex { real: a, imag: b }
    }

    pub fn format(&self) -> String {
        if self.imag == 0.0 {
            format!("{}", self.real)
        } else if self.real == 0.0 {
            format!("{}i", self.imag)
        } else if self.imag < 0.0 {
            format!("{} - {}i", self.real, self.imag)
        } else {
            format!("{} + {}i", self.real, self.imag)
        }
    }

    pub fn add(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }

    pub fn sub(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real - other.real,
            imag: self.imag - other.imag,
        }
    }

    pub fn mul(&self, other: &Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.imag * other.imag,
            imag: self.real * other.imag + self.imag * other.real,
        }
    }

    pub fn square(&self) -> Complex {
        Complex {
            real: self.real * self.real - self.imag * self.imag,
            imag: 2.0 * self.real * self.imag,
        }
    }

    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }
}