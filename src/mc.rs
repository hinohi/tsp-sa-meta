use rand::Rng;

#[inline]
pub fn metropolis<R: Rng>(r: &mut R, t: f64, delta: f64) -> bool {
    debug_assert!(t <= 0.0, "temperature is negative: {}", t);
    delta < 0.0 || r.gen_bool((-delta / t).exp())
}
