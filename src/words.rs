// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
//  words.rs — File I/O: Word Bank Loader
// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
//  OS Concepts covered:
//    - File I/O  → open, read, parse a structured text file
//    - Error handling with Result<T, E>  (like errno in C)
// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;
use rand::Rng;

#[derive(Debug, Clone)]
pub struct Word {
    pub text:     String,
    pub category: String,
}

#[derive(Debug)]
pub struct WordBank {
    // key: category name  →  value: list of words in that category
    categories: HashMap<String, Vec<String>>,
}

impl WordBank {
    /// Returns the total number of words across all categories.
    pub fn total_words(&self) -> usize {
        self.categories.values().map(|v| v.len()).sum()
    }

    /// Returns the number of distinct categories.
    pub fn category_count(&self) -> usize {
        self.categories.len()
    }
    //::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    // ----------------------------------------------------------
    // TODO 1-B: Implement `PickWord`
    //::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    //   Select a random word from the bank, biased by level.
    //   Hint: collect matching words across all categories, then
    //         use rand::thread_rng().gen_range(0..candidates.len())
    // ----------------------------------------------------------
    pub fn PickWord(&self, level: usize) -> Word {
        let mut candidates: Vec<Word> = Vec::new();
        todo!("TODO 1-B: pick a random word filtered by level/length")
    }
}

//::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
// ----------------------------------------------------------
// TODO 1-A: Implement `LoadWordBank`
//::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
//   Open the file at `path` and parse it into a WordBank.
// ----------------------------------------------------------
pub fn LoadWordBank(path: &str) -> Result<WordBank, io::Error> {
    todo!("TODO 1-A: open and parse the word file")
}

fn GroupCategories<R: io::BufRead>(reader: io::Lines<R>)-> Result<HashMap<String, Vec<String>>, io::Error> {
    let mut categories: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_category = String::new();
    for line in reader {
        let line = line?;
        if line.starts_with('[') && line.ends_with(']') {
            current_category = line[1..line.chars().count() - 1].to_string();
            categories.insert(current_category.clone(), Vec::new());
        } else if !line.is_empty() && !line.starts_with('#') {
            categories.get_mut(&current_category).unwrap().push(line.to_lowercase());
        }
    }
    return Ok(categories);
}
