use crate::types::{InputState, StatsSnapshot};

pub fn calculate_stats(state: &InputState, elapsed_seconds: f64) -> StatsSnapshot {
    if elapsed_seconds <= 0.0 {
        return StatsSnapshot::default();
    }

    let mut correct_word_chars: usize = 0;
    let mut correct_spaces: usize = 0;
    let mut all_correct_chars: usize = 0;
    let mut incorrect_chars: usize = 0;
    let mut extra_chars: usize = 0;
    let mut missed_chars: usize = 0;
    let mut spaces: usize = 0;

    for i in 0..=state.current_word_index.min(state.words.len().saturating_sub(1)) {
        let word_state = &state.words[i];

        for cr in &word_state.chars {
            if cr.extra {
                extra_chars += 1;
            } else if cr.correct {
                all_correct_chars += 1;
            } else {
                incorrect_chars += 1;
            }
        }

        // Count missed characters (word was completed but not fully typed)
        if word_state.completed && word_state.typed.len() < word_state.word.len() {
            missed_chars += word_state.word.len() - word_state.typed.len();
        }

        if word_state.completed {
            spaces += 1;

            // WPM: only count chars from entirely correct words (MonkeyType formula)
            if word_state.typed == word_state.word {
                correct_word_chars += word_state.word.len();
                correct_spaces += 1;
            }
        }
    }

    let minutes = elapsed_seconds / 60.0;

    // Net WPM: only correctly-typed whole words + their spaces
    let wpm = round_to_2((correct_word_chars + correct_spaces) as f64 / 5.0 / minutes).max(0.0);

    // Raw WPM: all typed characters (correct + incorrect + extra + spaces)
    let total_typed = all_correct_chars + incorrect_chars + extra_chars + spaces;
    let raw_wpm = round_to_2(total_typed as f64 / 5.0 / minutes).max(0.0);

    // Accuracy: per-keypress correct / (correct + incorrect) (MonkeyType formula)
    let total_keypresses = state.keypress_correct + state.keypress_incorrect;
    let accuracy = if total_keypresses > 0 {
        round_to_2(state.keypress_correct as f64 / total_keypresses as f64 * 100.0)
    } else {
        100.0
    };

    StatsSnapshot {
        wpm,
        raw_wpm,
        accuracy,
        correct_chars: all_correct_chars,
        incorrect_chars,
        extra_chars,
        missed_chars,
        elapsed_seconds,
    }
}

/// MonkeyType's "kogasa" consistency function.
/// Maps the coefficient of variation (COV) from [0, +inf) to [100, 0).
fn kogasa(cov: f64) -> f64 {
    100.0 * (1.0 - (cov + cov.powi(3) / 3.0 + cov.powi(5) / 5.0).tanh())
}

pub fn calculate_consistency(wpm_history: &[f64]) -> f64 {
    if wpm_history.len() < 2 {
        return 100.0;
    }

    let sum: f64 = wpm_history.iter().sum();
    let mean = sum / wpm_history.len() as f64;
    if mean == 0.0 {
        return 0.0;
    }

    let variance =
        wpm_history.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / wpm_history.len() as f64;
    let stddev = variance.sqrt();
    let cov = stddev / mean;

    round_to_2(kogasa(cov))
}

fn round_to_2(n: f64) -> f64 {
    (n * 100.0).round() / 100.0
}
