use hangman_rs::game::GameState;

fn setup(word: &str, secs: u64) -> GameState {
    let mut s = GameState::new();
    s.NewRound(word.to_string(), secs, "test".to_string(), 1, 1);
    s
}

// ----------------------------------------------------------
// TODO: Run tests with `cargo test`
// All tests below must pass with your implementation.
// ----------------------------------------------------------

#[test]
fn test_new_round_resets_lives() {
    let s = setup("rust", 30);
    assert_eq!(s.lives, 6, "Lives should reset to 6 on new round");
}

#[test]
fn test_new_round_stores_lowercase() {
    let s = setup("RUST", 30);
    assert_eq!(s.secret, "rust", "Secret should be stored in lowercase");
}

#[test]
fn test_correct_guess_no_life_loss() {
    let mut s = setup("rust", 30);
    s.Guess('r');
    assert_eq!(s.lives, 6, "Correct guess must not cost a life");
}

#[test]
fn test_wrong_guess_costs_life() {
    let mut s = setup("rust", 30);
    s.Guess('z');
    assert_eq!(s.lives, 5, "Wrong guess must decrement lives");
}

#[test]
fn test_duplicate_guess_no_penalty() {
    let mut s = setup("rust", 30);
    s.Guess('z');
    s.Guess('z');
    assert_eq!(s.lives, 5, "Duplicate guess must not cost an extra life");
}

#[test]
fn test_win_condition() {
    let mut s = setup("hi", 30);
    s.Guess('h');
    s.Guess('i');
    assert!(s.is_won(), "Should be won after guessing all letters");
    assert!(s.is_over(), "Round should be over after win");
}

#[test]
fn test_loss_condition() {
    let mut s = setup("hi", 30);
    for c in ['a', 'b', 'c', 'd', 'e', 'f'] {
        s.Guess(c);
    }
    assert!(!s.is_won(), "Should not be won after exhausting lives");
    assert!(s.is_over(), "Round should be over after 0 lives");
}

#[test]
fn test_masked_word_all_hidden() {
    let s = setup("rust", 30);
    assert_eq!(s.masked_word(), "_ _ _ _");
}

#[test]
fn test_masked_word_partial() {
    let mut s = setup("rust", 30);
    s.Guess('r');
    s.Guess('t');
    assert_eq!(s.masked_word(), "r _ _ t");
}

#[test]
fn test_masked_word_fully_revealed() {
    let mut s = setup("hi", 30);
    s.Guess('h');
    s.Guess('i');
    assert_eq!(s.masked_word(), "h i");
}

#[test]
fn test_round_score_zero_on_loss() {
    let mut s = setup("hi", 30);
    for c in ['a', 'b', 'c', 'd', 'e', 'f'] {
        s.Guess(c);
    }
    assert_eq!(s.round_score(), 0, "Score must be 0 on loss");
}

#[test]
fn test_round_score_positive_on_win() {
    let mut s = setup("hi", 30);
    s.Guess('h');
    s.Guess('i');
    assert!(s.round_score() > 0, "Score must be positive on win");
}

#[test]
fn test_total_score_accumulates() {
    let mut s = setup("hi", 30);
    s.Guess('h');
    s.Guess('i');
    let r1 = s.round_score();
    s.NewRound("ok".to_string(), 30, "test".to_string(), 1, 2);
    s.Guess('o');
    s.Guess('k');
    s.round_score();
    assert!(s.total_score > r1, "Total score should accumulate across rounds");
}
