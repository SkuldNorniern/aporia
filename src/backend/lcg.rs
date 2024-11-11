use super::RandomBackend;

pub struct LCG {
    state: u64,
}

impl LCG {
    pub fn new(seed: u64) -> Self {
        Self { state: seed }
    }
}

impl RandomBackend for LCG {
    fn next_u64(&mut self) -> u64 {
        // Parameters from MMIX by Donald Knuth
        const MULTIPLIER: u64 = 6364136223846793005;
        const INCREMENT: u64 = 1442695040888963407;

        self.state = self.state.wrapping_mul(MULTIPLIER).wrapping_add(INCREMENT);
        self.state
    }
}
