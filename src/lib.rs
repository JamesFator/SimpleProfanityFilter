use std::collections::HashSet;

static BLOCKLIST_CONTENTS: &str = include_str!("blocklist.txt");

pub struct ProfanityFilter {
    blocklist: HashSet<String>,
}

impl ProfanityFilter {
    pub fn new() -> ProfanityFilter {
        ProfanityFilter {
            blocklist: BLOCKLIST_CONTENTS.lines().map(String::from).collect(),
        }
    }

    pub fn filter(&self, sentance: &str) -> std::string::String {
        sentance
            .split_whitespace()
            .map(|w| self.convert_word(w))
            .collect::<Vec<_>>()
            .join(" ")
    }

    fn convert_word(&self, word: &str) -> String {
        match self.blocklist.contains(&word.to_lowercase()) {
            true => (0..word.len()).map(|_| "*").collect::<String>(),
            false => String::from(word),
        }
    }
}