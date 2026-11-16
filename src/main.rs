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

const MIN_WORDS: u32 = 6;
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
        value_parser = parse_word_count
    )]
    words: u32,

    /// Character(s) to insert between each word
    #[arg(short, long, value_enum, default_value_t = Separator::Hyphen)]
    separator: Separator,

    /// Passphrase word case
    #[arg(short, long, value_enum, default_value_t = Case::Sentence)]
    case: Case,

    /// Number placement
    #[arg(
        long,
        value_enum,
        default_value_t = NumberPlacement::Trailing
    )]
    number: NumberPlacement,

    /// Do not copy the passphrase to the clipboard
    #[arg(long, default_value_t = false)]
    no_clipboard: bool,

    /// Do not display a QR code
    #[arg(long, default_value_t = false)]
    no_qr: bool,

    /// Suppress informational messages
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn parse_word_count(value: &str) -> Result<u32, String> {
    let words: u32 = value
        .parse()
        .map_err(|_| format!("'{value}' is not a valid number"))?;

    if !(MIN_WORDS..=MAX_WORDS).contains(&words) {
        return Err(format!(
            "Passphrases must contain between {MIN_WORDS} and {MAX_WORDS} words."
        ));
    }

    Ok(words)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum Separator {
    /// Hyphen (-)
    Hyphen,
    /// Underscore (_)
    Underscore,
    /// Period (.)
    Period,
    /// Space
    Space,
    /// No separator
    None,
}

impl Separator {
    fn as_str(self) -> &'static str {
        match self {
            Self::Hyphen => "-",
            Self::Underscore => "_",
            Self::Period => ".",
            Self::Space => " ",
            Self::None => "",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
enum NumberPlacement {
    None,
    /// Append a number to the phrase
    Trailing,
    /// Inject the number at the end of a random word
    Random,
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
    number: NumberPlacement,
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

    match number {
        NumberPlacement::None => {}
        NumberPlacement::Trailing => {
            let digit = dice::random_digit()?;
            words.last_mut().unwrap().push(char::from(b'0' + digit));
        }
        NumberPlacement::Random => {
            let index = dice::random_index(words.len())?;
            let digit = dice::random_digit()?;
            words[index].push(char::from(b'0' + digit));
        }
    }

    Ok(words.join(separator))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let passphrase =
        generate_passphrase(args.words, &args.separator.as_str(), args.case, args.number)?;

    let _clipboard_guard = if !args.no_clipboard {
        Some(clipboard::ClipboardGuard::new(passphrase.clone())?)
    } else {
        None
    };

    if !args.no_qr {
        print_qr(&passphrase)?;
    }

    if !args.quiet {
        println!(
            "Estimated entropy: {:.1} bits",
            entropy(args.words as usize)
        );

        if !args.no_clipboard {
            eprintln!("Passphrase copied to clipboard.");
        }

        if !args.no_qr {
            eprintln!("Scan the QR code to retrieve your passphrase.");
        }

        if !args.no_clipboard {
            eprintln!("Press Enter to clear clipboard and exit.");
        }
    }

    if !args.no_clipboard {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
    }

    Ok(())
}
