//! Xoshiro256\*\* random number generator implementation.
//!
//! Xoshiro256\*\* is a fast all-purpose pseudorandom number generator with excellent statistical properties.
//! It is suitable for simulation and games, but not for cryptographic purposes.
//!
//! # Characteristics
//!
//! - State size: 32 bytes (4 * 8 bytes)
//! - Period: 2<sup>256</sup>−1
//! - Speed: Very Fast
//! - Quality: Excellent
//!
//! # Example
//!
//! ```rust
//! use aporia::{Rng, backend::Xoshiro256StarStar};
//!
//! let backend = Xoshiro256StarStar::new(13579);
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [Sebastiano Vigna (2018), "Xoshiro/Xoroshiro generators and the PRNG shootout"](http://xoshiro.di.unimi.it/)
//! - [Wikipedia: xorshift](https://en.wikipedia.org/wiki/Xorshift#xorshift*)

use super::RandomBackend;
use crate::backend::SplitMix64;

/// Xoshiro256\*\* random number generator struct.
#[derive(Clone, Debug)]
pub struct Xoshiro256StarStar {
    s: [u64; 4],
}

impl Xoshiro256StarStar {
    /// Creates a new `Xoshiro256StarStar` instance with the given seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    ///
    /// The seed is expanded using `SplitMix64` to fill the state array.
    pub fn new(seed: u64) -> Self {
        // Initialize the state using SplitMix64
        let mut state = SplitMix64::new(seed);
        let mut s = [0u64; 4];

        for x in s.iter_mut().take(4) {
            *x = state.next_u64();
        }

        Self { s }
    }

    /// Rotates the bits of `x` left by `k` positions.
    ///
    /// # Arguments
    ///
    /// * `x` - The value to rotate.
    /// * `k` - The number of bits to rotate left.
    fn rotl(x: u64, k: u32) -> u64 {
        (x << k) | (x >> (64 - k))
    }
}

impl RandomBackend for Xoshiro256StarStar {
    /// Generates the next random `u64` using the Xoshiro256\*\* algorithm.
    fn next_u64(&mut self) -> u64 {
        let result = Self::rotl(self.s[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.s[1] << 17;

        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];

        self.s[2] ^= t;
        self.s[3] = Self::rotl(self.s[3], 45);

        result
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xoshiro256starstar_progresses() {
        let mut xo = Xoshiro256StarStar::new(13579);
        let a = xo.next_u64();
        let b = xo.next_u64();
        assert_ne!(a, b);
    }
}
