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
        assert!(float >= 0.0 && float < 1.0);
    }
}
