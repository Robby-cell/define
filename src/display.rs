use colored::*;

use crate::models::WordEntry;

/// Locate the first available phonetic transcription text.
fn find_phonetic_text(entries: &[WordEntry]) -> Option<String> {
    entries
        .iter()
        .flat_map(|e| &e.phonetics)
        .find_map(|p| p.text.clone().filter(|t| !t.is_empty()))
}

/// Locate the first available audio URL.
pub fn find_audio_url(entries: &[WordEntry]) -> Option<String> {
    entries
        .iter()
        .flat_map(|e| &e.phonetics)
        .find_map(|p| p.audio.clone().filter(|a| !a.is_empty()))
}

fn header(entries: &[WordEntry]) {
    let word = &entries[0].word;
    let phon = find_phonetic_text(entries);
    println!(
        "\n{} {}",
        word.to_uppercase().bold().cyan(),
        phon.map(|t| t.green()).unwrap_or_default()
    );
    println!("{}", "─".repeat(40).dimmed());
}

/// Print only definitions (Dictionary mode).
pub fn display_dictionary(entries: &[WordEntry]) {
    if entries.is_empty() {
        return;
    }
    header(entries);

    for entry in entries {
        for meaning in &entry.meanings {
            println!("\n  {}", meaning.part_of_speech.italic().magenta());
            for (i, def) in meaning.definitions.iter().enumerate() {
                println!(
                    "    {} {}",
                    format!("{}.", i + 1).bold().yellow(),
                    def.definition
                );
                if let Some(ex) = &def.example {
                    println!("       {} {}", "ex:".dimmed(), ex.italic());
                }
                if !def.synonyms.is_empty() {
                    println!(
                        "       {} {}",
                        "syn:".dimmed(),
                        def.synonyms.join(", ").blue()
                    );
                }
                if !def.antonyms.is_empty() {
                    println!(
                        "       {} {}",
                        "ant:".dimmed(),
                        def.antonyms.join(", ").red()
                    );
                }
            }
        }
    }
    println!();
}

/// Deduplicate while preserving order.
fn dedupe(v: Vec<String>) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    v.into_iter()
        .filter(|s| seen.insert(s.to_lowercase()))
        .collect()
}

/// Print only synonyms & antonyms (Thesaurus mode).
pub fn display_thesaurus(entries: &[WordEntry]) {
    if entries.is_empty() {
        return;
    }
    header(entries);

    let mut syns: Vec<String> = Vec::new();
    let mut ants: Vec<String> = Vec::new();

    for entry in entries {
        for meaning in &entry.meanings {
            syns.extend(meaning.synonyms.iter().cloned());
            ants.extend(meaning.antonyms.iter().cloned());
            for def in &meaning.definitions {
                syns.extend(def.synonyms.iter().cloned());
                ants.extend(def.antonyms.iter().cloned());
            }
        }
    }

    syns = dedupe(syns);
    ants = dedupe(ants);

    println!();
    if syns.is_empty() {
        println!("  {} No synonyms found.", "Synonyms:".bold().blue());
    } else {
        println!("  {} {}", "Synonyms:".bold().blue(), syns.join(", "));
    }
    if ants.is_empty() {
        println!("  {} No antonyms found.", "Antonyms:".bold().red());
    } else {
        println!("  {} {}", "Antonyms:".bold().red(), ants.join(", "));
    }
    println!();
}

/// Print both dictionary + thesaurus-style summary (Both mode, the default).
pub fn display_both(entries: &[WordEntry]) {
    if entries.is_empty() {
        return;
    }
    header(entries);

    let mut syns: Vec<String> = Vec::new();
    let mut ants: Vec<String> = Vec::new();

    for entry in entries {
        for meaning in &entry.meanings {
            println!("\n  {}", meaning.part_of_speech.italic().magenta());

            for (i, def) in meaning.definitions.iter().enumerate() {
                println!(
                    "    {} {}",
                    format!("{}.", i + 1).bold().yellow(),
                    def.definition
                );
                if let Some(ex) = &def.example {
                    println!("       {} {}", "ex:".dimmed(), ex.italic());
                }
                syns.extend(def.synonyms.iter().cloned());
                ants.extend(def.antonyms.iter().cloned());
            }

            syns.extend(meaning.synonyms.iter().cloned());
            ants.extend(meaning.antonyms.iter().cloned());

            if !meaning.synonyms.is_empty() {
                println!(
                    "    {} {}",
                    "synonyms:".bold().blue(),
                    meaning.synonyms.join(", ").blue()
                );
            }
            if !meaning.antonyms.is_empty() {
                println!(
                    "    {} {}",
                    "antonyms:".bold().red(),
                    meaning.antonyms.join(", ").red()
                );
            }
        }
    }

    syns = dedupe(syns);
    ants = dedupe(ants);

    println!("\n{}", "─".repeat(40).dimmed());
    if !syns.is_empty() {
        println!("  {} {}", "All Synonyms:".bold().blue(), syns.join(", "));
    }
    if !ants.is_empty() {
        println!("  {} {}", "All Antonyms:".bold().red(), ants.join(", "));
    }
    println!();
}

/// Stream + play pronunciation audio in-memory.
pub fn play_audio(entries: &[WordEntry]) -> Result<(), String> {
    let url = find_audio_url(entries)
        .ok_or_else(|| "No audio pronunciation available for this word.".to_string())?;

    let bytes = crate::api::fetch_audio(&url)?;
    crate::audio::play_bytes(&bytes)
}
