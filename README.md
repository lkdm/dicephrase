# dicephrase

Generate strong, memorable passphrases securely and entirely offline.

`dicephrase` uses your operating system's cryptographically secure random number generator (CSPRNG) to generate Diceware passphrases using the EFF Large Diceware Wordlist.

1. Generates cryptographically secure random numbers.
1. Uses those numbers to select words from the [EFF Large Diceware Wordlist](https://www.eff.org/dice).
1. Combines the words into a passphrase with sensible defaults.
1. Copies the passphrase to clipboard and displays a QR by default.

**Nothing is saved to disk, logged, or transmitted. The wordlist is included in the binary.**

## Installation

Requires [Rust](https://rustup.rs).

```bash
cargo install --git https://github.com/lkdm/dicephrase.git
```

## Usage

Generate a six-word passphrase, with sentence case, and a trailing number (default):

```bash
dicephrase
```

By default, passphrases are copied to clipboard and a QR code is displayed.

## Examples

```bash
dicephrase 8
dicephrase 6 --case upper
dicephrase 8 --separator underscore
dicephrase 8 --number random
dicephrase --no-clipboard
dicephrase --no-qr
```

Run `dicephrase --help` for all available options.

## Security

- 7,776-word EFF Diceware wordlist.
- Approximately 12.92 bits of entropy per word.
- Six words provide approximately 77.55 bits of entropy.
- Uses your operating system's CSPRNG.
- Supports passphrases of 6–100 words.

Never reuse passphrases across accounts.

## EFF Recommendations

[EFF Security Recommendations](https://www.eff.org/files/2025/08/19/diceware.pdf)

- **Accept the random result.** Don't regenerate until you get words you like. Choosing preferred combinations reduces randomness.
- **Don't edit the passphrase.** Avoid changing, removing, or rearranging words.
- **Make it memorable.** Create a mental image or short story connecting the words.
- **Be consistent.** Decide on capitalization and separators before using your passphrase. Always enter it the same way.
- **Never reuse passphrases.** Use a unique passphrase for every account and purpose.
- **Store it securely.** Keep a written copy somewhere safe, such as in a password manager, while memorising it.
