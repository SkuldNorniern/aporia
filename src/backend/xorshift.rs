//! XorShift random number generator implementation.
//!
//! XorShift generators are a class of pseudorandom number generators that use
//! XOR and shift operations to produce sequences of numbers with good statistical properties.
//!
//! # Characteristics
//!
//! - State size: 8 bytes
//! - Period: 2<sup>64</sup>−1
//! - Speed: Very Fast
//! - Quality: Good
//!
//! # Example
//!
//! ```rust
//! use aporia::{Rng, backend::XorShift};
//!
//! let backend = XorShift::new(987654321);
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [George Marsaglia (2003), "Xorshift RNGs"](https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf)
//! - [Wikipedia: Xorshift](https://en.wikipedia.org/wiki/Xorshift)

use super::RandomBackend;

/// XorShift random number generator struct.
#[derive(Clone, Debug)]
pub struct XorShift {
    state: u64,
}

impl XorShift {
    /// Creates a new `XorShift` instance with the given seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    ///
    /// Returns a new `XorShift` instance, or an error if the seed is invalid.
    pub fn try_new(seed: u64) -> core::result::Result<Self, crate::AporiaError> {
        if seed == 0 {
            return Err(crate::AporiaError::InvalidSeed("XorShift seed must be non-zero"));
        }
        Ok(Self { state: seed })
    }

    /// Creates a new `XorShift` and will panic on invalid seed.
    /// Only use when the seed is known-nonzero at call site.
    pub fn new(seed: u64) -> Self {
        // Safety: caller guarantees seed != 0 to avoid invalid state.
        if seed == 0 { panic!("invalid zero seed for XorShift"); }
        Self { state: seed }
    }
}

impl RandomBackend for XorShift {
    /// Generates the next random `u64` using the XorShift algorithm.
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xorshift_basic_sequence_nonzero() {
        let mut backend = XorShift::new(1);
        // XorShift with non-zero seed should not produce zero immediately
        let v1 = backend.next_u64();
        let v2 = backend.next_u64();
        assert_ne!(v1, 0);
        assert_ne!(v2, 0);
        assert_ne!(v1, v2);
    }

    #[test]
    fn xorshift_zero_seed_panics() {
        assert!(XorShift::try_new(0).is_err());
    }
}
