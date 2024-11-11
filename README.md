# Aporia

**Aporia** is a Rust library that provides implementations of various random number generators (RNGs). With **Aporia**, the die is cast—bringing randomness to your Rust projects!

> Aporia (ἀπορία): A Greek term meaning "difficulty," "perplexity," or "impasse," reflecting the unpredictable and puzzling nature of randomness.

## Features

- Multiple RNG backends:
  - PCG (Permuted Congruential Generator)
  - XorShift
  - LCG (Linear Congruential Generator)
  - MT19937_64 (64-bit Mersenne Twister)
  - SplitMix64
  - Xoshiro256\*\*\* (StarStar variant)
- Simple and consistent API
- Seedable generators for reproducible results

## Installation

Add Aporia to your `Cargo.toml`:

```toml
[dependencies]
aporia = "0.1.0"
```

Then run:

```shell
cargo build
```

## Usage

Here's how to use different RNG backends with Aporia:

```rust
use aporia::{
    backend::{LCG, MT19937_64, PCG, SplitMix64, XorShift, Xoshiro256StarStar},
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
```

## RNG Backends

### PCG (Permuted Congruential Generator)

A family of simple, fast, space-efficient, and statistically good algorithms for random number generation.

### XorShift

A class of pseudorandom number generators that use XOR and shift bitwise operations.

### LCG (Linear Congruential Generator)

One of the oldest and best-known pseudorandom number generator algorithms.

### MT19937_64 (64-bit Mersenne Twister)

A widely used pseudorandom number generator known for its high period and equidistribution properties.

### SplitMix64

A fast, non-cryptographic PRNG with a large period and good statistical properties.

### Xoshiro256\*\*\* (StarStar variant)

A state-of-the-art generator suitable for most applications except cryptography.

## Contributing

Contributions are welcome! If you have ideas for improvements or encounter any issues, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.

## Acknowledgements

- Inspired by various RNG implementations and algorithms.
- Thanks to the Rust community for their support.

## Contact

For any questions or suggestions, feel free to contact the project maintainer.

---

With **Aporia**, take a chance on randomness—you won't have to roll the dice on reliability!