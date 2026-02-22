use buddytype::engine::input_processor::*;
use buddytype::engine::stats_calculator::*;

// ── calculateStats ──────────────────────────────────────────────

#[test]
fn returns_zeros_for_zero_elapsed_time() {
    let state = create_input_state(&["hello".into()]);
    let stats = calculate_stats(&state, 0.0);
    assert_eq!(stats.wpm, 0.0);
    assert_eq!(stats.raw_wpm, 0.0);
    assert_eq!(stats.accuracy, 0.0);
}

#[test]
fn calculates_wpm_correctly_for_perfect_typing() {
    let state = create_input_state(&["hello".into(), "world".into()]);
    // Type "hello" correctly
    let mut state = state;
    for c in "hello".chars() {
        state = process_char(state, c);
    }
    state = process_space(state);
    // Type "world" correctly
    for c in "world".chars() {
        state = process_char(state, c);
    }

    // "hello" (5 chars) + 1 space = 6 correct word chars
    // In 12 seconds: (6 / 5) / (12 / 60) = 1.2 / 0.2 = 6 WPM
    let stats = calculate_stats(&state, 12.0);
    assert!((stats.wpm - 6.0).abs() < 1.0);
    assert!((stats.raw_wpm - 11.0).abs() < 1.0);
    assert_eq!(stats.accuracy, 100.0);
}

#[test]
fn calculates_wpm_only_for_entirely_correct_words() {
    let state = create_input_state(&["hello".into(), "world".into(), "test".into()]);
    let mut state = state;
    for c in "hello".chars() {
        state = process_char(state, c);
    }
    state = process_space(state);
    // "world" with a typo
    for c in "worxd".chars() {
        state = process_char(state, c);
    }
    state = process_space(state);
    for c in "test".chars() {
        state = process_char(state, c);
    }
    state = process_space(state);

    // WPM: "hello"(5)+space(1) + "test"(4)+space(1) = 11 correct word chars
    // In 60 seconds: (11/5)/1 = 2.2 WPM
    let stats = calculate_stats(&state, 60.0);
    assert!((stats.wpm - 2.2).abs() < 0.1);
}

#[test]
fn tracks_incorrect_characters() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'x'); // incorrect
    let state = process_char(state, 'i'); // correct

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.correct_chars, 1);
    assert_eq!(stats.incorrect_chars, 1);
    assert_eq!(stats.accuracy, 50.0);
}

#[test]
fn tracks_extra_characters() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_char(state, 'x'); // extra

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.extra_chars, 1);
    assert_eq!(stats.correct_chars, 2);
}

#[test]
fn tracks_missed_characters() {
    let state = create_input_state(&["hello".into(), "world".into()]);
    let state = process_char(state, 'h');
    let state = process_space(state); // skip rest of "hello"

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.missed_chars, 4); // missed e, l, l, o
}

#[test]
fn keeps_accuracy_penalty_for_corrected_mistakes() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'x'); // incorrect
    let state = process_backspace(state);
    let state = process_char(state, 'h'); // correct
    let state = process_char(state, 'i'); // correct

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.correct_chars, 2);
    assert_eq!(stats.incorrect_chars, 0);
    // 2 correct / 3 total = 66.67%
    assert!((stats.accuracy - 66.67).abs() < 0.01);
}

#[test]
fn counts_space_as_incorrect_when_word_has_errors() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let state = process_char(state, 'x');
    let state = process_char(state, 'i');
    let state = process_space(state);

    // x(incorrect), i(correct), space(incorrect) = 1 correct, 2 incorrect
    let stats = calculate_stats(&state, 10.0);
    assert!((stats.accuracy - 33.33).abs() < 0.01);
}

#[test]
fn counts_space_as_correct_when_word_is_perfect() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_space(state);

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.accuracy, 100.0);
}

#[test]
fn raw_wpm_includes_all_typed_chars_including_spaces() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let mut state = state;
    for c in "hi".chars() {
        state = process_char(state, c);
    }
    state = process_space(state);
    for c in "ok".chars() {
        state = process_char(state, c);
    }

    // Raw: 4 chars + 1 space = 5 total
    // In 60 seconds: (5/5)/1 = 1 raw WPM
    let stats = calculate_stats(&state, 60.0);
    assert_eq!(stats.raw_wpm, 1.0);
}

