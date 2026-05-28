// ============================================================
//  University of Puerto Rico, Mayagüez
// ============================================================
//  Authors (Team of 2):
//    1. Evelyn Vásquez
//    2. _______________________________
//
// ============================================================

#![allow(warnings)]
use hangman_rs::{game, words, timer, display};

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("=== HANGMAN — OS Edition ===\n");

    // ----------------------------------------------------------
    // TASK 1 — File I/O: Load word bank
    // (see words.rs)
    // ----------------------------------------------------------
    let word_bank = words::LoadWordBank("words/words.txt")
        .expect("[ERROR] Could not load word bank. Check the file path.");

    println!(
        "[INFO] Loaded {} words across {} categories.",
        word_bank.total_words(),
        word_bank.category_count()
    );

    // ----------------------------------------------------------
    // TASK 2 — Shared State: Initialize game state behind a Mutex
    // (see game.rs)
    // ----------------------------------------------------------
    let state = Arc::new(Mutex::new(game::GameState::new()));

    // ----------------------------------------------------------
    // TASK 3 — Signals: Register SIGALRM handler
    // (see timer.rs)
    // ----------------------------------------------------------
    timer::RegisterSignalHandler(Arc::clone(&state));

    // Run the game loop
    run_game(state, word_bank);
}

/// Orchestrates rounds and levels. Spawns the display thread each round.
fn run_game(state: Arc<Mutex<game::GameState>>, word_bank: words::WordBank) {
    const ROUNDS_PER_LEVEL: usize = 3;
    const MAX_LEVELS: usize = 3;

    for level in 1..=MAX_LEVELS {
        println!("\n--- Level {} ---", level);

        // ----------------------------------------------------------
        // TASK 4 — Timers: Set per-level timeout (seconds)
        //   Level 1 → 300s | Level 2 → 180s | Level 3 → 120s
        // (see timer.rs)
        // ----------------------------------------------------------
        let timeout_secs = timer::TimeOutForLevel(level);
        println!("[INFO] Time per word: {}s", timeout_secs);

        for round in 1..=ROUNDS_PER_LEVEL {
            let word = word_bank.PickWord(level);
            println!("\n  Round {}/{} — Category: {}", round, ROUNDS_PER_LEVEL, word.category);

            {
                let mut s = state.lock().unwrap();
                s.NewRound(word.text.clone(), timeout_secs, word.category.clone(), level, round);
            }

            let state_for_display = Arc::clone(&state);
            let display_handle = thread::spawn(move || {
                display::run_display_thread(state_for_display);
            });

            timer::Start(timeout_secs);
            game::run_input_loop(Arc::clone(&state));
            timer::Stop();
            {
                let mut s = state.lock().unwrap();
                s.round_score();
            }
            // Wait for display thread to finish
            display_handle.join().unwrap();
        }
    }

    let s = state.lock().unwrap();
    println!("\n=== GAME OVER — Final Score: {} ===", s.total_score());
}
