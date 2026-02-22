use buddytype::engine::input_processor::*;
use buddytype::engine::stats_calculator::*;
use buddytype::engine::timer::*;
use buddytype::engine::word_generator::generate_words;
use buddytype::types::GeneratorConfig;
use std::time::{Duration, Instant};

#[test]
fn simulates_complete_word_mode_test() {
    // 1. Generate words
    let words = generate_words(&GeneratorConfig {
        language: "english".into(),
        count: 3,
        punctuation: false,
        numbers: false,
    });
    assert_eq!(words.len(), 3);

    // 2. Create input state
    let mut state = create_input_state(&words);
    assert!(!state.finished);

    // 3. Type each word perfectly + space
    for w in &words {
        for ch in w.chars() {
            state = process_char(state, ch);
        }
        state = process_space(state);
    }

    // 4. Should be finished
    assert!(state.finished);

    // 5. Calculate stats
    let stats = calculate_stats(&state, 10.0);
    assert!(stats.wpm > 0.0);
    assert_eq!(stats.accuracy, 100.0);
    assert_eq!(stats.incorrect_chars, 0);
    assert_eq!(stats.extra_chars, 0);
    assert_eq!(stats.missed_chars, 0);
}

#[test]
fn simulates_test_with_errors_and_corrections() {
    let words: Vec<String> = vec!["hello".into(), "world".into()];
    let mut state = create_input_state(&words);

    // Type "hello" with a mistake, then fix it
    state = process_char(state, 'h');
    state = process_char(state, 'x'); // typo
    state = process_backspace(state);
    state = process_char(state, 'e');
    state = process_char(state, 'l');
    state = process_char(state, 'l');
    state = process_char(state, 'o');
    state = process_space(state);

    // Type "world" correctly
    for ch in "world".chars() {
        state = process_char(state, ch);
    }
    state = process_space(state);

    assert!(state.finished);

    let stats = calculate_stats(&state, 30.0);
    assert!(stats.wpm > 0.0);
    assert!(stats.accuracy < 100.0);
    assert!(stats.accuracy > 0.0);
}

#[test]
fn simulates_ctrl_backspace_during_typing() {
    let mut state = create_input_state(&["test".into(), "word".into()]);

    state = process_char(state, 't');
    state = process_char(state, 'x');
    state = process_char(state, 'y');
    state = process_ctrl_backspace(state);

    assert_eq!(state.words[0].typed, "");
    assert_eq!(state.cursor_position, 0);
    assert_eq!(state.current_word_index, 0);

    for ch in "test".chars() {
        state = process_char(state, ch);
    }
    state = process_space(state);
    assert_eq!(state.current_word_index, 1);
}

#[test]
fn calculates_consistency_from_wpm_history() {
    let steady = calculate_consistency(&[60.0, 62.0, 58.0, 61.0, 59.0, 60.0]);
    assert!(steady > 90.0);

    let erratic = calculate_consistency(&[20.0, 80.0, 15.0, 90.0, 25.0, 75.0]);
    assert!(erratic < 40.0);

    assert!(steady > erratic);
}

#[test]
fn timer_works_with_full_test_flow() {
    let timer = create_timer(Some(30));
    let now = Instant::now();
    let started = start_timer(timer, now);

    assert!(started.running);
    assert_eq!(get_elapsed_seconds(&started), 0.0);
    assert_eq!(get_remaining_seconds(&started), 30.0);

    // Simulate some elapsed time
    let later = now + Duration::from_secs(15);
    let midway = tick_timer(started, later);
    assert!((get_elapsed_seconds(&midway) - 15.0).abs() < 0.01);
    assert!((get_remaining_seconds(&midway) - 15.0).abs() < 0.01);
}

#[test]
fn handles_extra_characters_in_stats() {
    let mut state = create_input_state(&["hi".into()]);
    state = process_char(state, 'h');
    state = process_char(state, 'i');
    state = process_char(state, 'x'); // extra
    state = process_char(state, 'y'); // extra
    state = process_space(state);

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.extra_chars, 2);
    assert_eq!(stats.wpm, 0.0); // extra chars means word not correct
    assert!(stats.raw_wpm > 0.0);
}

#[test]
fn handles_missed_characters_in_stats() {
    let mut state = create_input_state(&["hello".into(), "ok".into()]);
    state = process_char(state, 'h');
    state = process_char(state, 'e');
    state = process_space(state);

    let stats = calculate_stats(&state, 10.0);
    assert_eq!(stats.missed_chars, 3); // l, l, o
}

#[test]
fn going_back_to_previous_word_preserves_state() {
    let mut state = create_input_state(&["hello".into(), "world".into()]);

    for ch in "hello".chars() {
        state = process_char(state, ch);
    }
    state = process_space(state);
    assert_eq!(state.current_word_index, 1);

    state = process_backspace(state);
    assert_eq!(state.current_word_index, 0);
    assert_eq!(state.words[0].typed, "hello");
    assert!(!state.words[0].completed);
    assert_eq!(state.cursor_position, 5);
}
