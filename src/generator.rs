use crate::random::SecureRandom;
use std::io;

/// Character sets for password generation
const LOWERCASE: &str = "abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMERIC: &str = "0123456789";

/// Visually ambiguous characters to exclude
const AMBIGUOUS_CHARS: &[char] = &['0', 'O', '1', 'l', 'I', '|'];

/// Constraints for password generation
#[derive(Debug, Clone)]
pub struct PasswordConstraints {
    pub min_numeric: Option<usize>,
    pub max_numeric: Option<usize>,
    pub min_lower: Option<usize>,
    pub max_lower: Option<usize>,
    pub min_upper: Option<usize>,
    pub max_upper: Option<usize>,
    pub min_symbol: Option<usize>,
    pub max_symbol: Option<usize>,
    pub min_length: usize,
    pub max_length: usize,
    pub symbols: String,
    pub exclude_ambiguous: bool,
}

impl PasswordConstraints {
    /// Validate that constraints are internally consistent
    pub fn validate(&self) -> Result<(), String> {
        // Check min <= max for each character type
        if let (Some(min), Some(max)) = (self.min_numeric, self.max_numeric) {
            if min > max {
                return Err("min_numeric cannot be greater than max_numeric".to_string());
            }
        }
        if let (Some(min), Some(max)) = (self.min_lower, self.max_lower) {
            if min > max {
                return Err("min_lower cannot be greater than max_lower".to_string());
            }
        }
        if let (Some(min), Some(max)) = (self.min_upper, self.max_upper) {
            if min > max {
                return Err("min_upper cannot be greater than max_upper".to_string());
            }
        }
        if let (Some(min), Some(max)) = (self.min_symbol, self.max_symbol) {
            if min > max {
                return Err("min_symbol cannot be greater than max_symbol".to_string());
            }
        }

        // Check length constraints
        if self.min_length > self.max_length {
            return Err("min_length cannot be greater than max_length".to_string());
        }

        // Check that minimum requirements can be satisfied
        let total_min = self.min_numeric.unwrap_or(0)
            + self.min_lower.unwrap_or(0)
            + self.min_upper.unwrap_or(0)
            + self.min_symbol.unwrap_or(0);

        if total_min > self.max_length {
            return Err(format!(
                "Sum of minimum character requirements ({}) exceeds maximum length ({})",
                total_min, self.max_length
            ));
        }

        if total_min > self.min_length {
            return Err(format!(
                "Sum of minimum character requirements ({}) exceeds minimum length ({})",
                total_min, self.min_length
            ));
        }

        Ok(())
    }
}

/// Password generator
pub struct PasswordGenerator {
    constraints: PasswordConstraints,
    lowercase_chars: Vec<char>,
    uppercase_chars: Vec<char>,
    numeric_chars: Vec<char>,
    symbol_chars: Vec<char>,
}

impl PasswordGenerator {
    /// Create a new password generator with the given constraints
    pub fn new(constraints: PasswordConstraints) -> Result<Self, String> {
        constraints.validate()?;

        let filter_ambiguous = |s: &str| -> Vec<char> {
            if constraints.exclude_ambiguous {
                s.chars()
                    .filter(|c| !AMBIGUOUS_CHARS.contains(c))
                    .collect()
            } else {
                s.chars().collect()
            }
        };

        let lowercase_chars = filter_ambiguous(LOWERCASE);
        let uppercase_chars = filter_ambiguous(UPPERCASE);
        let numeric_chars = filter_ambiguous(NUMERIC);
        let symbol_chars = filter_ambiguous(&constraints.symbols);

        Ok(Self {
            constraints,
            lowercase_chars,
            uppercase_chars,
            numeric_chars,
            symbol_chars,
        })
    }