#[test]
fn returns_safe_values_for_negative_time() {
    let state = create_input_state(&["hello".into()]);
    let stats = calculate_stats(&state, -5.0);
    assert_eq!(stats.wpm, 0.0);
    assert_eq!(stats.raw_wpm, 0.0);
}

#[test]
fn wpm_is_zero_when_no_words_completed() {
    let state = create_input_state(&["hello".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'e');
    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.wpm, 0.0);
    assert!(stats.raw_wpm > 0.0);
}

#[test]
fn wpm_is_zero_when_all_completed_words_have_errors() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let state = process_char(state, 'x');
    let state = process_char(state, 'y');
    let state = process_space(state);
    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.wpm, 0.0);
}

#[test]
fn handles_multiple_completed_words() {
    let state = create_input_state(&["a".into(), "b".into(), "c".into(), "d".into()]);
    let mut state = state;
    for w in ["a", "b", "c", "d"] {
        for c in w.chars() {
            state = process_char(state, c);
        }
        state = process_space(state);
    }
    let stats = calculate_stats(&state, 60.0);
    assert!(stats.wpm > 0.0);
    assert_eq!(stats.accuracy, 100.0);
}

#[test]
fn counts_multiple_missed_chars_across_words() {
    let state = create_input_state(&["hello".into(), "world".into(), "test".into()]);
    let mut state = state;
    // Type only 2 chars of "hello"
    state = process_char(state, 'h');
    state = process_char(state, 'e');
    state = process_space(state); // missed l, l, o = 3
    // Type only 1 char of "world"
    state = process_char(state, 'w');
    state = process_space(state); // missed o, r, l, d = 4

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.missed_chars, 7);
}

#[test]
fn accuracy_is_100_when_nothing_typed() {
    let state = create_input_state(&["hello".into()]);
    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.accuracy, 100.0);
}

#[test]
fn elapsed_seconds_is_reflected_in_stats() {
    let state = create_input_state(&["hello".into()]);
    let stats = calculate_stats(&state, 42.0);
    assert_eq!(stats.elapsed_seconds, 42.0);
}

// ── calculateConsistency ────────────────────────────────────────

#[test]
fn consistency_100_for_uniform_wpm() {
    let consistency = calculate_consistency(&[50.0, 50.0, 50.0, 50.0]);
    assert_eq!(consistency, 100.0);
}

#[test]
fn consistency_100_for_single_sample() {
    assert_eq!(calculate_consistency(&[50.0]), 100.0);
}

#[test]
fn consistency_lower_for_variable_wpm() {
    let consistency = calculate_consistency(&[20.0, 80.0, 30.0, 70.0]);
    assert!(consistency < 55.0);
    assert!(consistency > 40.0);
}

#[test]
fn consistency_0_for_zero_mean() {
    assert_eq!(calculate_consistency(&[0.0, 0.0, 0.0]), 0.0);
}

#[test]
fn consistency_bounded_0_to_100() {
    let result = calculate_consistency(&[1.0, 100.0, 1.0, 100.0]);
    assert!(result >= 0.0);
    assert!(result <= 100.0);
}

#[test]
fn highly_consistent_typing_above_90() {
    let consistency = calculate_consistency(&[58.0, 62.0, 60.0, 61.0, 59.0, 60.0]);
    assert!(consistency > 90.0);
}

#[test]
fn very_erratic_typing_below_30() {
    let consistency = calculate_consistency(&[10.0, 90.0, 5.0, 95.0, 15.0, 85.0]);
    assert!(consistency < 30.0);
}

#[test]
fn consistency_100_for_empty_array() {
    assert_eq!(calculate_consistency(&[]), 100.0);
}

#[test]
fn moderate_variation_between_50_and_90() {
    let consistency = calculate_consistency(&[40.0, 60.0, 45.0, 55.0, 50.0]);
    assert!(consistency > 50.0);
    assert!(consistency < 90.0);
}
