// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
//  game.rs — Shared Game State & Input Logic
// ::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::

use std::sync::{Arc, Mutex};
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct GameState {
    pub secret:         String,// The secret word for the current round (lowercase)    
    pub guessed:        Vec<char>,// Letters the player has already guessed    
    pub lives:          u8,// Lives remaining (start at 6, matching classic hangman stages)    
    pub secs_remaining: u64,// Seconds remaining in the current round (decremented by timer)    
    pub round_over:     bool,// Set to true by either the input loop or the signal handler    
    pub won:            bool,// True when the player guessed all letters before time/lives ran out    
    pub total_score:    u64,// Cumulative score across all rounds    
    pub last_round_score: u64,// Score earned in the most recent completed round
    pub category: String,
    pub level: usize,
    pub level_scores: [u64; 3],
    pub round: usize,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            secret:           String::new(),
            guessed:          Vec::new(),
            lives:            6,
            secs_remaining:   0,
            round_over:       false,
            won:              false,
            total_score:      0,
            last_round_score: 0,
            category:         String::new(),
            level:            1,
            level_scores:     [0; 3],
            round:            1,
        }
    }

    // ----------------------------------------------------------
    // TODO 2-A: Implement `NewRound`
    //   Reset all per-round fields for a new word in lower case.
    // ----------------------------------------------------------
    pub fn NewRound(&mut self, word: String, timeout_secs: u64, category: String, level: usize, round: usize) {
        self.secret = word.to_lowercase();
        todo!("TODO 2-A: reset per-round state");
    }

    // ----------------------------------------------------------
    // TODO 2-B: Implement `Guess`
    //   Record a letter guess and update state.
    //   Hint: use `AllLettersFound()` to check for win
    // ----------------------------------------------------------
    pub fn Guess(&mut self, letter: char) {
        // Hint: if self.guessed.contains(&letter) {
        //     return;
        // }
        
        todo!("TODO 2-B: handle a player guess");
    }

    // ----------------------------------------------------------
    // TODO 2-C: Implement `AllLettersFound`
    //   Returns true when every letter in `secret` appears
    //   in `guessed`.
    // ----------------------------------------------------------
    pub fn AllLettersFound(&self) -> bool {
        todo!("TODO 3-B: Returns true when every letter in `secret` appear");
    }


    pub fn masked_word(&self) -> String {
        let mut word_masked = String::new();
        for letter in self.secret.chars() {
            if self.guessed.contains(&letter) {
                word_masked.push(letter);
            } else {
                word_masked.push('_');
            }
            word_masked.push(' ');
        }
        return word_masked.trim_end().to_string()
    }


    pub fn round_score(&mut self) -> u64 { //Unsigned 64 bit integer
        if self.won {
            self.last_round_score = 100 + (self.secs_remaining * 5) + (self.lives as u64 * 10);
        } else {
            self.last_round_score = 0;
        }
        self.total_score += self.last_round_score;
        self.level_scores[self.level - 1] += self.last_round_score;
        return self.last_round_score;
    }


    pub fn is_won(&self)       -> bool   { self.won }
    pub fn is_over(&self)      -> bool   { self.round_over }
    pub fn secret_word(&self)  -> &str   { &self.secret }
    pub fn total_score(&self)  -> u64    { self.total_score }
}

pub fn run_input_loop(state: Arc<Mutex<GameState>>) {
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => break,
        };

        let letter = match line.trim().chars().next() {
            Some(c) if c.is_alphabetic() => c.to_ascii_lowercase(),
            _ => continue,
        };

        {
            let mut s = state.lock().unwrap();
            s.Guess(letter);
            if s.round_over {
                break;
            }
        }
    }
}


