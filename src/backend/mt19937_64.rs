//! 64-bit Mersenne Twister (MT19937-64) implementation.
//!
//! The Mersenne Twister is a pseudorandom number generator with an extremely long period
//! of 2<sup>19937</sup>−1. This 64-bit variant is known for its high quality and speed
//! in generating random numbers.
//!
//! # Characteristics
//!
//! - State size: 2.5 KB (312 * 8 bytes)
//! - Period: 2<sup>19937</sup>−1
//! - Speed: Moderate
//! - Quality: High
//!
//! # Example
//!
//! ```rust
//! use aporia::{Rng, backend::MT19937_64};
//!
//! let backend = MT19937_64::new(5489); // Default seed from reference implementation
//! let mut rng = Rng::new(backend);
//! let random_number = rng.next_u64();
//! ```
//!
//! # References
//!
//! - [Mutsuo Saito and Makoto Matsumoto - MT19937-64 C code](https://www.math.sci.hiroshima-u.ac.jp/~m-mat/MT/emt64.html)
//! - [Wikipedia: Mersenne Twister](https://en.wikipedia.org/wiki/Mersenne_Twister)
use super::RandomBackend;

/// 64-bit Mersenne Twister (MT19937-64) struct.
pub struct MT19937_64 {
    mt: [u64; 312],
    index: usize,
}

impl MT19937_64 {
    /// Creates a new `MT19937_64` instance with the given seed.
    ///
    /// # Arguments
    ///
    /// * `seed` - The initial seed value.
    pub fn new(seed: u64) -> Self {
        let mut mt = [0u64; 312];
        mt[0] = seed;
        
        for i in 1..312 {
            mt[i] = 6364136223846793005u64
                .wrapping_mul(mt[i-1] ^ (mt[i-1] >> 62))
                .wrapping_add(i as u64);
        }
        
        Self { mt, index: 312 }
    }
    
    /// Performs the twist operation to update the state of the MT19937-64 generator.
    fn twist(&mut self) {
        const LOWER_MASK: u64 = (1u64 << 31) - 1;
        const UPPER_MASK: u64 = !LOWER_MASK;
        const MATRIX_A: u64 = 0xB5026F5AA96619E9;
        
        for i in 0..312 {
            let x = (self.mt[i] & UPPER_MASK) | (self.mt[(i + 1) % 312] & LOWER_MASK);
            let mut x_a = x >> 1;
            
            if x & 1 != 0 {
                x_a ^= MATRIX_A;
            }
            
            self.mt[i] = self.mt[(i + 156) % 312] ^ x_a;
        }
        self.index = 0;
    }
}

impl RandomBackend for MT19937_64 {
    /// Generates the next 64-bit unsigned integer.
    fn next_u64(&mut self) -> u64 {
        if self.index >= 312 {
            self.twist();
        }
        
        let mut y = self.mt[self.index];
        y ^= (y >> 29) & 0x5555555555555555;
        y ^= (y << 17) & 0x71D67FFFEDA60000;
        y ^= (y << 37) & 0xFFF7EEE000000000;
        y ^= y >> 43;
        
        self.index += 1;
        y
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mt19937_64_generates_values() {
        let mut mt = MT19937_64::new(5489);
        let a = mt.next_u64();
        let b = mt.next_u64();
        assert_ne!(a, b);
    }
}