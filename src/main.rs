extern crate num;
use num::complex::Complex64 as complex;

struct Oscillator {
    nosc: f64,
    en:   f64,
    eg:   f64,
    nu:   f64,
    phi:  f64,
}

struct HODispersion {
    oscillators: Vec<Oscillator>
}

trait dispersion {
    fn getN(&self, wavelength: f64) -> complex;
}

impl dispersion for HODispersion {
    fn getN(&self, wavelength: f64) -> complex {
        self.oscillators.iter().fold(complex{re: 0.0, im: 0.0}, |n,osc| n + osc.nosc)
    }
}

fn main() {
    println!("Hello!");
}
