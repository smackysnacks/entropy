//! # Entropy
//!
//! A Rust library for calculating Shannon entropy of byte sequences.
//!
//! ## Overview
//!
//! [Shannon entropy](https://en.wikipedia.org/wiki/Entropy_(information_theory)) measures the
//! average amount of information (in bits) contained in a message. It's commonly used in:
//!
//! - Cryptography (measuring randomness)
//! - Data compression (estimating compressibility)
//! - Malware analysis (detecting packed/encrypted content)
//! - Password strength estimation
//!
//! ## Quick Start
//!
//! ```
//! use entropy::shannon_entropy;
//!
//! // Calculate Shannon entropy (bits)
//! let h = shannon_entropy("hello, world");
//! assert_eq!(h, 3.0220551);
//! ```
//!
//! ## Interpreting Results
//!
//! - **Shannon entropy** ranges from 0 to 8 bits for byte data
//!   - 0 = completely uniform (e.g., "aaaa")
//!   - 8 = maximum entropy (all 256 byte values equally distributed)

#![warn(rust_2018_idioms)]

/// Calculates the Shannon entropy of a byte sequence.
///
/// Shannon entropy measures the average information content per symbol,
/// expressed in bits. For byte data, the result ranges from 0.0 (completely
/// uniform) to 8.0 (all 256 byte values equally represented).
///
/// # Arguments
///
/// * `data` - Any type that can be referenced as a byte slice (`&[u8]`),
///   including `&str`, `String`, `&[u8]`, `Vec<u8>`, etc.
///
/// # Returns
///
/// The Shannon entropy in bits. Returns `0.0` for empty input.
///
/// # Examples
///
/// ```
/// use entropy::shannon_entropy;
///
/// // Works with string slices
/// let h1 = shannon_entropy("hello, world");
/// assert_eq!(h1, 3.0220551);
///
/// // Works with byte slices
/// let h2 = shannon_entropy(b"hello, world");
/// assert_eq!(h2, 3.0220551);
///
/// // Uniform data has zero entropy
/// assert_eq!(shannon_entropy("aaaa"), 0.0);
///
/// // Two equally distributed symbols = 1 bit
/// assert_eq!(shannon_entropy("ab"), 1.0);
///
/// // Maximum entropy for bytes
/// let all_bytes: Vec<u8> = (0..=255).collect();
/// assert_eq!(shannon_entropy(all_bytes), 8.0);
/// ```
pub fn shannon_entropy<T: AsRef<[u8]>>(data: T) -> f32 {
    let bytes = data.as_ref();
    if bytes.is_empty() {
        return 0.0;
    }

    let mut entropy = 0.0;
    let mut counts = [0usize; 256];

    for &b in bytes {
        counts[b as usize] += 1;
    }

    for &count in &counts {
        if count == 0 {
            continue;
        }

        let p: f32 = (count as f32) / (bytes.len() as f32);
        entropy -= p * p.log(2.0);
    }

    entropy
}

pub fn metric_entropy<T: AsRef<[u8]>>(data: T) -> f32 {
    let bytes = data.as_ref();
    if bytes.is_empty() {
        return 0.0;
    }

    let h = shannon_entropy(bytes);
    h / (bytes.len() as f32)
}

#[cfg(test)]
mod tests {
    use super::{metric_entropy, shannon_entropy};

    #[test]
    fn test_entropy_empty() {
        let h = shannon_entropy(b"");
        assert_eq!(h, 0.0);
    }

    #[test]
    fn test_entropy_a() {
        let h = shannon_entropy(b"a");
        assert_eq!(h, 0.0);
    }

    #[test]
    fn test_entropy_aaaaa() {
        let h = shannon_entropy(b"aaaaa");
        assert_eq!(h, 0.0);
    }

    #[test]
    fn test_entropy_ab() {
        let h = shannon_entropy(b"ab");
        assert_eq!(h, 1.0);
    }

    #[test]
    fn test_entropy_aab() {
        let h = shannon_entropy(b"aab");
        assert_eq!(h, 0.9182958);
    }

    #[test]
    fn test_entropy_equal_distribution1() {
        let mut bytes = [0u8; 256];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = i as u8;
        }

        let h = shannon_entropy(bytes);
        assert_eq!(h, 8.0);
    }

    #[test]
    fn test_entropy_equal_distribution2() {
        let mut bytes = [0u8; 256 * 2];
        for (i, b) in bytes.iter_mut().enumerate() {
            *b = (i % 256) as u8;
        }

        let h = shannon_entropy(bytes);
        assert_eq!(h, 8.0);
    }

    #[test]
    fn test_entropy_helloworld() {
        let h = shannon_entropy(b"hello, world");
        assert_eq!(h, 3.0220551);
    }

    #[test]
    fn metric_entropy_empty_input() {
        assert_eq!(metric_entropy(b""), 0.0)
    }
}
