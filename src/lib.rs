/// Calculates the Shannon entropy of a byte string.
///
/// # Examples
///
/// ```
/// use entropy::shannon_entropy;
///
/// let h = shannon_entropy(b"hello, world");
/// assert_eq!(h, 3.0220551);
/// ```
pub fn shannon_entropy(bytes: &[u8]) -> f32 {
    let mut entropy = 0.0;
    let mut counts = [0; 256];

    for &b in bytes {
        counts[b as usize] += 1;
    }

    for &count in counts.iter() {
        if count == 0 { continue }

        let p: f32 = (count as f32) / (bytes.len() as f32);
        entropy -= p * p.log(2.0);
    }

    entropy
}

pub fn metric_entropy(bytes: &[u8]) -> f32 {
    let h = shannon_entropy(bytes);

    h / (bytes.len() as f32)
}

#[cfg(test)]
mod tests {
    use super::shannon_entropy;

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
        for i in 0..256 {
            bytes[i] = i as u8;
        }

        let h = shannon_entropy(&bytes);
        assert_eq!(h, 8.0);
    }

    #[test]
    fn test_entropy_equal_distribution2() {
        let mut bytes = [0u8; 256*2];
        for i in 0..256*2 {
            bytes[i] = (i % 256) as u8;
        }

        let h = shannon_entropy(&bytes);
        assert_eq!(h, 8.0);
    }

    #[test]
    fn test_entropy_helloworld() {
        let h = shannon_entropy(b"hello, world");
        assert_eq!(h, 3.0220551);
    }
}
