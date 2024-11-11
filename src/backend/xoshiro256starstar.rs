use super::RandomBackend;

pub struct Xoshiro256StarStar {
    s: [u64; 4],
}

impl Xoshiro256StarStar {
    pub fn new(seed: u64) -> Self {
        // Initialize the state using SplitMix64
        let mut state = seed;
        let mut s = [0u64; 4];
        
        for i in 0..4 {
            state = state.wrapping_add(0x9E3779B97F4A7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
            s[i] = z ^ (z >> 31);
        }
        
        Self { s }
    }
    
    fn rotl(x: u64, k: u32) -> u64 {
        (x << k) | (x >> (64 - k))
    }
}

impl RandomBackend for Xoshiro256StarStar {
    fn next_u64(&mut self) -> u64 {
        let result = Self::rotl(self.s[1].wrapping_mul(5), 7).wrapping_mul(9);
        let t = self.s[1] << 17;
        
        self.s[2] ^= self.s[0];
        self.s[3] ^= self.s[1];
        self.s[1] ^= self.s[2];
        self.s[0] ^= self.s[3];
        
        self.s[2] ^= t;
        self.s[3] = Self::rotl(self.s[3], 45);
        
        result
    }
} 