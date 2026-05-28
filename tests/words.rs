#[cfg(test)]
mod tests {
    use hangman_rs::words::{WordBank, LoadWordBank};
    use std::io::Write;
    use tempfile::NamedTempFile;

    /// Helper: write a small word file to a temp file and load it.
    fn load_temp(content: &str) -> WordBank {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "{}", content).unwrap();
        LoadWordBank(f.path().to_str().unwrap()).unwrap()
    }

    #[test]
    fn test_loads_categories() {
        let bank = load_temp("[animals]\ncat\ndog\n[tech]\nkernel\n");
        assert_eq!(bank.category_count(), 2);
    }

    #[test]
    fn test_loads_words() {
        let bank = load_temp("[animals]\ncat\ndog\n");
        assert_eq!(bank.total_words(), 2);
    }

    #[test]
    fn test_ignores_comments() {
        let bank = load_temp("[animals]\n# this is a comment\ncat\n");
        assert_eq!(bank.total_words(), 1);
    }

    #[test]
    fn test_words_stored_lowercase() {
        let bank = load_temp("[test]\nCAT\n");
        let w = bank.PickWord(1);
        assert_eq!(w.text, w.text.to_lowercase(), "Words must be stored lowercase");
    }

    #[test]
    fn test_missing_file_returns_err() {
        let result = LoadWordBank("/nonexistent/path/words.txt");
        assert!(result.is_err(), "Missing file should return Err");
    }

    #[test]
    fn test_pick_word_level1_short() {
        // "cat" (3) and "elephant" (8) — level 1 should prefer "cat"
        let bank = load_temp("[animals]\ncat\nelephant\n");
        let w = bank.PickWord(1);
        assert!(w.text.len() <= 5, "Level 1 should pick short words (len ≤ 5)");
    }

    #[test]
    fn test_pick_word_returns_valid_category() {
        let bank = load_temp("[animals]\ncat\n[tech]\nkernel\n");
        let w = bank.PickWord(1);
        assert!(
            w.category == "animals" || w.category == "tech",
            "Category should match one of the loaded categories"
        );
    }
}