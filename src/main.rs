use rand::rngs::OsRng;
use rand::seq::SliceRandom; // Trait for choosing random elements from a slice

// Define charsets as byte slices for efficiency and immutability.
// These will be combined to form the pool of characters for password generation.
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?";

/// Generates a cryptographically secure password of the specified length.
///
/// # Arguments
///
/// * `length` - The length of the password to generate.
/// * `charset` - A slice of bytes representing the allowed characters.
///
/// # Returns
///
/// A `String` containing the generated password.
///
/// # Security
///
/// - Uses `rand::rngs::OsRng` to ensure randomness is sourced from the operating system's
///   CSPRNG (Cryptographically Secure Pseudo-Random Number Generator).
/// - We explicitly avoid `rand::thread_rng` because while it is currently secure,
///   `OsRng` is the most direct interface to the OS entropy source, minimizing user-space buffering
///   or state that could theoretically be compromised or seeded poorly in some environments.
/// - Uses `SliceRandom::choose` which guarantees uniform distribution (no modulo bias)
///   when selecting characters from the charset.
fn generate_password(length: usize, charset: &[u8]) -> String {
    // Ensure we have a valid charset to avoid runtime panics or infinite loops if logic were different.
    // In this specific implementation, `choose` would return None if empty, so we should handle that.
    if charset.is_empty() {
        return String::new();
    }

    // We use OsRng directly for cryptographic security.
    // This ensures we are getting high-quality entropy from the OS.
    let mut rng = OsRng;

    // We collect the characters into a String.
    // (0..length) creates an iterator that yields `length` items.
    // `map` generates a random character for each iteration.
    // `choose` selects a random element from the slice uniformly.
    // It returns an Option, but since we checked !charset.is_empty(), unwrap is safe here.
    // However, for robustness in a library context, we might handle it, but for this internal helper, it's fine.
    (0..length)
        .map(|_| {
            *charset
                .choose(&mut rng)
                .expect("Charset must not be empty") as char
        })
        .collect()
}

fn main() {
    // Phase 1 Demo: Hardcoded 16-character password generation.
    
    // Combine all charsets for the demo.
    // We use a Vec<u8> to build the combined charset dynamically.
    let mut all_chars = Vec::new();
    all_chars.extend_from_slice(UPPERCASE);
    all_chars.extend_from_slice(LOWERCASE);
    all_chars.extend_from_slice(DIGITS);
    all_chars.extend_from_slice(SYMBOLS);

    let password_length = 16;
    let password = generate_password(password_length, &all_chars);

    println!("Secure Password Generator (Phase 1 Demo)");
    println!("----------------------------------------");
    println!("Generated Password (length {}): {}", password_length, password);
}
