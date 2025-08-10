//! Random number generator backend implementations.
//!
//! This module defines the core RNG backend trait [`RandomBackend`] and re-exports
//! a set of concrete algorithms. Each backend is small, dependency-free, and focuses
//! on clarity and correctness. These are not cryptographic RNGs.
//!
//! Highlights:
//! - Minimal trait surface with sensible defaults: `next_u64`, `next_u32`, `next_f64`, `fill_bytes`
//! - Multiple algorithms with different trade-offs (speed, state size, quality)
//! - Designed to be used through the high-level [`crate::Rng`] wrapper
//!
//! This module provides various random number generator (RNG) implementations with different
//! characteristics and trade-offs. Each backend implements the [`RandomBackend`] trait,
//! allowing them to be used interchangeably.
//!
//! # Available Backends
//!
//! - [`LCG`]: Linear Congruential Generator - Simple and fast, but with known limitations
//! - [`PCG`]: Permuted Congruential Generator - High-quality output with good statistical properties
//! - [`XorShift`]: Simple and fast algorithm with reasonable quality
//! - [`MT19937_64`]: 64-bit Mersenne Twister - Large state, very long period (2^19937-1)
//! - [`SplitMix64`]: Fast, simple generator suitable for initialization
//! - [`Xoshiro256StarStar`]: Modern, high-quality generator with excellent statistical properties
//!
//! # Choosing a Backend
//!
//! Each backend has different characteristics:
//!
//! | Backend | State Size | Speed | Quality | Period |
//! |---------|------------|-------|---------|---------|
//! | LCG | 8 bytes | Very Fast | Basic | 2^64 |
//! | PCG | 16 bytes | Fast | High | 2^64 |
//! | XorShift | 8 bytes | Very Fast | Good | 2^64 - 1 |
//! | MT19937_64 | 2.5KB | Moderate | High | 2^19937 - 1 |
//! | SplitMix64 | 8 bytes | Very Fast | Good | 2^64 |
//! | Xoshiro256** | 32 bytes | Very Fast | Excellent | 2^256 - 1 |
//!
//! # Examples
//!
//! ```rust
//! use aporia::{Rng, backend::XorShift};
//!
//! // Create a new RNG with XorShift backend
//! let backend = XorShift::new(12345);
//! let mut rng = Rng::new(backend);
//!
//! // Generate random numbers
//! let random_int = rng.next_u64();
//! let random_float = rng.next_f64();
//! ```

// Re-export all backends
pub use self::lcg::LCG;
pub use self::pcg::PCG;
pub use self::xorshift::XorShift;
pub use self::mt19937_64::MT19937_64;
pub use self::splitmix64::SplitMix64;
pub use self::xoshiro256starstar::Xoshiro256StarStar;

mod lcg;
mod pcg;
mod xorshift;
mod mt19937_64;
mod splitmix64;
mod xoshiro256starstar;

/// Trait that defines the interface for random number generator backends.
///
/// This trait must be implemented by all RNG backends. It provides a minimal
/// interface for generating random numbers, with a default implementation
/// for floating-point numbers.
///
/// # Required Methods
///
/// - `next_u64`: Generate the next 64-bit random integer
///
/// # Provided Methods
///
/// - `next_f64`: Generate a random float in [0, 1) using `next_u64`
///
/// # Examples
///
/// ```rust
/// use aporia::RandomBackend;
///
/// struct MyBackend {
///     state: u64
/// }
///
/// impl RandomBackend for MyBackend {
///     fn next_u64(&mut self) -> u64 {
///         // Your implementation here
///         self.state
///     }
/// }
/// ```
pub trait RandomBackend {
    /// Generates the next 64-bit unsigned integer.
    ///
    /// This is the core method that must be implemented by all backends.
    fn next_u64(&mut self) -> u64;

    /// Generates a random floating-point number in the range [0, 1).
    ///
    /// This method has a default implementation that converts the result
    /// of `next_u64()` to a floating-point number using the upper 53 bits.
    ///
    /// Using the upper 53 bits ensures the value is in [0, 1) and matches the
    /// precision of `f64` mantissa, avoiding the possibility of returning 1.0.
    fn next_f64(&mut self) -> f64 {
        // Take the top 53 bits to construct an f64 in [0, 1).
        // 53 is the number of mantissa bits in an f64.
        let val = self.next_u64() >> 11; // 64 - 53
        (val as f64) * (1.0 / ((1u64 << 53) as f64))
    }

    /// Generates the next 32-bit unsigned integer.
    ///
    /// Default implementation returns the upper 32 bits of `next_u64()`.
    fn next_u32(&mut self) -> u32 {
        (self.next_u64() >> 32) as u32
    }

    /// Fills `buf` with random bytes using repeated `next_u64()` calls.
    /// The tail shorter than 8 bytes is handled with a final partial copy.
    fn fill_bytes(&mut self, buf: &mut [u8]) {
        let mut i = 0;
        let len = buf.len();
        while i + 8 <= len {
            let v = self.next_u64().to_ne_bytes();
            buf[i..i + 8].copy_from_slice(&v);
            i += 8;
        }
        if i < len {
            let v = self.next_u64().to_ne_bytes();
            let rem = len - i;
            buf[i..].copy_from_slice(&v[..rem]);
        }
    }
}
