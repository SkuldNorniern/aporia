# Aporia

Aporia is a small, dependency-free Rust RNG library with multiple backends and a consistent, ergonomic API. It favors clarity, correctness, and reproducibility. Not intended for cryptography.

> Aporia (ἀπορία): Greek for “difficulty,” “perplexity,” or “impasse.”

## Features

- Multiple RNG backends:
  - PCG (Permuted Congruential Generator)
  - XorShift
  - LCG (Linear Congruential Generator)
  - MT19937_64 (64-bit Mersenne Twister)
  - SplitMix64
  - Xoshiro256** (StarStar variant)
- Consistent `Rng` wrapper API across backends
- Unbiased integer ranges via zone rejection
- Iterators over `u64`/`f64` and a `fill_bytes` helper
- `no_std` support (with optional `std` feature)

## Installation

Default (with `std` feature enabled):

```toml
[dependencies]
aporia = "0.1.2"
```

no_std (disable default `std` feature):

```toml
[dependencies]
aporia = { version = "0.1.2", default-features = false }
```

## Quick start

```rust
use aporia::{Rng, backend::XorShift};

fn main() {
    let mut rng = Rng::new(XorShift::new(12345));
    let n = rng.next_u64();
    let x = rng.next_f64(); // in [0, 1)
    println!("n = {n}, x = {x}");
}
```

## Ranges without bias

The `Rng::gen_range(min, max)` method returns `Result<u64, AporiaError>`. It uses the unbiased “zone” rejection method to avoid modulo bias.

```rust
use aporia::{Rng, backend::XorShift};

fn sample_10_to_20() -> Result<u64, aporia::AporiaError> {
    let mut rng = Rng::new(XorShift::new(1));
    let v = rng.gen_range(10, 20)?; // returns Ok(10..20)
    Ok(v)
}

fn sample_floats() -> Result<f64, aporia::AporiaError> {
    let mut rng = Rng::new(XorShift::new(2));
    let v = rng.gen_range_f64(0.0, 1.0)?; // Ok([0.0, 1.0))
    Ok(v)
}
```

If bounds are guaranteed valid at the call site, using `expect` is acceptable:

```rust
use aporia::{Rng, backend::XorShift};

let mut rng = Rng::new(XorShift::new(7));
// Bounds are known-valid here; expecting success is safe.
let v = rng.gen_range(100, 200).expect("min < max holds by construction");
```

## Iterators and bytes

```rust
use aporia::{Rng, backend::SplitMix64};

let mut rng = Rng::new(SplitMix64::new(123));

// Take some u64s
let sum: u64 = rng.iter_u64().take(5).sum();

// Iterate f64s in [0,1)
let mut avg = 0.0;
for (i, x) in rng.iter_f64().take(10).enumerate() {
    avg = (avg * i as f64 + x) / (i as f64 + 1.0);
}

// Fill a byte buffer
let mut buf = [0u8; 32];
rng.fill_bytes(&mut buf);
```

## Backends at a glance

- XorShift: very fast, tiny state, simple
- PCG: fast, statistically strong for general use
- LCG: very simple, good for basic use
- MT19937_64: high quality, very long period, large state
- SplitMix64: very fast, good initializer for other RNGs
- Xoshiro256**: modern, high quality, very fast

## Error handling

The crate uses a small custom error enum:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum AporiaError {
    InvalidRangeU64 { min: u64, max: u64 },
    InvalidRangeF64 { min: f64, max: f64 },
    InvalidSeed(&'static str),
}
```

Examples of `Result`-based APIs:
- `Rng::gen_range(min, max) -> Result<u64, AporiaError>`
- `Rng::gen_range_f64(min, max) -> Result<f64, AporiaError>`
- `XorShift::try_new(seed) -> Result<XorShift, AporiaError>`

When `std` feature is enabled (default), `AporiaError` implements `std::error::Error`.

## no_std

- The crate supports `#![no_std]` when built with `default-features = false`.
- All core APIs are available; printing and OS entropy are not provided by this crate.

## Stability

For a given backend and seed, sequences are intended to remain stable across patch/minor versions.


## Contributing

Contributions are welcome. Please open an issue or PR for bugs or enhancements.