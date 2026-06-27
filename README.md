
# 📖 define

A blazing-fast, terminal-based Dictionary and Thesaurus written in Rust. 

`define` reaches out to the [Free Dictionary API](https://dictionaryapi.dev/) to fetch definitions, synonyms, antonyms, and phonetics, formatting them beautifully in your terminal. It can even stream and play pronunciation audio directly in memory without saving temporary files.

## ✨ Features
* ⚡ **Lightning Fast:** Built in Rust for zero-startup latency.
* 🔊 **Audio Pronunciation:** Play MP3 pronunciations directly in the terminal (in-memory streaming).
* 📚 **Multiple Modes:** Choose between full dictionary view, a streamlined thesaurus, or both.
* 🌍 **Multi-Language Support:** Look up words in English, Spanish, French, German, Japanese, and more.
* 🎨 **Beautiful UI:** Clean, color-coded output with a responsive loading spinner.

---

## 🚀 Installation

### Prerequisites
You will need [Rust and Cargo](https://rustup.rs/) installed on your machine.
> **Linux Users:** You may need the ALSA development headers for the audio playback library to compile. (e.g., `sudo apt install libasound2-dev` on Ubuntu/Debian).

### Build & Install globally
Clone the repository and install it directly via Cargo:

```bash
git clone https://github.com/Robby-cell/define.git
cd define
cargo install --path .
```
This will compile the binary and place it in your Cargo `bin` directory, allowing you to use the `define` command from anywhere on your system!

---

## 💻 Usage

Just pass the word you want to look up:
```bash
define serendipity
```

### Options & Flags

| Flag / Option | Short | Description | Example |
| :--- | :---: | :--- | :--- |
| `--play` | `-p` | Plays the audio pronunciation of the word. | `define ubiquitous -p` |
| `--mode` | `-m` | Output mode: `dict`, `thesaurus`, or `both` (default). | `define beautiful --mode thesaurus` |
| `--lang` | `-l` | Language code (default is `en`). | `define hola --lang es` |
| `--help` | `-h` | Prints help information. | `define --help` |

### Examples

**1. Play audio while getting a definition:**
```bash
define ephemeral -p
```

**2. Use as a pure thesaurus (synonyms & antonyms only):**
```bash
define fast --mode thesaurus
```

**3. Look up a word in a foreign language:**
```bash
define chat --lang fr
```

---

## 🛠️ Built With
* [Rust](https://www.rust-lang.org/) - The programming language.
* [Clap](https://docs.rs/clap/latest/clap/) - For robust command-line argument parsing.
* [Reqwest](https://docs.rs/reqwest/latest/reqwest/) - For making HTTP GET requests to the API.
* [Rodio](https://docs.rs/rodio/latest/rodio/) - For decoding and playing the MP3 audio directly in-memory.
* [Colored](https://docs.rs/colored/latest/colored/) - For the beautiful terminal colors.
* [Serde](https://serde.rs/) - For deserializing the JSON API responses.

## 📄 License
This project is licensed under the MIT License. Feel free to fork, modify, and use it however you'd like!
