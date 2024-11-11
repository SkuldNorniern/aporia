use super::RandomBackend;

pub struct MT19937_64 {
    mt: [u64; 312],
    index: usize,
}

impl MT19937_64 {
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