use aporia::{backend::LCG, Rng};

fn main() {
    let backend = LCG::new(12345);
    let mut rng = Rng::new(backend);
    println!("Random number: {}", rng.next_u64());
}