    /// Generate a password satisfying the constraints
    pub fn generate(&self) -> io::Result<String> {
        // Determine actual password length
        let length = if self.constraints.min_length == self.constraints.max_length {
            self.constraints.min_length
        } else {
            let range = self.constraints.max_length - self.constraints.min_length + 1;
            self.constraints.min_length + SecureRandom::random_range(range)?
        };

        // Start with minimum counts for each type
        let mut numeric_count = self.constraints.min_numeric.unwrap_or(0);
        let mut lower_count = self.constraints.min_lower.unwrap_or(0);
        let mut upper_count = self.constraints.min_upper.unwrap_or(0);
        let mut symbol_count = self.constraints.min_symbol.unwrap_or(0);

        // Distribute remaining slots
        let total = numeric_count + lower_count + upper_count + symbol_count;
        let mut remaining = length - total;

        while remaining > 0 {
            // Build a list of character types that can still accept more characters
            let mut available_types = Vec::new();

            if self.constraints.max_numeric.map_or(true, |max| numeric_count < max) {
                available_types.push(0);
            }
            if self.constraints.max_lower.map_or(true, |max| lower_count < max) {
                available_types.push(1);
            }
            if self.constraints.max_upper.map_or(true, |max| upper_count < max) {
                available_types.push(2);
            }
            if self.constraints.max_symbol.map_or(true, |max| symbol_count < max) {
                available_types.push(3);
            }

            if available_types.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "Cannot satisfy length requirement with given max constraints",
                ));
            }

            // Pick a random available type and increment its count
            let type_index = SecureRandom::random_range(available_types.len())?;
            match available_types[type_index] {
                0 => numeric_count += 1,
                1 => lower_count += 1,
                2 => upper_count += 1,
                3 => symbol_count += 1,
                _ => unreachable!(),
            }

            remaining -= 1;
        }

        // Build password
        let mut password = Vec::new();

        // Add characters according to final counts
        for _ in 0..numeric_count {
            password.push(self.pick_random(&self.numeric_chars)?);
        }
        for _ in 0..lower_count {
            password.push(self.pick_random(&self.lowercase_chars)?);
        }
        for _ in 0..upper_count {
            password.push(self.pick_random(&self.uppercase_chars)?);
        }
        for _ in 0..symbol_count {
            password.push(self.pick_random(&self.symbol_chars)?);
        }

        // Shuffle to avoid predictable patterns
        SecureRandom::shuffle(&mut password)?;

        Ok(password.into_iter().collect())
    }

    /// Pick a random character from a character set
    fn pick_random(&self, chars: &[char]) -> io::Result<char> {
        if chars.is_empty() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Character set is empty",
            ));
        }
        let index = SecureRandom::random_range(chars.len())?;
        Ok(chars[index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_password_generation() {
        let constraints = PasswordConstraints {
            min_numeric: None,
            max_numeric: None,
            min_lower: None,
            max_lower: None,
            min_upper: None,
            max_upper: None,
            min_symbol: None,
            max_symbol: None,
            min_length: 16,
            max_length: 16,
            symbols: "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string(),
            exclude_ambiguous: false,
        };

        let generator = PasswordGenerator::new(constraints).unwrap();
        let password = generator.generate().unwrap();

        assert_eq!(password.len(), 16);
    }

    #[test]
    fn test_password_with_requirements() {
        let constraints = PasswordConstraints {
            min_numeric: Some(2),
            max_numeric: Some(4),
            min_lower: Some(2),
            max_lower: Some(4),
            min_upper: Some(2),
            max_upper: Some(4),
            min_symbol: Some(2),
            max_symbol: Some(4),
            min_length: 16,
            max_length: 16,
            symbols: "!@#$".to_string(),
            exclude_ambiguous: false,
        };

        let generator = PasswordGenerator::new(constraints).unwrap();
        let password = generator.generate().unwrap();

        assert_eq!(password.len(), 16);

        // Count character types
        let numeric_count = password.chars().filter(|c| c.is_numeric()).count();
        let lower_count = password.chars().filter(|c| c.is_lowercase()).count();
        let upper_count = password.chars().filter(|c| c.is_uppercase()).count();
        let symbol_count = password.chars().filter(|c| "!@#$".contains(*c)).count();

        assert!(numeric_count >= 2 && numeric_count <= 4);
        assert!(lower_count >= 2 && lower_count <= 4);
        assert!(upper_count >= 2 && upper_count <= 4);
        assert!(symbol_count >= 2 && symbol_count <= 4);
    }

    #[test]
    fn test_exclude_ambiguous() {
        let constraints = PasswordConstraints {
            min_numeric: Some(5),
            max_numeric: Some(5),
            min_lower: Some(5),
            max_lower: Some(5),
            min_upper: Some(5),
            max_upper: Some(5),
            min_symbol: None,
            max_symbol: None,
            min_length: 15,
            max_length: 15,
            symbols: "!@#$".to_string(),
            exclude_ambiguous: true,
        };

        let generator = PasswordGenerator::new(constraints).unwrap();
        let password = generator.generate().unwrap();

        // Ensure no ambiguous characters
        for c in password.chars() {
            assert!(!AMBIGUOUS_CHARS.contains(&c));
        }
    }

    #[test]
    fn test_validation_min_greater_than_max() {
        let constraints = PasswordConstraints {
            min_numeric: Some(10),
            max_numeric: Some(5),
            min_lower: None,
            max_lower: None,
            min_upper: None,
            max_upper: None,
            min_symbol: None,
            max_symbol: None,
            min_length: 16,
            max_length: 16,
            symbols: "!@#$".to_string(),
            exclude_ambiguous: false,
        };

        assert!(constraints.validate().is_err());
    }

    #[test]
    fn test_validation_requirements_exceed_length() {
        let constraints = PasswordConstraints {
            min_numeric: Some(10),
            max_numeric: Some(10),
            min_lower: Some(10),
            max_lower: Some(10),
            min_upper: None,
            max_upper: None,
            min_symbol: None,
            max_symbol: None,
            min_length: 15,
            max_length: 15,
            symbols: "!@#$".to_string(),
            exclude_ambiguous: false,
        };

        assert!(constraints.validate().is_err());
    }
}
