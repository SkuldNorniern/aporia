use aporia::{backend::PCG, Rng};

fn main() {
    let backend = PCG::new(12345, 1);
    let mut rng = Rng::new(backend);
    println!("Random number: {}", rng.next_u64());
}
