use std::fs;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;

/// Configuration that can be saved and loaded from ~/.genpassconfig
#[derive(Debug, Clone, Default)]
pub struct Config {
    pub min_numeric: Option<usize>,
    pub max_numeric: Option<usize>,
    pub min_lower: Option<usize>,
    pub max_lower: Option<usize>,
    pub min_upper: Option<usize>,
    pub max_upper: Option<usize>,
    pub min_symbol: Option<usize>,
    pub max_symbol: Option<usize>,
    pub length: Option<usize>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub symbols: Option<String>,
    pub exclude_ambiguous: Option<bool>,
    pub count: Option<usize>,
}

impl Config {
    /// Get the path to the config directory (~/.genpass/)
    pub fn config_dir() -> io::Result<PathBuf> {
        let home = std::env::var("HOME").map_err(|_| {
            io::Error::new(io::ErrorKind::NotFound, "HOME environment variable not set")
        })?;
        Ok(PathBuf::from(home).join(".genpass"))
    }

    /// Get the path to a named config file
    /// If name is None or "default", returns ~/.genpass/default
    pub fn config_path(name: Option<&str>) -> io::Result<PathBuf> {
        let dir = Self::config_dir()?;
        let filename = name.unwrap_or("default");
        Ok(dir.join(filename))
    }

