use rand::Rng;

pub trait Transition {
    fn set_max_iteration(&mut self, count: u64);
    fn set_iteration(&mut self, count: u64);
    fn trans<R: Rng>(&mut self, r: &mut R, delta: f64) -> bool;
    fn get_temperature(&self) -> f64;
}

#[inline]
fn metropolis<R: Rng>(r: &mut R, t: f64, delta: f64) -> bool {
    delta < 0.0 || r.gen_bool((-delta / t).exp())
}

#[derive(Debug, Clone)]
pub struct MetropolisPow {
    a: f64,
    b: f64,
    exponent: f64,
    max_iteration_count: f64,
    iteration_count: f64,
}

impl MetropolisPow {
    pub fn new(temp_max: f64, temp_min: f64, exponent: f64) -> MetropolisPow {
        MetropolisPow {
            a: 2.0 * (temp_max - temp_min),
            b: 2.0 * temp_min - temp_max,
            exponent,
            max_iteration_count: 1.0,
            iteration_count: 0.0,
        }
    }
}

impl Transition for MetropolisPow {
    fn set_max_iteration(&mut self, count: u64) {
        self.max_iteration_count = count as f64 - 1.0;
    }
    fn set_iteration(&mut self, count: u64) {
        self.iteration_count = count as f64;
    }
    fn trans<R: Rng>(&mut self, r: &mut R, delta: f64) -> bool {
        self.iteration_count += 1.0;
        metropolis(r, self.get_temperature(), delta)
    }
    fn get_temperature(&self) -> f64 {
        let x = (self.iteration_count - 1.0) / self.max_iteration_count;
        self.a / (1.0 + x.powf(self.exponent)) + self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metropolis_pow() {
        let mut m = MetropolisPow::new(10.0, 0.25, 2.0);
        m.set_max_iteration(100);
        m.set_iteration(1);
        assert_eq!(m.get_temperature(), 10.0);
        m.set_iteration(100);
        assert_eq!(m.get_temperature(), 0.25);
    }
}
