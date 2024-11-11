mod lcg;
mod pcg;
mod xorshift;
mod mt19937_64;
mod splitmix64;
mod xoshiro256starstar;

pub use lcg::LCG;
pub use pcg::PCG;
pub use xorshift::XorShift;
pub use mt19937_64::MT19937_64;
pub use splitmix64::SplitMix64;
pub use xoshiro256starstar::Xoshiro256StarStar;

pub trait RandomBackend {
    fn next_u64(&mut self) -> u64;

    fn next_f64(&mut self) -> f64 {
        let val = self.next_u64();
        (val as f64) / (u64::MAX as f64)
    }
}
