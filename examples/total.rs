use alea::{backend::XorShift, backend::LCG, backend::PCG, Rng};

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

    println!("PCG: {}", rng1.next_u64());
    println!("XorShift: {}", rng2.next_u64());
    println!("LCG: {}", rng3.next_u64());
}
