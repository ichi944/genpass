use std::fs::File;
use std::io::{self, Read};

/// A zero-dependency secure random number generator
///
/// Uses /dev/urandom on Unix-like systems for cryptographically secure random numbers
pub struct SecureRandom;

impl SecureRandom {
    /// Fill a buffer with cryptographically secure random bytes
    ///
    /// # Errors
    /// Returns an error if unable to read from the system's secure random source
    pub fn fill_bytes(buf: &mut [u8]) -> io::Result<()> {
        #[cfg(unix)]
        {
            let mut file = File::open("/dev/urandom")?;
            file.read_exact(buf)?;
            Ok(())
        }

        #[cfg(not(unix))]
        {
            // TODO: Implement Windows support using BCryptGenRandom
            compile_error!("Windows support not yet implemented. Please use Unix-like systems for now.");
        }
    }

    /// Generate a random number in the range [0, max)
    ///
    /// Uses rejection sampling to avoid modulo bias
    pub fn random_range(max: usize) -> io::Result<usize> {
        if max == 0 {
            return Ok(0);
        }

        // Use u32 for better performance and sufficient range
        let max = max as u32;

        // Calculate the largest multiple of max that fits in u32
        let range = u32::MAX - (u32::MAX % max);

        loop {
            let mut buf = [0u8; 4];
            Self::fill_bytes(&mut buf)?;
            let value = u32::from_le_bytes(buf);

            // Reject values outside the range to avoid modulo bias
            if value < range {
                return Ok((value % max) as usize);
            }
        }
    }

    /// Shuffle a slice in place using Fisher-Yates algorithm
    pub fn shuffle<T>(slice: &mut [T]) -> io::Result<()> {
        let len = slice.len();
        for i in (1..len).rev() {
            let j = Self::random_range(i + 1)?;
            slice.swap(i, j);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fill_bytes() {
        let mut buf = [0u8; 32];
        SecureRandom::fill_bytes(&mut buf).unwrap();

        // Check that not all bytes are zero (extremely unlikely with random data)
        assert!(buf.iter().any(|&b| b != 0));
    }

    #[test]
    fn test_random_range() {
        // Test basic functionality
        for _ in 0..100 {
            let value = SecureRandom::random_range(10).unwrap();
            assert!(value < 10);
        }
    }

    #[test]
    fn test_random_range_edge_cases() {
        // Test edge case: max = 0
        assert_eq!(SecureRandom::random_range(0).unwrap(), 0);

        // Test edge case: max = 1
        assert_eq!(SecureRandom::random_range(1).unwrap(), 0);
    }

    #[test]
    fn test_shuffle() {
        let mut data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let original = data.clone();

        SecureRandom::shuffle(&mut data).unwrap();

        // Check that all elements are still present
        let mut sorted_data = data.clone();
        sorted_data.sort();
        assert_eq!(sorted_data, original);
    }
}
