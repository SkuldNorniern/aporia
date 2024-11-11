use aporia::{
    backend::{MT19937_64, PCG, XorShift, LCG, SplitMix64, Xoshiro256StarStar},
    Rng,
};

fn main() {
    // Using PCG
    let pcg = PCG::new(12345, 67890);
    let mut rng1 = Rng::new(pcg);

    // Using XorShift
    let xorshift = XorShift::new(12345);
    let mut rng2 = Rng::new(xorshift);

    // Using LCG
    let lcg = LCG::new(12345);
    let mut rng3 = Rng::new(lcg);

    // Using MT19937_64
    let mt19937_64 = MT19937_64::new(12345);
    let mut rng4 = Rng::new(mt19937_64);

    // Using SplitMix64
    let splitmix64 = SplitMix64::new(12345);
    let mut rng5 = Rng::new(splitmix64);

    // Using Xoshiro256StarStar
    let xoshiro256starstar = Xoshiro256StarStar::new(12345);
    let mut rng6 = Rng::new(xoshiro256starstar);

    println!("PCG: {}", rng1.next_u64());
    println!("XorShift: {}", rng2.next_u64());
    println!("LCG: {}", rng3.next_u64());
    println!("MT19937_64: {}", rng4.next_u64());
    println!("SplitMix64: {}", rng5.next_u64());
    println!("Xoshiro256StarStar: {}", rng6.next_u64());
}
