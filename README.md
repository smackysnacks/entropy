# entropy

[![Crates.io](https://img.shields.io/crates/v/entropy.svg)](https://crates.io/crates/entropy)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/entropy)](https://crates.io/crates/entropy)
[![Documentation](https://docs.rs/entropy/badge.svg)](https://docs.rs/entropy)
[![License](https://img.shields.io/crates/l/entropy.svg)](https://github.com/smackysnacks/entropy#license)

A Rust library for calculating Shannon entropy of byte sequences.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
entropy = "0.4"
```

## Usage

```rust
use entropy::shannon_entropy;

// Calculate Shannon entropy (in bits)
let h = shannon_entropy("hello, world");
assert_eq!(h, 3.0220551);

// Works with byte slices too
let h = shannon_entropy(b"\x00\x01\x02\x03");
```

## What is Shannon Entropy?

Shannon entropy measures the average amount of information contained in a message, expressed in bits. For byte data:

| Entropy Value | Meaning |
|---------------|---------|
| 0             | Completely uniform (e.g., "aaaa") |
| 8             | Maximum randomness (all 256 byte values equally distributed) |

### Common Use Cases

- Cryptography - Measuring randomness of keys or random number generators
- Data compression - Estimating how compressible data is
- Malware analysis - Detecting packed or encrypted executables
- Password strength - Estimating password complexity
