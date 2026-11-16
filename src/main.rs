use clap::{Parser, ValueEnum};
use std::error::Error;

pub mod clipboard;
pub mod dice;
pub mod qr;
pub mod words;

use dice::DiceRolls;

use crate::{qr::print_qr, words::entropy};

const HELP: &str = concat!(
    "Wordlist: EFF Large Diceware (7,776 words)\n",
    "Entropy:  12.92 bits per word; 77.55 bits by default\n",
    "Randomness: OS-provided cryptographically secure RNG\n\n",
    "Passphrases are copied to the clipboard by default.\n",
    "Use at least six words and never reuse passphrases.\n\n",
    "https://www.eff.org/dice\n",
    env!("CARGO_PKG_REPOSITORY"),
);

const MIN_WORDS: u32 = 1;
const MAX_WORDS: u32 = 100;

/// Generate a random Diceware passphrase
///
/// https://www.eff.org/dice
#[derive(Parser, Debug)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    author,
    about = env!("CARGO_PKG_DESCRIPTION"),
    after_help = HELP,
)]
struct Args {
    /// Number of words in the passphrase (EFF recommends at least 6)
    #[arg(
        default_value_t = 6,
        value_parser = clap::value_parser!(u32).range(MIN_WORDS as i64..=MAX_WORDS as i64)
    )]
    words: u32,

    /// Character(s) to insert between each word
    #[arg(short, long, default_value = "-")]
    separator: String,

    /// Passphrase word case
    #[arg(short, long, value_enum, default_value_t = Case::Sentence)]
    case: Case,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = Output::Clipboard)]
    output: Output,

    /// Add a number
    #[arg(short, long, default_value_t = true)]
    number: bool,

    /// Suppress informational messages
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Case {
    /// Lowercase all characters
    Lower,
    /// Uppercase all characters
    Upper,
    /// Uppercase the first character of each word
    Title,
    /// Randomly shift case for each word
    Random,
    /// Uppercase the first letter of the phrase
    Sentence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Output {
    Clipboard,
    Qr,
    Stdout,
}

fn format_word(word: &str, case: Case) -> Result<String, Box<dyn Error>> {
    match case {
        Case::Lower | Case::Sentence => Ok(word.to_lowercase()),
        Case::Upper => Ok(word.to_uppercase()),
        Case::Title => {
            let mut chars = word.chars();

            Ok(match chars.next() {
                Some(first) => {
                    first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                }
                None => String::new(),
            })
        }
        Case::Random => {
            if dice::random_bool()? {
                Ok(word.to_uppercase())
            } else {
                Ok(word.to_lowercase())
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
        words.push(format_word(word, case)?);
    }

    if case == Case::Sentence {
        if let Some(first_word) = words.first_mut() {
            if let Some(first_char) = first_word.get_mut(0..1) {
                first_char.make_ascii_uppercase();
            }
        }
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
            println!("{passphrase}");
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

            if !args.quiet {
                println!("Remember to clear screen after scan.");
            }
        }
    }

    Ok(())
}
