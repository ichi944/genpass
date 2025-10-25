```
# Run on source
cargo run
# With options
cargo run -- --length 20 --min-numeric 3 --exclude-ambiguous

# Build
cargo build --release
cp ./target/release/genpass ~/[your]/[favorite]/[location]/
genpass

# On Mac, if you want to send the strings to clipboard, you can add below to ~/.zshrc
alias genpass='genpass | pbcopy && pbpaste'

```
