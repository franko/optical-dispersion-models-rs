extern crate num;
use num::complex::Complex64 as complex;
use std::ops::Mul;

struct Oscillator {
    pub nosc: f64,
    pub en:   f64,
    pub eg:   f64,
    pub phi:  f64,
}

const I: complex = complex{re: 0.0, im: 1.0};

fn sqr<T> (x: T) -> T where T: Mul<Output=T> + Copy { x * x }

impl Oscillator {
    fn value(&self, wavelength: f64) -> complex {
        let e = 1240.0 / wavelength;
        self.nosc / (sqr(self.en) - sqr(e) + I * self.eg * e)
    }
}

struct HODispersion {
    pub oscillators: Vec<Oscillator>
}

trait Dispersion {
    fn get_n(&self, wavelength: f64) -> complex;
}

impl Dispersion for HODispersion {
    fn get_n(&self, wavelength: f64) -> complex {
        let eps = self.oscillators.iter().fold(complex{re: 0.0, im: 0.0}, |eps, osc| eps + osc.value(wavelength));
        eps.sqrt()
    }
}

fn main() {
    let d = HODispersion{oscillators: vec![Oscillator{nosc: 15.2, en: 15.7, eg: 0.1, phi: 0.0}]};
    println!("Hello {}!", d.get_n(633.0));
}
