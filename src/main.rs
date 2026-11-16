use clap::{Parser, ValueEnum};
use std::error::Error;

pub mod clipboard;
pub mod dice;
pub mod qr;
pub mod words;

use dice::DiceRolls;

use crate::{qr::print_qr, words::entropy};

const HELP: &str = concat!(
    env!("CARGO_PKG_NAME"),
    " ",
    env!("CARGO_PKG_VERSION"),
    "\n",
    env!("CARGO_PKG_DESCRIPTION"),
    "\n\n",
    "Wordlist\n",
    "  EFF Large Diceware (7,776 words)\n",
    "  https://www.eff.org/dice\n\n",
    "Security\n",
    "  Rolls per word:      5\n",
    "  Entropy per word:    12.92 bits\n",
    "  Default words:       6\n",
    "  Default entropy:     77.55 bits\n\n",
    "Randomness\n",
    "  Uses your operating system's cryptographically secure\n",
    "  random number generator (CSPRNG).\n\n",
    "Usage\n",
    "  Generate a new passphrase whenever needed. Each generation\n",
    "  produces an independent random passphrase.\n",
    "  Use six or more words for strong passphrases.\n",
    "  Never reuse a passphrase across different accounts.\n",
    "  Store your passphrases securely.\n\n",
    "Clipboard\n",
    "  Passphrases are copied to the clipboard by default rather\n",
    "  than printed to the terminal.\n\n",
    "Project: ",
    env!("CARGO_PKG_REPOSITORY"),
);

/// Generate a random Diceware passphrase
///
/// https://www.eff.org/dice
#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    author,
    about = env!("CARGO_PKG_DESCRIPTION"),
    long_about = HELP,
)]
struct Args {
    /// Number of words in the passphrase (EFF recommends at least 6)
    #[arg(default_value_t = 6, value_parser = clap::value_parser!(u32).range(1..))]
    words: u32,

    /// Character(s) to insert between each word
    #[arg(short, long, default_value = "-")]
    separator: String,

    /// Passphrase word case
    #[arg(short, long, value_enum, default_value_t = Case::Lower)]
    case: Case,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Clipboard)]
    output: Output,

    /// Clipboard timeout in seconds
    #[arg(short, long, default_value_t = 30)]
    timeout: u32,

    /// Suppress terminal output
    #[arg(short, long, default_value_t = false)]
    quiet: bool,

    /// Add a number
    #[arg(short, long, default_value_t = false)]
    number: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Case {
    Lower,
    Upper,
    Title,
    Random,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Output {
    Clipboard,
    Qr,
    Stdout,
}

fn format_word(word: &str, case: Case) -> String {
    match case {
        Case::Lower => word.to_lowercase(),
        Case::Upper => word.to_uppercase(),
        Case::Title => {
            let mut chars = word.chars();

            match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            }
        }
        Case::Random => {
            // Randomly choose uppercase or lowercase for each word.
            if dice::random_bool().expect("Failed to generate randomness") {
                word.to_uppercase()
            } else {
                word.to_lowercase()
            }
        }
    }
}

fn generate_passphrase(
    word_count: u32,
    separator: &str,
    case: Case,
    number: bool,
) -> Result<String, Box<dyn Error>> {
    let mut words = Vec::with_capacity(word_count as usize);

    for _ in 0..word_count {
        let rolls = DiceRolls::new()?;
        let word = words::get_word(&rolls)?;
        words.push(format_word(word, case));
    }

    if number {
        let index = dice::random_index(words.len())?;
        let digit = dice::random_digit()?;

        words[index].push(char::from(b'0' + digit));
    }

    Ok(words.join(separator))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let passphrase = generate_passphrase(args.words, &args.separator, args.case, args.number)?;

    if args.words < 6 && !args.quiet {
        eprintln!("Warning: The EFF recommends using at least 6 words for better security.");
    }

    match args.output {
        Output::Stdout => {
            if !args.quiet {
                println!("{passphrase}");
            }
        }

        Output::Clipboard => {
            let _guard = clipboard::ClipboardGuard::new(passphrase.clone())?;

            if !args.quiet {
                println!(
                    "Passphrase generated successfully!\nEstimated entropy: {:.1} bits",
                    entropy(args.words as usize)
                );

                println!("\nPassphrase copied.");
                println!("Press Enter to clear clipboard and exit.");
            }

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
        }

        Output::Qr => {
            print_qr(&passphrase)?;
            println!("Remember to clear screen after scan.");
        }
    }

    Ok(())
}
