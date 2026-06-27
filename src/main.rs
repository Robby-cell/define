mod api;
mod audio;
mod display;
mod models;

use std::time::Duration;

use clap::{Parser, ValueEnum};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    /// Full dictionary view (definitions only).
    Dict,
    /// Synonyms & antonyms only.
    Thesaurus,
    /// Both (the default).
    Both,
}

#[derive(Parser)]
#[command(
    name = "define",
    version,
    about = "A blazing-fast, terminal-based Dictionary and Thesaurus written in Rust",
    long_about = "Reaches out to the Free Dictionary API to fetch definitions, \
                  synonyms, antonyms, and phonetics. Can stream and play pronunciation \
                  audio directly in memory."
)]
struct Cli {
    /// The word you want to look up.
    word: String,

    /// Plays the audio pronunciation of the word.
    #[arg(short, long)]
    play: bool,

    /// Output mode: dict, thesaurus, or both.
    #[arg(short, long, value_enum, default_value_t = Mode::Both)]
    mode: Mode,

    /// Language code (e.g. en, es, fr, de, ja).
    #[arg(short, long, default_value = "en")]
    lang: String,
}

fn spinner(msg: impl Into<String>) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner} {msg}")
            .unwrap()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
    );
    pb.set_message(msg.into());
    pb.enable_steady_tick(Duration::from_millis(80));
    pb
}

fn main() {
    let cli = Cli::parse();

    // ── Fetch ──────────────────────────────────────────────────────────────
    let pb = spinner(format!("Looking up '{}'…", cli.word));
    let entries = match api::fetch_word(&cli.word, &cli.lang) {
        Ok(e) => e,
        Err(e) => {
            pb.finish_with_message(format!("{} {}", "✗".red(), e.red()));
            std::process::exit(1);
        }
    };
    pb.finish_with_message(format!(
        "{} Found {} entr{} for '{}'",
        "✓".green(),
        entries.len(),
        if entries.len() == 1 { "y" } else { "ies" },
        entries[0].word
    ));

    // ── Display ────────────────────────────────────────────────────────────
    match cli.mode {
        Mode::Dict => display::display_dictionary(&entries),
        Mode::Thesaurus => display::display_thesaurus(&entries),
        Mode::Both => display::display_both(&entries),
    }

    // ── Optional audio playback ────────────────────────────────────────────
    if cli.play {
        let pb = spinner("Fetching pronunciation audio…".to_string());
        match display::play_audio(&entries) {
            Ok(_) => pb.finish_with_message(format!("{} Audio played.", "✓".green())),
            Err(e) => {
                pb.finish_with_message(format!("{} {}", "✗".red(), e.red()));
            }
        }
    }
}
