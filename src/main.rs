use clap::Parser;
use rand::rngs::OsRng;
use rand::seq::SliceRandom; // Trait for choosing random elements from a slice

// Define charsets as byte slices for efficiency and immutability.
// These will be combined to form the pool of characters for password generation.
const UPPERCASE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const LOWERCASE: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const DIGITS: &[u8] = b"0123456789";
const SYMBOLS: &[u8] = b"!@#$%^&*()-_=+[]{};:,.<>?";

/// Secure Password Generator
///
/// Generates cryptographically secure passwords with configurable length and character sets.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Password length
    #[arg(short, long, default_value_t = 16)]
    length: usize,

    /// Exclude symbols from the password
    #[arg(long)]
    no_symbols: bool,

    /// Exclude digits from the password
    #[arg(long)]
    no_numbers: bool,

    /// Use only letters (uppercase and lowercase)
    /// This is equivalent to --no-symbols --no-numbers
    #[arg(long)]
    only_letters: bool,
}

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
    // Ensure we have a valid charset to avoid runtime panics.
    if charset.is_empty() {
        return String::new();
    }

    // We use OsRng directly for cryptographic security.
    let mut rng = OsRng;

    // We collect the characters into a String.
    (0..length)
        .map(|_| {
            *charset
                .choose(&mut rng)
                .expect("Charset must not be empty") as char
        })
        .collect()
}

fn main() {
    let args = Cli::parse();

    // Validation: Length must be >= 8
    if args.length < 8 {
        eprintln!("Error: Password length must be at least 8 characters.");
        std::process::exit(1);
    }

    // Construct the charset based on flags
    let mut charset = Vec::new();

    // Logic:
    // 1. Always include letters (Upper + Lower) unless explicitly excluded (no flag for that yet).
    // 2. If --only-letters is set, we skip digits and symbols.
    // 3. Otherwise, include digits unless --no-numbers is set.
    // 4. Otherwise, include symbols unless --no-symbols is set.

    charset.extend_from_slice(UPPERCASE);
    charset.extend_from_slice(LOWERCASE);

    if args.only_letters {
        // Do not add digits or symbols
    } else {
        if !args.no_numbers {
            charset.extend_from_slice(DIGITS);
        }
        if !args.no_symbols {
            charset.extend_from_slice(SYMBOLS);
        }
    }

    // Ensure we have a charset (sanity check, though letters are currently always added)
    if charset.is_empty() {
        eprintln!("Error: Character set is empty. Please check your flags.");
        std::process::exit(1);
    }

    let password = generate_password(args.length, &charset);
    println!("{}", password);
}
