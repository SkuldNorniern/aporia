//! Alea: A flexible random number generation library
//! 
//! This crate provides a modular and extensible framework for random number generation
//! with multiple backend implementations. It allows users to choose different RNG
//! algorithms while maintaining a consistent interface.
//!
//! Design goals:
//! - Small, dependency-free core
//! - Clear separation between algorithm backends and the user-facing `Rng`
//! - Ergonomic helpers (`next_*`, `gen_range`, `fill_bytes`) and iterators
//! - Deterministic, reproducible sequences given the same seed
//!
//! # Features
//! 
//! - Multiple RNG backend implementations
//! - Consistent interface across all backends
//! - Easy to extend with new backends
//! - Support for both integer and floating-point random numbers
//!
//! # Examples
//!
//! ```rust
//! use aporia::{Rng, backend::XorShift};
//!
//! // Create a new RNG with the XorShift backend
//! let backend = XorShift::new(12345);
//! let mut rng = Rng::new(backend);
//!
//! // Generate random numbers
//! let random_int = rng.next_u64();
//! let random_float = rng.next_f64();
//! ```
//!
//! # Available Backends
//!
//! - `XorShift`: Fast and simple algorithm
//! - `PCG`: High-quality permuted congruential generator
//! - `LCG`: Linear congruential generator
//! - `MT19937_64`: 64-bit Mersenne Twister
//! - `SplitMix64`: Fast, simple generator good for initialization
//! - `Xoshiro256StarStar`: Modern, high-quality generator
//!
//! # Implementing Custom Backends
//!
//! To implement a custom backend, implement the `RandomBackend` trait:
//!
//! ```rust
//! use aporia::RandomBackend;
//!
//! struct MyBackend {
//!     state: u64,
//! }
//!
//! impl RandomBackend for MyBackend {
//!     fn next_u64(&mut self) -> u64 {
//!         // Your implementation here
//!         self.state
//!     }
//! }
//! ```

pub mod backend;
mod rng;

pub use backend::RandomBackend;
pub use rng::Rng;

#[cfg(test)]
mod tests {
    use super::*;
    use backend::XorShift;
    #[test]
    fn basic_random_generation() {
        let backend = XorShift::new(12345);
        let mut rng = Rng::new(backend);

        let val = rng.next_u64();
        assert!(val != 0);

        let float = rng.next_f64();
        assert!((0.0..1.0).contains(&float));
    }
}
