use aporia::{backend::XorShift, Rng};

fn main() {
    let backend = XorShift::new(12345);
    let mut rng = Rng::new(backend);
    println!("Random number: {}", rng.next_u64());
}
