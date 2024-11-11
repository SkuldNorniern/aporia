use super::RandomBackend;

pub struct PCG {
    state: u64,
    increment: u64,
}

impl PCG {
    pub fn new(seed: u64, sequence: u64) -> Self {
        let increment = (sequence << 1) | 1;
        let mut pcg = Self {
            state: 0,
            increment,
        };
        pcg.state = seed.wrapping_add(increment);
        pcg.next_u64(); // Advance to initial state
        pcg
    }
}

impl RandomBackend for PCG {
    fn next_u64(&mut self) -> u64 {
        const MULTIPLIER: u64 = 6364136223846793005;

        let old_state = self.state;
        self.state = old_state
            .wrapping_mul(MULTIPLIER)
            .wrapping_add(self.increment);

        let xorshifted = (((old_state >> 18) ^ old_state) >> 27) as u32;
        let rot = (old_state >> 59) as u32;

        ((xorshifted >> rot) | (xorshifted << ((-(rot as i32)) & 31))) as u64
    }
}
