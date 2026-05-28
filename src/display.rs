// :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
//  display.rs — Display Thread  [PROVIDED — DO NOT MODIFY]
// :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
use pancurses::{initscr, endwin, noecho, curs_set};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::game::GameState;

// ASCII art stages: index = lives remaining (0 = dead, 6 = full)
const STAGES: [&str; 7] = [
    // 0 lives
    " ___\n |   |\n |   O\n |  /|\\\n |  / \\\n_|_",
    // 1 life
    " ___\n |   |\n |   O\n |  /|\\\n |  /\n_|_",
    // 2 lives
    " ___\n |   |\n |   O\n |  /|\\\n |\n_|_",
    // 3 lives
    " ___\n |   |\n |   O\n |  /|\n |\n_|_",
    // 4 lives
    " ___\n |   |\n |   O\n |   |\n |\n_|_",
    // 5 lives
    " ___\n |   |\n |   O\n |\n |\n_|_",
    // 6 lives (no body parts)
    " ___\n |   |\n |\n |\n |\n_|_",
];

/// Redraws the terminal every second until the round ends.
/// Receives shared game state through an Arc<Mutex<GameState>>.
pub fn run_display_thread(state: Arc<Mutex<GameState>>) {
    let window = initscr();
    pancurses::nocbreak();
    pancurses::echo();
    curs_set(0);

    loop {
        thread::sleep(Duration::from_millis(100));

        // Lock as briefly as possible — clone what we need, then drop
        let snapshot = {
            let s = state.lock().unwrap();
            (
                s.lives,
                s.masked_word(),
                s.secs_remaining,
                s.is_over(),
                s.guessed.clone(),
                s.level,
                s.category.clone(),
                s.is_won(),
                s.secret_word().to_string(),
                s.level_scores,
                s.total_score,
                s.round,
            )
        }; // lock released here

        let (lives, masked, secs, over, guessed, level, category, is_won, secret_word, level_scores, total_score, round) = snapshot;
        window.clear();
        window.mvaddstr(0, 0, &format!(
            "Level 1: {} | Level 2: {} | Level 3: {} --- Final Score: {}",
            level_scores[0], level_scores[1], level_scores[2], total_score
        ));
        window.mvaddstr(1, 0, &format!(
            "---------------------------Level: {}---------------------------", level
        ));
        window.mvaddstr(2, 0, &format!(
            "Round: {}/3 -- Category word: {}", round, category
        ));

        // Draw hangman stage
        let stage_index = (lives as usize).min(6);
        let mut row = 4;
        for line in STAGES[stage_index].lines() {
            window.mvaddstr(row, 0, line);
            row += 1;
        }
        row += 1;

        window.mvaddstr(row, 0, &format!("Word: {}", masked));
        row += 1;

        // Draw guessed letters
        let guessed_str: String = guessed.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        window.mvaddstr(row, 0, &format!("Guessed: [{}]", guessed_str));
        row += 1;

        // Draw timer
        window.mvaddstr(row, 0, &format!("Time left: {}s   Lives: {}", secs, lives));

        window.refresh();
        if over {
            row += 2;
            if is_won {
                window.mvaddstr(row, 0, ":::::::::::::Correct!::::::::::::::");
            } else {
                window.mvaddstr(row, 0, &format!("  Time's up. The word was: {}", secret_word));
            }
            window.refresh();
            thread::sleep(Duration::from_secs(2));  // dale tiempo al usuario para verlo
            break;
        }
    }
    endwin();
}
