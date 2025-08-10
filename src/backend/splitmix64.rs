//! SplitMix64 pseudorandom number generator implementation.
//!
//! SplitMix64 is a fast generator with a reasonable statistical quality, often used to
//! initialize other generators due to its speed and simplicity.
//!
//! # Characteristics
//!
//! - State size: 8 bytes
//! - Period: 2<sup>64</sup>
//! - Speed: Very Fast
//! - Quality: Good
//!
//! # Example
//!
//! ```rust
//! use aporia::{Rng, backend::SplitMix64};
//!
//! let backend = SplitMix64::new(123456789);
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [Wikipedia: SplitMix64](https://en.wikipedia.org/wiki/SplitMix64)
//! - [Steele, G.L., Vigna, S. (2019) "Computationally easy, spectrally pure pseudorandom number generators"](https://vigna.di.unimi.it/ftp/papers/SplitMix.pdf)

use super::RandomBackend;

/// SplitMix64 random number generator struct.
pub struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    /// Creates a new `SplitMix64` instance with the given seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl RandomBackend for SplitMix64 {
    /// Generates the next random `u64` using the SplitMix64 algorithm.
    fn next_u64(&mut self) -> u64 {
        self.state = self.state.wrapping_add(0x9E3779B97F4A7C15);
        let mut z = self.state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitmix64_basic_changes() {
        let mut sm = SplitMix64::new(0);
        let a = sm.next_u64();
        let b = sm.next_u64();
        assert_ne!(a, b);
    }
}