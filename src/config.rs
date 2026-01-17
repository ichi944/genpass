use std::fs;
use std::io::{self, Write};
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
    pub copy: Option<bool>,
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
        if let Some(v) = self.copy {
            content.push_str(&format!("copy={}\n", v));
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
                    "copy" => config.copy = value.parse().ok(),
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
        if cli.copy {
            self.copy = Some(true);
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
