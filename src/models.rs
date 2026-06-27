use serde::Deserialize;

/// Top-level entry returned by the Free Dictionary API.
#[derive(Debug, Deserialize)]
pub(crate) struct WordEntry {
    pub(crate) word: String,
    #[serde(default)]
    pub(crate) phonetics: Vec<Phonetic>,
    #[serde(default)]
    pub(crate) meanings: Vec<Meaning>,
    #[allow(dead_code)]
    pub(crate) origin: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Phonetic {
    pub(crate) text: Option<String>,
    pub(crate) audio: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub(crate) part_of_speech: String,
    #[serde(default)]
    pub(crate) definitions: Vec<Definition>,
    #[serde(default)]
    pub(crate) synonyms: Vec<String>,
    #[serde(default)]
    pub(crate) antonyms: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct Definition {
    pub(crate) definition: String,
    pub(crate) example: Option<String>,
    #[serde(default)]
    pub(crate) synonyms: Vec<String>,
    #[serde(default)]
    pub(crate) antonyms: Vec<String>,
}
