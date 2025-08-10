//! Permuted Congruential Generator (PCG) implementation.
//!
//! PCG is a family of simple algorithms for pseudorandom number generation that combine
//! a Linear Congruential Generator (LCG) with a permutation function to improve statistical properties.
//!
//! # Characteristics
//!
//! - State size: 16 bytes
//! - Period: 2<sup>64</sup>
//! - Speed: Fast
//! - Quality: High
//!
//! # Example
//!
//! ```rust
//! use aporia::{Rng, backend::PCG};
//!
//! let backend = PCG::new(42, 54); // Seed and sequence values
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [PCG: A Family of Better Random Number Generators](http://www.pcg-random.org)
//! - [Melissa E. O'Neill (2014), "PCG: A Family of Simple Fast Space-Efficient Statistically Good Algorithms for Random Number Generation"](https://www.cs.hmc.edu/tr/hmc-cs-2014-0905.pdf)

use super::RandomBackend;

/// Permuted Congruential Generator (PCG) struct.
#[derive(Clone, Debug)]
pub struct PCG {
    state: u64,
    increment: u64,
}

impl PCG {
    /// Creates a new `PCG` instance with the given seed and sequence.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    /// * `sequence` - The stream/sequence selector.
    pub fn new(seed: u64, sequence: u64) -> Self {
        let increment = (sequence << 1) | 1;
        let mut pcg = Self {
            state: 0,
            increment,
        };
        pcg.state = seed.wrapping_add(increment);
        let _ = pcg.next_u64(); // Advance to initial state
        pcg
    }
}

impl RandomBackend for PCG {
    /// Generates the next random `u64` using the PCG algorithm.
    fn next_u64(&mut self) -> u64 {
        const MULTIPLIER: u64 = 6364136223846793005;

        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);

        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;

        u64::from(xorshifted.rotate_right(rot))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pcg_basic_generation_changes_state() {
        let mut pcg = PCG::new(42, 54);
        let a = pcg.next_u64();
        let b = pcg.next_u64();
        assert_ne!(a, b);
    }
}
