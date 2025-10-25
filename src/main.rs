mod random;
mod generator;

use clap::Parser;
use generator::{PasswordConstraints, PasswordGenerator};
use std::process;

/// A lightweight, flexible password generator
#[derive(Parser, Debug)]
#[command(name = "genpass")]
#[command(version, about, long_about = None)]
struct Cli {
    /// Minimum number of numeric characters (0-9)
    #[arg(long)]
    min_numeric: Option<usize>,

    /// Maximum number of numeric characters (0-9)
    #[arg(long)]
    max_numeric: Option<usize>,

    /// Minimum number of lowercase letters (a-z)
    #[arg(long)]
    min_lower: Option<usize>,

    /// Maximum number of lowercase letters (a-z)
    #[arg(long)]
    max_lower: Option<usize>,

    /// Minimum number of uppercase letters (A-Z)
    #[arg(long)]
    min_upper: Option<usize>,

    /// Maximum number of uppercase letters (A-Z)
    #[arg(long)]
    max_upper: Option<usize>,

    /// Minimum number of symbol characters
    #[arg(long)]
    min_symbol: Option<usize>,

    /// Maximum number of symbol characters
    #[arg(long)]
    max_symbol: Option<usize>,

    /// Exact password length (shorthand for setting both min and max length)
    #[arg(long, conflicts_with_all = ["min_length", "max_length"])]
    length: Option<usize>,

    /// Minimum total password length
    #[arg(long)]
    min_length: Option<usize>,

    /// Maximum total password length
    #[arg(long)]
    max_length: Option<usize>,

    /// Define which symbol characters to use
    #[arg(long, default_value = "!@#$%^&*()_+-=[]{}|;:,.<>?")]
    symbols: String,

    /// Exclude visually ambiguous characters (0/O, 1/l/I, etc.)
    #[arg(long)]
    exclude_ambiguous: bool,

    /// Number of passwords to generate
    #[arg(long, short = 'c', default_value = "1")]
    count: usize,
}

fn main() {
    let cli = Cli::parse();

    // Determine password length constraints
    let (min_length, max_length) = if let Some(length) = cli.length {
        (length, length)
    } else {
        let min = cli.min_length.unwrap_or(16); // Default minimum length
        let max = cli.max_length.unwrap_or(min); // Default max equals min
        (min, max)
    };

    // Build password constraints
    let constraints = PasswordConstraints {
        min_numeric: cli.min_numeric,
        max_numeric: cli.max_numeric,
        min_lower: cli.min_lower,
        max_lower: cli.max_lower,
        min_upper: cli.min_upper,
        max_upper: cli.max_upper,
        min_symbol: cli.min_symbol,
        max_symbol: cli.max_symbol,
        min_length,
        max_length,
        symbols: cli.symbols,
        exclude_ambiguous: cli.exclude_ambiguous,
    };

    // Create password generator
    let generator = match PasswordGenerator::new(constraints) {
        Ok(generator) => generator,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };

    // Generate passwords
    for _ in 0..cli.count {
        match generator.generate() {
            Ok(password) => println!("{}", password),
            Err(e) => {
                eprintln!("Error generating password: {}", e);
                process::exit(1);
            }
        }
    }
}
