use super::RandomBackend;

pub struct XorShift {
    state: u64,
}

impl XorShift {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl RandomBackend for XorShift {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}
