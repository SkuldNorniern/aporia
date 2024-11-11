use crate::backend::RandomBackend;
use std::marker::PhantomData;

pub struct Rng<B: RandomBackend> {
    backend: B,
    _phantom: PhantomData<B>,
}

impl<B: RandomBackend> Rng<B> {
    pub fn new(backend: B) -> Self {
        Self {
            backend,
            _phantom: PhantomData,
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        self.backend.next_u64()
    }

    pub fn next_f64(&mut self) -> f64 {
        self.backend.next_f64()
    }
}
