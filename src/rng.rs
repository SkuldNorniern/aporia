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
    #[inline]
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Generates the next 64-bit unsigned integer.
    ///
    /// # Returns
    ///
    /// A randomly generated `u64` value
    #[inline]
    #[must_use]
    pub fn next_u64(&mut self) -> u64 {
        self.backend.next_u64()
    }

    /// Generates the next floating-point number in the range [0, 1).
    ///
    /// # Returns
    ///
    /// A randomly generated `f64` value between 0 (inclusive) and 1 (exclusive)
    #[inline]
    #[must_use]
    pub fn next_f64(&mut self) -> f64 {
        self.backend.next_f64()
    }

    /// Generates the next 32-bit unsigned integer.
    #[inline]
    #[must_use]
    pub fn next_u32(&mut self) -> u32 {
        self.backend.next_u32()
    }

    /// Generates the next 32-bit floating point number in [0, 1).
    #[inline]
    #[must_use]
    pub fn next_f32(&mut self) -> f32 {
        let val = (self.backend.next_u64() >> 40) as u32; // top 24 bits
        (val as f32) * (1.0 / ((1u32 << 24) as f32))
    }

    /// Generates a random boolean with p=0.5.
    #[inline]
    #[must_use]
    pub fn next_bool(&mut self) -> bool {
        (self.backend.next_u64() & 1) != 0
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
    #[inline]
    #[must_use]
    pub fn gen_range(&mut self, min: u64, max: u64) -> u64 {
        if min >= max {
            panic!("min must be less than max");
        }
        let range = max - min;
        let zone = u64::MAX - (u64::MAX % range);
        loop {
            let v = self.next_u64();
            if v < zone {
                return min + (v % range);
            }
        }
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
    #[inline]
    #[must_use]
    pub fn gen_range_f64(&mut self, min: f64, max: f64) -> f64 {
        if min >= max {
            panic!("min must be less than max");
        }
        
        let rand = self.next_f64();
        min + (rand * (max - min))
    }

    /// Fills `buf` with random bytes from the backend.
    #[inline]
    pub fn fill_bytes(&mut self, buf: &mut [u8]) {
        self.backend.fill_bytes(buf)
    }
}

impl<B> Clone for Rng<B>
where
    B: RandomBackend + Clone,
{
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
        }
    }
}

impl<B> core::fmt::Debug for Rng<B>
where
    B: RandomBackend + core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rng").field("backend", &self.backend).finish()
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

    #[test]
    fn next_u32_and_f32_and_bool_work() {
        let backend = XorShift::new(7);
        let mut rng = Rng::new(backend);
        let _u32v = rng.next_u32();
        let f = rng.next_f32();
        assert!(f >= 0.0 && f < 1.0);
        let _b = rng.next_bool();
    }
}
