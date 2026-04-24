// Week 14 — generator.rs
//
// Implement the three password generation functions below.
// Each function must produce output that satisfies the constraints in its docstring.
// The tests at the bottom verify your implementations.

#![allow(dead_code, unused_imports)]
use rand::Rng;

/// Generates a random password of the given `length`.
pub fn generate_random(length: usize, use_symbols: bool) -> String {
    if length == 0 {
        panic!("Length must be greater than 0");
    }

    let mut charset =
        String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789");
    if use_symbols {
        charset.push_str("!@#$%^&*");
    }

    let mut rng = rand::thread_rng();
    let charset_bytes = charset.as_bytes();

    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset_bytes.len());
            charset_bytes[idx] as char
        })
        .collect()
}

/// Generates a passphrase made of `word_count` random common English words joined by `separator`.
pub fn generate_passphrase(word_count: usize, separator: char) -> String {
    let mut rng = rand::thread_rng();
    let mut words = Vec::new();

    for _ in 0..word_count {
        let idx = rng.gen_range(0..WORD_LIST.len());
        words.push(WORD_LIST[idx]);
    }

    words.join(&separator.to_string())
}

/// Generates a numeric PIN of the given `length` (digits 0–9 only).
pub fn generate_pin(length: usize) -> String {
    if length == 0 {
        panic!("Length must be greater than 0");
    }

    let mut rng = rand::thread_rng();
    (0..length)
        .map(|_| {
            let digit = rng.gen_range(0..10);
            char::from_digit(digit, 10).unwrap()
        })
        .collect()
}

// A small word list for passphrases.
pub const WORD_LIST: &[&str] = &[
    "apple", "river", "cloud", "stone", "flame", "ocean", "tiger", "maple", "storm", "frost",
    "eagle", "cedar", "brook", "ember", "coral", "prism", "solar", "lunar", "amber", "blaze",
    "cliff", "delta", "fable", "grove", "haven", "ivory", "jewel", "knoll", "lemon", "meadow",
    "north", "olive", "pearl", "quill", "ridge", "spark", "thorn", "umbra", "valor", "willow",
    "xenon", "yarrow", "zenith", "acorn", "birch", "crane", "drift", "elder", "flint", "glade",
    "hyena", "inlet", "junco", "kestrel",
];

// ============================================================================
// TESTS — DO NOT MODIFY
// ============================================================================
#[cfg(test)]
mod tests {
    use super::*;

    // --- generate_random ---

    #[test]
    fn test_random_correct_length() {
        let pwd = generate_random(12, false);
        assert_eq!(pwd.len(), 12);
    }

    #[test]
    fn test_random_no_symbols_only_alphanumeric() {
        let pwd = generate_random(100, false);
        assert!(
            pwd.chars().all(|c| c.is_ascii_alphanumeric()),
            "Password without symbols should only contain alphanumeric characters"
        );
    }

    #[test]
    fn test_random_with_symbols_contains_valid_chars() {
        let valid: std::collections::HashSet<char> =
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*"
                .chars()
                .collect();
        let pwd = generate_random(100, true);
        assert!(
            pwd.chars().all(|c| valid.contains(&c)),
            "Password with symbols must only contain chars from the allowed set"
        );
    }

    #[test]
    fn test_random_length_one() {
        let pwd = generate_random(1, false);
        assert_eq!(pwd.len(), 1);
    }

    // --- generate_passphrase ---

    #[test]
    fn test_passphrase_word_count() {
        let phrase = generate_passphrase(4, '-');
        assert_eq!(phrase.split('-').count(), 4);
    }

    #[test]
    fn test_passphrase_separator() {
        let phrase = generate_passphrase(3, '_');
        assert!(phrase.contains('_'));
        assert!(!phrase.contains('-'));
    }

    #[test]
    fn test_passphrase_words_from_list() {
        let phrase = generate_passphrase(5, '-');
        for word in phrase.split('-') {
            assert!(
                WORD_LIST.contains(&word),
                "Word '{}' is not in WORD_LIST",
                word
            );
        }
    }

    #[test]
    fn test_passphrase_single_word() {
        let phrase = generate_passphrase(1, '-');
        assert!(!phrase.contains('-'));
        assert!(WORD_LIST.contains(&phrase.as_str()));
    }

    // --- generate_pin ---

    #[test]
    fn test_pin_correct_length() {
        let pin = generate_pin(6);
        assert_eq!(pin.len(), 6);
    }

    #[test]
    fn test_pin_only_digits() {
        let pin = generate_pin(20);
        assert!(
            pin.chars().all(|c| c.is_ascii_digit()),
            "PIN must contain only digits"
        );
    }

    #[test]
    fn test_pin_length_one() {
        let pin = generate_pin(1);
        assert_eq!(pin.len(), 1);
        assert!(pin.chars().all(|c| c.is_ascii_digit()));
    }
}