    /// Load configuration from a named config
    /// Returns default config if file doesn't exist
    pub fn load(name: Option<&str>) -> io::Result<Self> {
        let path = Self::config_path(name)?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)?;
        Self::parse(&content)
    }

    /// Save configuration to a named config file
    /// Overwrites existing file if present
    pub fn save(&self, name: Option<&str>) -> io::Result<()> {
        // Ensure config directory exists
        let dir = Self::config_dir()?;
        if !dir.exists() {
            fs::create_dir_all(&dir)?;
        }

        let path = Self::config_path(name)?;
        let mut content = String::new();

        content.push_str("# genpass configuration file\n");
        content.push_str("# Generated automatically - edit with caution\n\n");

        if let Some(v) = self.min_numeric {
            content.push_str(&format!("min-numeric={}\n", v));
        }
        if let Some(v) = self.max_numeric {
            content.push_str(&format!("max-numeric={}\n", v));
        }
        if let Some(v) = self.min_lower {
            content.push_str(&format!("min-lower={}\n", v));
        }
        if let Some(v) = self.max_lower {
            content.push_str(&format!("max-lower={}\n", v));
        }
        if let Some(v) = self.min_upper {
            content.push_str(&format!("min-upper={}\n", v));
        }
        if let Some(v) = self.max_upper {
            content.push_str(&format!("max-upper={}\n", v));
        }
        if let Some(v) = self.min_symbol {
            content.push_str(&format!("min-symbol={}\n", v));
        }
        if let Some(v) = self.max_symbol {
            content.push_str(&format!("max-symbol={}\n", v));
        }
        if let Some(v) = self.length {
            content.push_str(&format!("length={}\n", v));
        }
        if let Some(v) = self.min_length {
            content.push_str(&format!("min-length={}\n", v));
        }
        if let Some(v) = self.max_length {
            content.push_str(&format!("max-length={}\n", v));
        }
        if let Some(ref v) = self.symbols {
            content.push_str(&format!("symbols={}\n", v));
        }
        if let Some(v) = self.exclude_ambiguous {
            content.push_str(&format!("exclude-ambiguous={}\n", v));
        }
        if let Some(v) = self.count {
            content.push_str(&format!("count={}\n", v));
        }

        let mut file = fs::File::create(&path)?;
        file.write_all(content.as_bytes())?;

        Ok(())
    }

    /// List all available config names
    pub fn list_configs() -> io::Result<Vec<String>> {
        let dir = Self::config_dir()?;

        if !dir.exists() {
            return Ok(Vec::new());
        }

        let mut configs = Vec::new();
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            if entry.path().is_file() {
                if let Some(name) = entry.file_name().to_str() {
                    configs.push(name.to_string());
                }
            }
        }

        configs.sort();
        Ok(configs)
    }

    /// Display the configuration in a human-readable format
    pub fn display(&self, name: Option<&str>) {
        let config_name = name.unwrap_or("default");
        println!("Configuration: {}", config_name);
        println!();

        // Character type constraints
        println!("Character Type Constraints:");
        Self::display_constraint("  Numeric (0-9)", self.min_numeric, self.max_numeric);
        Self::display_constraint("  Lowercase (a-z)", self.min_lower, self.max_lower);
        Self::display_constraint("  Uppercase (A-Z)", self.min_upper, self.max_upper);
        Self::display_constraint("  Symbols", self.min_symbol, self.max_symbol);
        println!();

        // Password length
        println!("Password Length:");
        if let Some(length) = self.length {
            println!("  Exact length: {}", length);
        } else {
            if let Some(min) = self.min_length {
                println!("  Minimum: {}", min);
            } else {
                println!("  Minimum: 16 (default)");
            }
            if let Some(max) = self.max_length {
                println!("  Maximum: {}", max);
            }
        }
        println!();

        // Symbol characters
        println!("Symbol Characters:");
        if let Some(ref symbols) = self.symbols {
            println!("  {}", symbols);
        } else {
            println!("  !@#$%^&*()_+-=[]{{}}|;:,.<>? (default)");
        }
        println!();

        // Other options
        println!("Options:");
        match self.exclude_ambiguous {
            Some(true) => println!("  Exclude ambiguous characters: yes"),
            Some(false) => println!("  Exclude ambiguous characters: no"),
            None => println!("  Exclude ambiguous characters: no (default)"),
        }
        if let Some(count) = self.count {
            println!("  Password count: {}", count);
        } else {
            println!("  Password count: 1 (default)");
        }
    }

    /// Helper to display min/max constraints
    fn display_constraint(label: &str, min: Option<usize>, max: Option<usize>) {
        match (min, max) {
            (Some(min_val), Some(max_val)) if min_val == max_val => {
                println!("{}: exactly {}", label, min_val);
            }
            (Some(min_val), Some(max_val)) => {
                println!("{}: {} to {}", label, min_val, max_val);
            }
            (Some(min_val), None) => {
                println!("{}: minimum {}", label, min_val);
            }
            (None, Some(max_val)) => {
                println!("{}: maximum {}", label, max_val);
            }
            (None, None) => {
                println!("{}: no constraint", label);
            }
        }
    }

    /// Interactive wizard to configure password generation
    pub fn wizard() -> io::Result<(Self, Option<String>)> {
        let stdin = io::stdin();
        let mut reader = stdin.lock();
        let mut config = Self::default();

        println!("=== Password Generator Configuration Wizard ===");
        println!();

        // Password length
        println!("Password Length:");
        println!("  Choose an option:");
        println!("  1. Exact length (recommended)");
        println!("  2. Range (min to max)");
        let length_choice = Self::read_choice(&mut reader, &["1", "2"])?;

        if length_choice == "1" {
            config.length = Some(Self::read_number(&mut reader, "  Enter password length", Some(16))?);
        } else {
            config.min_length = Some(Self::read_number(&mut reader, "  Enter minimum length", Some(12))?);
            config.max_length = Some(Self::read_number(&mut reader, "  Enter maximum length", Some(20))?);
        }
        println!();

        // Character type constraints
        println!("Character Type Requirements:");
        println!("  (Press Enter to skip any constraint)");
        println!();

        config.min_numeric = Self::read_optional_number(&mut reader, "  Minimum numeric characters (0-9)")?;
        config.max_numeric = Self::read_optional_number(&mut reader, "  Maximum numeric characters (0-9)")?;
        println!();

        config.min_lower = Self::read_optional_number(&mut reader, "  Minimum lowercase letters (a-z)")?;
        config.max_lower = Self::read_optional_number(&mut reader, "  Maximum lowercase letters (a-z)")?;
        println!();

        config.min_upper = Self::read_optional_number(&mut reader, "  Minimum uppercase letters (A-Z)")?;
        config.max_upper = Self::read_optional_number(&mut reader, "  Maximum uppercase letters (A-Z)")?;
        println!();

        config.min_symbol = Self::read_optional_number(&mut reader, "  Minimum symbol characters")?;
        config.max_symbol = Self::read_optional_number(&mut reader, "  Maximum symbol characters")?;
        println!();

        // Symbol characters
        println!("Symbol Characters:");
        println!("  Default: !@#$%^&*()_+-=[]{{}}|;:,.<>?");
        print!("  Use custom symbols? (y/N): ");
        io::stdout().flush()?;
        if Self::read_yes_no(&mut reader, false)? {
            config.symbols = Some(Self::read_string(&mut reader, "  Enter symbols to use")?);
        }
        println!();

        // Exclude ambiguous
        println!("Options:");
        print!("  Exclude ambiguous characters (0/O, 1/l/I)? (y/N): ");
        io::stdout().flush()?;
        config.exclude_ambiguous = Some(Self::read_yes_no(&mut reader, false)?);
        println!();

        // Password count
        config.count = Some(Self::read_number(&mut reader, "  Number of passwords to generate", Some(1))?);
        println!();

        // Summary
        println!("=== Configuration Summary ===");
        config.display(None);
        println!();

        // Save configuration
        print!("Save this configuration? (y/N): ");
        io::stdout().flush()?;
        let save_name = if Self::read_yes_no(&mut reader, false)? {
            print!("  Configuration name (default): ");
            io::stdout().flush()?;
            let mut name = String::new();
            reader.read_line(&mut name)?;
            let name = name.trim();
            if name.is_empty() {
                None
            } else {
                Some(name.to_string())
            }
        } else {
            None
        };

        Ok((config, save_name))
    }

    /// Read a line from stdin
    fn read_line(reader: &mut io::StdinLock) -> io::Result<String> {
        let mut input = String::new();
        reader.read_line(&mut input)?;
        Ok(input.trim().to_string())
    }

    /// Read a number with optional default
    fn read_number(reader: &mut io::StdinLock, prompt: &str, default: Option<usize>) -> io::Result<usize> {
        loop {
            if let Some(def) = default {
                print!("{} [{}]: ", prompt, def);
            } else {
                print!("{}: ", prompt);
            }
            io::stdout().flush()?;

            let input = Self::read_line(reader)?;

            if input.is_empty() {
                if let Some(def) = default {
                    return Ok(def);
                }
                println!("  Please enter a number.");
                continue;
            }

            match input.parse::<usize>() {
                Ok(n) => return Ok(n),
                Err(_) => println!("  Invalid number, please try again."),
            }
        }
    }

    /// Read an optional number (can be skipped)
    fn read_optional_number(reader: &mut io::StdinLock, prompt: &str) -> io::Result<Option<usize>> {
        print!("{}: ", prompt);
        io::stdout().flush()?;

        let input = Self::read_line(reader)?;

        if input.is_empty() {
            return Ok(None);
        }

        match input.parse::<usize>() {
            Ok(n) => Ok(Some(n)),
            Err(_) => {
                println!("  Invalid number, skipping.");
                Ok(None)
            }
        }
    }

    /// Read a string
    fn read_string(reader: &mut io::StdinLock, prompt: &str) -> io::Result<String> {
        print!("{}: ", prompt);
        io::stdout().flush()?;
        Self::read_line(reader)
    }

    /// Read yes/no with default
    fn read_yes_no(reader: &mut io::StdinLock, default: bool) -> io::Result<bool> {
        let input = Self::read_line(reader)?;

        if input.is_empty() {
            return Ok(default);
        }

        match input.to_lowercase().as_str() {
            "y" | "yes" => Ok(true),
            "n" | "no" => Ok(false),
            _ => Ok(default),
        }
    }

    /// Read a choice from a list of options
    fn read_choice(reader: &mut io::StdinLock, options: &[&str]) -> io::Result<String> {
        loop {
            print!("  Enter choice: ");
            io::stdout().flush()?;

            let input = Self::read_line(reader)?;

            if options.contains(&input.as_str()) {
                return Ok(input);
            }

            println!("  Invalid choice, please try again.");
        }
    }

    /// Parse configuration from a string
    fn parse(content: &str) -> io::Result<Self> {
        let mut config = Self::default();

        for line in content.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse key=value
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "min-numeric" => config.min_numeric = value.parse().ok(),
                    "max-numeric" => config.max_numeric = value.parse().ok(),
                    "min-lower" => config.min_lower = value.parse().ok(),
                    "max-lower" => config.max_lower = value.parse().ok(),
                    "min-upper" => config.min_upper = value.parse().ok(),
                    "max-upper" => config.max_upper = value.parse().ok(),
                    "min-symbol" => config.min_symbol = value.parse().ok(),
                    "max-symbol" => config.max_symbol = value.parse().ok(),
                    "length" => config.length = value.parse().ok(),
                    "min-length" => config.min_length = value.parse().ok(),
                    "max-length" => config.max_length = value.parse().ok(),
                    "symbols" => config.symbols = Some(value.to_string()),
                    "exclude-ambiguous" => config.exclude_ambiguous = value.parse().ok(),
                    "count" => config.count = value.parse().ok(),
                    _ => {
                        // Unknown keys are ignored for forward compatibility
                    }
                }
            }
        }

        Ok(config)
    }

    /// Merge with CLI arguments (CLI args take precedence)
    pub fn merge_with_cli(&mut self, cli: &crate::Cli) {
        if cli.min_numeric.is_some() {
            self.min_numeric = cli.min_numeric;
        }
        if cli.max_numeric.is_some() {
            self.max_numeric = cli.max_numeric;
        }
        if cli.min_lower.is_some() {
            self.min_lower = cli.min_lower;
        }
        if cli.max_lower.is_some() {
            self.max_lower = cli.max_lower;
        }
        if cli.min_upper.is_some() {
            self.min_upper = cli.min_upper;
        }
        if cli.max_upper.is_some() {
            self.max_upper = cli.max_upper;
        }
        if cli.min_symbol.is_some() {
            self.min_symbol = cli.min_symbol;
        }
        if cli.max_symbol.is_some() {
            self.max_symbol = cli.max_symbol;
        }
        if cli.length.is_some() {
            self.length = cli.length;
        }
        if cli.min_length.is_some() {
            self.min_length = cli.min_length;
        }
        if cli.max_length.is_some() {
            self.max_length = cli.max_length;
        }
        // For symbols, check if it's not the default value
        if cli.symbols != "!@#$%^&*()_+-=[]{}|;:,.<>?" {
            self.symbols = Some(cli.symbols.clone());
        } else if self.symbols.is_none() {
            self.symbols = Some(cli.symbols.clone());
        }
        if cli.exclude_ambiguous {
            self.exclude_ambiguous = Some(true);
        }
        // For count, only override if not default
        if cli.count != 1 {
            self.count = Some(cli.count);
        } else if self.count.is_none() {
            self.count = Some(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_config() {
        let content = r#"
# Comment line
min-numeric=2
max-numeric=5
min-length=16
symbols=!@#$
exclude-ambiguous=true
count=3
"#;

        let config = Config::parse(content).unwrap();
        assert_eq!(config.min_numeric, Some(2));
        assert_eq!(config.max_numeric, Some(5));
        assert_eq!(config.min_length, Some(16));
        assert_eq!(config.symbols, Some("!@#$".to_string()));
        assert_eq!(config.exclude_ambiguous, Some(true));
        assert_eq!(config.count, Some(3));
    }

    #[test]
    fn test_parse_empty_config() {
        let content = "";
        let config = Config::parse(content).unwrap();
        assert_eq!(config.min_numeric, None);
        assert_eq!(config.count, None);
    }

    #[test]
    fn test_parse_with_unknown_keys() {
        let content = r#"
min-numeric=2
unknown-key=value
future-option=123
"#;

        let config = Config::parse(content).unwrap();
        assert_eq!(config.min_numeric, Some(2));
    }
}
