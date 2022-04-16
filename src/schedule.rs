pub trait Schedule {
    fn set_max_iteration(&mut self, count: u64);
    fn set_iteration(&mut self, count: u64);
    fn get_temperature(&mut self) -> f64;
}

#[derive(Debug, Clone)]
pub struct PowSchedule {
    a: f64,
    b: f64,
    exponent: f64,
    max_iteration_count: f64,
    iteration_count: f64,
}

impl PowSchedule {
    pub fn new(temp_max: f64, temp_min: f64, exponent: f64) -> PowSchedule {
        PowSchedule {
            a: 2.0 * (temp_max - temp_min),
            b: 2.0 * temp_min - temp_max,
            exponent,
            max_iteration_count: 1.0,
            iteration_count: 0.0,
        }
    }
}

impl Schedule for PowSchedule {
    fn set_max_iteration(&mut self, count: u64) {
        self.max_iteration_count = count as f64 - 1.0;
    }

    fn set_iteration(&mut self, count: u64) {
        self.iteration_count = count as f64;
    }

    fn get_temperature(&mut self) -> f64 {
        self.iteration_count += 1.0;
        let x = (self.iteration_count - 1.0) / self.max_iteration_count;
        self.a / (1.0 + x.powf(self.exponent)) + self.b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pow_schedule() {
        let mut m = PowSchedule::new(10.0, 0.25, 2.0);
        m.set_max_iteration(100);
        assert_eq!(m.get_temperature(), 10.0);
        m.set_iteration(99);
        assert_eq!(m.get_temperature(), 0.25);
    }
}
