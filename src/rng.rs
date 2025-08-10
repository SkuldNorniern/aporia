//! The main RNG wrapper type that provides a consistent interface across backends.

use crate::backend::RandomBackend;

/// A random number generator that works with any backend implementing [`RandomBackend`].
///
/// This struct provides a consistent interface for random number generation,
/// regardless of the underlying algorithm used.
///
/// # Type Parameters
///
/// * `B` - The backend type that implements [`RandomBackend`]
///
/// # Examples
///
/// ```rust
/// use aporia::{Rng, backend::XorShift};
///
/// let backend = XorShift::new(12345);
/// let mut rng = Rng::new(backend);
///
/// let random_number = rng.next_u64();
/// let random_float = rng.next_f64();
/// ```
pub struct Rng<B: RandomBackend> {
    backend: B,
}

impl<B: RandomBackend> Rng<B> {
    /// Creates a new RNG with the specified backend.
    ///
    /// # Arguments
    ///
    /// * `backend` - The RNG backend to use
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Generates the next 64-bit unsigned integer.
    ///
    /// # Returns
    ///
    /// A randomly generated `u64` value
    pub fn next_u64(&mut self) -> u64 {
        self.backend.next_u64()
    }

    /// Generates the next floating-point number in the range [0, 1).
    ///
    /// # Returns
    ///
    /// A randomly generated `f64` value between 0 (inclusive) and 1 (exclusive)
    pub fn next_f64(&mut self) -> f64 {
        self.backend.next_f64()
    }

    /// Generates a random number within the given range.
    ///
    /// # Arguments
    ///
    /// * `min` - The inclusive lower bound
    /// * `max` - The exclusive upper bound
    ///
    /// # Returns
    ///
    /// A randomly generated number within the range [min, max)
    ///
    /// # Panics
    ///
    /// Panics if `min >= max`
    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            panic!("min must be less than max");
        }
        
        let range = max - min;
        let mut rand = self.next_u64();
        
        // Modulo bias correction
        let threshold = (u64::MAX - range + 1) % range;
        while rand < threshold {
            rand = self.next_u64();
        }
        
        min + (rand % range)
    }

    /// Generates a random floating-point number within the given range.
    ///
    /// # Arguments
    ///
    /// * `min` - The inclusive lower bound
    /// * `max` - The exclusive upper bound
    ///
    /// # Returns
    ///
    /// A randomly generated f64 value within the range [min, max)
    ///
    /// # Panics
    ///
    /// Panics if `min >= max`
    pub fn gen_range_f64(&mut self, min: f64, max: f64) -> f64 {
        if min >= max {
            panic!("min must be less than max");
        }
        
        let rand = self.next_f64();
        min + (rand * (max - min))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::{XorShift, SplitMix64};

    #[test]
    fn next_f64_in_unit_interval() {
        let backend = XorShift::new(1);
        let mut rng = Rng::new(backend);
        for _ in 0..1000 {
            let x = rng.next_f64();
            assert!(x >= 0.0 && x < 1.0);
        }
    }

    #[test]
    fn gen_range_integral_bounds() {
        let backend = SplitMix64::new(123);
        let mut rng = Rng::new(backend);
        for _ in 0..1000 {
            let x = rng.gen_range(10, 20);
            assert!((10..20).contains(&x));
        }
    }

    #[test]
    fn gen_range_f64_bounds() {
        let backend = SplitMix64::new(123);
        let mut rng = Rng::new(backend);
        for _ in 0..1000 {
            let x = rng.gen_range_f64(0.5, 1.5);
            assert!(x >= 0.5 && x < 1.5);
        }
    }
}
