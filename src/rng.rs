//! The main RNG wrapper type that provides a consistent interface across backends.

use crate::backend::RandomBackend;
use std::marker::PhantomData;

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
    _phantom: PhantomData<B>,
}

impl<B: RandomBackend> Rng<B> {
    /// Creates a new RNG with the specified backend.
    ///
    /// # Arguments
    ///
    /// * `backend` - The RNG backend to use
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            _phantom: PhantomData,
        }
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
}
