# dicephrase

A command-line passphrase generator using the EFF Large Diceware Wordlist.

Uses your operating system's cryptographically secure random number generator (CSPRNG).

## Installation

Requires [Rust](https://rustup.rs).

```bash
cargo install --git https://github.com/lkdm/dicephrase.git
```

## Usage

Generate a six-word passphrase (default):

```bash
dicephrase 6
```

Passphrases are copied to the clipboard by default.

## Examples

```bash
dicephrase 8
dicephrase 6 --case upper
dicephrase 8 --separator "_" --number
dicephrase 6 --output stdout
```

## Security

- 7,776-word EFF Diceware wordlist.
- Approximately 12.92 bits of entropy per word.
- Six words provide approximately 77.55 bits of entropy.
- Uses your operating system's CSPRNG.

Use at least six words and never reuse passphrases across accounts.
