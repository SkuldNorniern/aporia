//! Linear Congruential Generator (LCG) implementation.
//!
//! The LCG is one of the oldest and simplest pseudorandom number generator algorithms.
//! It uses a linear equation to generate a sequence of pseudorandom numbers.
//!
//! # Characteristics
//!
//! - State size: 8 bytes
//! - Period: 2<sup>64</sup>
//! - Speed: Very Fast
//! - Quality: Basic (not suitable for cryptographic purposes)
//!
//! # Example
//!
//! ```rust
//! use aporia::{Rng, backend::LCG};
//!
//! let backend = LCG::new(12345);
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [Wikipedia: Linear congruential generator](https://en.wikipedia.org/wiki/Linear_congruential_generator)
//! - Donald Knuth, *The Art of Computer Programming*, Vol. 2

use super::RandomBackend;

/// Linear Congruential Generator (LCG) struct.
pub struct LCG {
    state: u64,
}

impl LCG {
    /// Creates a new `LCG` instance with the given seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl RandomBackend for LCG {
    /// Generates the next random `u64` using the LCG algorithm.
    fn next_u64(&mut self) -> u64 {
        // Parameters from MMIX by Donald Knuth
        const MULTIPLIER: u64 = 6364136223846793005;
        const INCREMENT: u64 = 1442695040888963407;

        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        self.state
    }
}
