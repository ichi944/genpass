# genpass

A lightweight, flexible password generator written in Rust.

**Why genpass?** Most password generators are inflexible or bloated. This tool gives you fine-grained control over exactly what characters go into your passwords, with a tiny binary footprint (~491KB).

## Key Features

- 🎯 **Precise character control** - Set min/max limits for each character type independently
- 🔧 **Custom symbol sets** - Define exactly which symbols to use (perfect for systems with special character restrictions)
- 💾 **Multiple named configurations** - Save different profiles for work, personal, high-security accounts, etc.
- 🪶 **Lightweight** - Zero dependencies for password generation, only stdlib + `/dev/urandom`
- ⚡ **Fast** - Generate passwords instantly with minimal overhead

## Installation

```bash
# Build from source
cargo build --release
cp ./target/release/genpass ~/bin/  # or your preferred location

# Run directly without installing
cargo run -- [options]
```

## Quick Start

```bash
# Generate a simple 16-character password
genpass

# Generate a 20-character password with at least 3 numbers
genpass -l 20 -n 3

# Generate 5 passwords at once
genpass -c 5
```

## Real-World Examples

### Example 1: Passwords for Strict Corporate Systems

Some corporate systems have annoying requirements like "exactly 2-4 symbols, no more."

```bash
# Password with exactly 12 chars: at least 2 uppercase, 2 numbers, max 4 symbols
genpass -l 12 -u 2 -n 2 -S 4

# Example output: aB7$kLm3@nPq
```

### Example 2: Custom Symbol Set for Legacy Systems

Old systems sometimes reject certain symbols. Use `--symbols` to define exactly what's allowed.

```bash
# Only use symbols that work with old databases: !@#$%
genpass -l 16 --symbols '!@#$%' -s 2

# Example output: k7@mPqR#tWxY2zAb

# No symbols at all (set max to 0)
genpass -l 12 -S 0

# Example output: k7mPqRtWxY2z
```

### Example 3: Multiple Named Configurations

Tired of typing the same options every time? Save multiple named configurations for different use cases!

```bash
# Save a "work" configuration for corporate systems
genpass -l 20 -n 3 -u 2 -s 1 --exclude-ambiguous --save-config work

# Save a "secure" configuration for high-security accounts
genpass -l 32 -n 4 -u 4 -s 4 --save-config secure

# Save a default configuration (used when no --config is specified)
genpass -l 16 -n 2 --save-config ""

# List all your saved configurations
genpass --list-configs
# Output:
#   Available configurations:
#     default
#     secure
#     work

# Use a named configuration
genpass --config work      # Uses work settings (20 chars, 3+ numbers, etc.)
genpass --config secure    # Uses secure settings (32 chars, 4+ of each type)
genpass                    # Uses default configuration

# Override specific options from a saved config
genpass --config work -c 5      # Use work config but generate 5 passwords
genpass --config secure -l 24   # Use secure config but with 24 chars instead
```

**Why this matters:** If you generate passwords for different systems with different requirements, you shouldn't have to remember and retype the same constraints every time.

### Example 4: Passwords Without Ambiguous Characters

Avoid characters that look similar (0/O, 1/l/I, etc.) - great for passwords you need to read aloud or type from a printout.

```bash
genpass -l 16 --exclude-ambiguous

# Excludes: 0, O, 1, l, I
# Example output: 2BkPqRtWxYzAb3mn
```

### Example 5: Flexible Length with Character Requirements

Sometimes you want a password between 16-24 characters with certain minimums.

```bash
# Between 16-24 chars, at least 3 numbers, at least 2 uppercase, at least 1 symbol
genpass --min-length 16 --max-length 24 -n 3 -u 2 -s 1
```

## All Options

### Character Type Controls

Each character type can have independent min/max constraints:

```bash
-n, --min-numeric <n>    Minimum number of digits (0-9)
-N, --max-numeric <n>    Maximum number of digits (0-9)

-a, --min-lower <n>      Minimum lowercase letters (a-z)
-A, --max-lower <n>      Maximum lowercase letters (a-z)

-u, --min-upper <n>      Minimum uppercase letters (A-Z)
-U, --max-upper <n>      Maximum uppercase letters (A-Z)

-s, --min-symbol <n>     Minimum symbol characters
-S, --max-symbol <n>     Maximum symbol characters
```

### Password Length

```bash
-l, --length <n>         Exact password length
    --min-length <n>     Minimum password length (default: 16)
    --max-length <n>     Maximum password length
```

### Symbol Customization

```bash
    --symbols <chars>    Define exactly which symbols to use
                         Default: !@#$%^&*()_+-=[]{}|;:,.<>?

    --exclude-ambiguous  Exclude visually similar characters
                         (0/O, 1/l/I, etc.)
```

### Output & Configuration

```bash
-c, --count <n>              Number of passwords to generate (default: 1)
    --config <name>          Load a named configuration
    --save-config <name>     Save current options to a named config
                             (use empty string "" for default)
    --list-configs           List all available saved configurations
```

## Configuration Files

Settings are saved to `~/.genpass/` directory with each configuration as a separate file:

- `~/.genpass/default` - Default configuration (loaded when no `--config` is specified)
- `~/.genpass/work` - Named "work" configuration
- `~/.genpass/secure` - Named "secure" configuration
- etc.

Each config file uses simple `key=value` format:

```
length=20
min-numeric=3
min-upper=2
exclude-ambiguous=true
symbols=!@#$%^&*
```

CLI arguments always override saved configuration.

## Tips & Tricks

### Clipboard Integration (macOS)

```bash
# Add to ~/.zshrc to auto-copy generated passwords
alias genpass='genpass | pbcopy && pbpaste'

# Now genpass automatically copies to clipboard and shows the password
genpass
```

### Clipboard Integration (Linux)

```bash
# Using xclip
alias genpass='genpass | xclip -selection clipboard && xclip -selection clipboard -o'

# Using wl-clipboard (Wayland)
alias genpass='genpass | wl-copy && wl-paste'
```

### Quick Password Variations

```bash
# Short PIN-like passwords (numbers only)
genpass -l 6 -S 0 -A 0 -U 0

# Passphrase-style (letters only, no symbols)
genpass -l 24 -S 0 -N 0

# Maximum security (long with everything)
genpass -l 32 -n 4 -u 4 -s 4
```

## Why I Built This

I got frustrated with password generators that either:
1. Don't let you control which symbols are used (and some systems reject certain symbols)
2. Don't let you set maximum limits (some old systems have weird rules like "max 3 symbols")
3. Make you type the same long command every time you need a password for different systems

This tool solves all three problems with `--symbols`, `-S/--max-symbol`, and multiple named configurations via `--save-config` and `--config`.

## Development

```bash
# Run with options during development
cargo run -- -l 20 -n 3

# Build optimized binary
cargo build --release

# Run tests
cargo test

# Check code
cargo clippy
cargo fmt
```

## License

MIT
