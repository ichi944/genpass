mod random;
mod generator;
mod config;

use clap::Parser;
use generator::{PasswordConstraints, PasswordGenerator};
use std::process;

/// A lightweight, flexible password generator
#[derive(Parser, Debug)]
#[command(name = "genpass")]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Minimum number of numeric characters (0-9)
    #[arg(short = 'n', long)]
    pub min_numeric: Option<usize>,

    /// Maximum number of numeric characters (0-9)
    #[arg(short = 'N', long)]
    pub max_numeric: Option<usize>,

    /// Minimum number of lowercase letters (a-z)
    #[arg(short = 'a', long)]
    pub min_lower: Option<usize>,

    /// Maximum number of lowercase letters (a-z)
    #[arg(short = 'A', long)]
    pub max_lower: Option<usize>,

    /// Minimum number of uppercase letters (A-Z)
    #[arg(short = 'u', long)]
    pub min_upper: Option<usize>,

    /// Maximum number of uppercase letters (A-Z)
    #[arg(short = 'U', long)]
    pub max_upper: Option<usize>,

    /// Minimum number of symbol characters
    #[arg(short = 's', long)]
    pub min_symbol: Option<usize>,

    /// Maximum number of symbol characters
    #[arg(short = 'S', long)]
    pub max_symbol: Option<usize>,

    /// Exact password length (shorthand for setting both min and max length)
    #[arg(short = 'l', long, conflicts_with_all = ["min_length", "max_length"])]
    pub length: Option<usize>,

    /// Minimum total password length
    #[arg(long)]
    pub min_length: Option<usize>,

    /// Maximum total password length
    #[arg(long)]
    pub max_length: Option<usize>,

    /// Define which symbol characters to use
    #[arg(long, default_value = "!@#$%^&*()_+-=[]{}|;:,.<>?")]
    pub symbols: String,

    /// Exclude visually ambiguous characters (0/O, 1/l/I, etc.)
    #[arg(long)]
    pub exclude_ambiguous: bool,

    /// Number of passwords to generate
    #[arg(long, short = 'c', default_value = "1")]
    pub count: usize,

    /// Load configuration from a named profile
    #[arg(long)]
    pub config: Option<String>,

    /// Save current options to a named config (default: "default")
    #[arg(long)]
    pub save_config: Option<String>,

    /// List all available saved configurations
    #[arg(long)]
    pub list_configs: bool,
}

fn main() {
    let cli = Cli::parse();

    // List configs if requested and exit
    if cli.list_configs {
        match config::Config::list_configs() {
            Ok(configs) => {
                if configs.is_empty() {
                    println!("No saved configurations found.");
                } else {
                    println!("Available configurations:");
                    for name in configs {
                        println!("  {}", name);
                    }
                }
                return;
            }
            Err(e) => {
                eprintln!("Error listing configurations: {}", e);
                process::exit(1);
            }
        }
    }

    // Load saved configuration
    let config_name = cli.config.as_deref();
    let mut config = match config::Config::load(config_name) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Warning: Could not load config: {}", e);
            config::Config::default()
        }
    };

    // Merge CLI args with config (CLI takes precedence)
    config.merge_with_cli(&cli);

    // Save config if requested
    if let Some(ref save_name) = cli.save_config {
        let name_to_save = if save_name.is_empty() {
            None
        } else {
            Some(save_name.as_str())
        };

        match config.save(name_to_save) {
            Ok(()) => {
                let path = config::Config::config_path(name_to_save).unwrap_or_default();
                eprintln!("Configuration saved to {}", path.display());
            }
            Err(e) => {
                eprintln!("Error saving configuration: {}", e);
                process::exit(1);
            }
        }
    }

    // Determine password length constraints
    let (min_length, max_length) = if let Some(length) = config.length {
        (length, length)
    } else {
        let min = config.min_length.unwrap_or(16); // Default minimum length
        let max = config.max_length.unwrap_or(min); // Default max equals min
        (min, max)
    };

    // Build password constraints
    let constraints = PasswordConstraints {
        min_numeric: config.min_numeric,
        max_numeric: config.max_numeric,
        min_lower: config.min_lower,
        max_lower: config.max_lower,
        min_upper: config.min_upper,
        max_upper: config.max_upper,
        min_symbol: config.min_symbol,
        max_symbol: config.max_symbol,
        min_length,
        max_length,
        symbols: config.symbols.unwrap_or_else(|| "!@#$%^&*()_+-=[]{}|;:,.<>?".to_string()),
        exclude_ambiguous: config.exclude_ambiguous.unwrap_or(false),
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
    let count = config.count.unwrap_or(1);
    for _ in 0..count {
        match generator.generate() {
            Ok(password) => println!("{}", password),
            Err(e) => {
                eprintln!("Error generating password: {}", e);
                process::exit(1);
            }
        }
    }
}
