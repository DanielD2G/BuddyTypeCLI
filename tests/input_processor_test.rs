use buddytype::engine::input_processor::*;

// ── createInputState ────────────────────────────────────────────

#[test]
fn initializes_with_correct_defaults() {
    let state = create_input_state(&["hello".into(), "world".into()]);
    assert_eq!(state.words.len(), 2);
    assert_eq!(state.current_word_index, 0);
    assert_eq!(state.cursor_position, 0);
    assert!(!state.finished);
    assert_eq!(state.words[0].word, "hello");
    assert_eq!(state.words[0].typed, "");
    assert!(!state.words[0].completed);
    assert_eq!(state.historical_error_chars, 0);
}

#[test]
fn initializes_keypress_counters_at_zero() {
    let state = create_input_state(&["hello".into()]);
    assert_eq!(state.keypress_correct, 0);
    assert_eq!(state.keypress_incorrect, 0);
}

#[test]
fn handles_single_character_words() {
    let state = create_input_state(&["a".into(), "b".into(), "c".into()]);
    assert_eq!(state.words.len(), 3);
    assert_eq!(state.words[0].word, "a");
}

// ── processChar ─────────────────────────────────────────────────

#[test]
fn adds_a_correct_character() {
    let state = create_input_state(&["hello".into()]);
    let next = process_char(state, 'h');
    assert_eq!(next.words[0].typed, "h");
    assert_eq!(next.words[0].chars.len(), 1);
    assert!(next.words[0].chars[0].correct);
    assert!(!next.words[0].chars[0].extra);
    assert_eq!(next.cursor_position, 1);
}

#[test]
fn adds_an_incorrect_character() {
    let state = create_input_state(&["hello".into()]);
    let next = process_char(state, 'x');
    assert_eq!(next.words[0].typed, "x");
    assert!(!next.words[0].chars[0].correct);
    assert!(!next.words[0].chars[0].extra);
    assert_eq!(next.historical_error_chars, 1);
}

#[test]
fn marks_extra_characters_beyond_word_length() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_char(state, 'x'); // extra
    assert!(state.words[0].chars[2].extra);
    assert!(!state.words[0].chars[2].correct);
    assert_eq!(state.cursor_position, 3);
    assert_eq!(state.historical_error_chars, 1);
}

#[test]
fn does_nothing_when_finished() {
    let mut state = create_input_state(&["a".into()]);
    state.finished = true;
    let next = process_char(state, 'a');
    assert_eq!(next.words[0].typed, "");
}

// ── processSpace ────────────────────────────────────────────────

#[test]
fn advances_to_next_word() {
    let state = create_input_state(&["hello".into(), "world".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'e');
    let state = process_char(state, 'l');
    let state = process_char(state, 'l');
    let state = process_char(state, 'o');
    let state = process_space(state);
    assert!(state.words[0].completed);
    assert_eq!(state.current_word_index, 1);
    assert_eq!(state.cursor_position, 0);
}

#[test]
fn does_not_advance_if_nothing_typed() {
    let state = create_input_state(&["hello".into(), "world".into()]);
    let next = process_space(state);
    assert_eq!(next.current_word_index, 0);
}

#[test]
fn sets_finished_on_last_word() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_space(state);
    assert!(state.finished);
    assert!(state.words[0].completed);
}

#[test]
fn allows_advancing_with_incomplete_word() {
    let state = create_input_state(&["hello".into(), "world".into()]);
    let state = process_char(state, 'h');
    let state = process_space(state);
    assert!(state.words[0].completed);
    assert_eq!(state.current_word_index, 1);
}

#[test]
fn space_does_nothing_when_finished() {
    let state = create_input_state(&["a".into()]);
    let state = process_char(state, 'a');
    let state = process_space(state);
    assert!(state.finished);
    let next = process_space(state);
    assert!(next.finished);
}

#[test]
fn handles_word_with_only_wrong_characters() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let state = process_char(state, 'x');
    let state = process_char(state, 'y');
    let state = process_space(state);
    assert!(state.words[0].completed);
    assert_eq!(state.current_word_index, 1);
}

// ── processBackspace ────────────────────────────────────────────

#[test]
fn removes_last_character() {
    let state = create_input_state(&["hello".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'e');
    let state = process_backspace(state);
    assert_eq!(state.words[0].typed, "h");
    assert_eq!(state.words[0].chars.len(), 1);
    assert_eq!(state.cursor_position, 1);
}

#[test]
fn goes_back_to_previous_completed_word() {
    let state = create_input_state(&["hi".into(), "there".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_space(state);
    assert_eq!(state.current_word_index, 1);

    let state = process_backspace(state);
    assert_eq!(state.current_word_index, 0);
    assert!(!state.words[0].completed);
    assert_eq!(state.cursor_position, 2);
}

#[test]
fn does_nothing_at_start_of_first_word() {
    let state = create_input_state(&["hello".into()]);
    let next = process_backspace(state);
    assert_eq!(next.words[0].typed, "");
    assert_eq!(next.cursor_position, 0);
}

#[test]
fn keeps_historical_errors_after_deleting_incorrect_char() {
    let state = create_input_state(&["hello".into()]);
    let state = process_char(state, 'x'); // incorrect
    assert_eq!(state.historical_error_chars, 1);

    let state = process_backspace(state);
    assert_eq!(state.words[0].typed, "");
    assert_eq!(state.words[0].chars.len(), 0);
    assert_eq!(state.historical_error_chars, 1);
}

#[test]
fn backspace_does_nothing_when_finished() {
    let state = create_input_state(&["a".into()]);
    let state = process_char(state, 'a');
    let state = process_space(state);
    assert!(state.finished);
    let next = process_backspace(state);
    assert!(next.finished);
}

#[test]
fn handles_multiple_backspaces_in_a_row() {
    let state = create_input_state(&["hello".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'e');
    let state = process_char(state, 'l');
    let state = process_backspace(state);
    let state = process_backspace(state);
    let state = process_backspace(state);
    assert_eq!(state.words[0].typed, "");
    assert_eq!(state.cursor_position, 0);
    // One more at start should do nothing
    let state = process_backspace(state);
    assert_eq!(state.cursor_position, 0);
}

#[test]
fn removes_extra_characters_with_backspace() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_char(state, 'x'); // extra
    let state = process_backspace(state);
    assert_eq!(state.words[0].typed, "hi");
    assert_eq!(state.words[0].chars.len(), 2);
}

// ── processCtrlBackspace ────────────────────────────────────────

#[test]
fn ctrl_backspace_clears_entire_word() {
    let state = create_input_state(&["hello".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'e');
    let state = process_char(state, 'l');
    let state = process_ctrl_backspace(state);
    assert_eq!(state.words[0].typed, "");
    assert_eq!(state.words[0].chars.len(), 0);
    assert_eq!(state.cursor_position, 0);
}

#[test]
fn ctrl_backspace_does_nothing_on_empty_word() {
    let state = create_input_state(&["hello".into()]);
    let next = process_ctrl_backspace(state);
    assert_eq!(next.words[0].typed, "");
    assert_eq!(next.cursor_position, 0);
}

#[test]
fn ctrl_backspace_does_nothing_when_finished() {
    let state = create_input_state(&["a".into()]);
    let state = process_char(state, 'a');
    let state = process_space(state);
    assert!(state.finished);
    let next = process_ctrl_backspace(state);
    assert!(next.finished);
}

#[test]
fn ctrl_backspace_clears_extra_characters() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_char(state, 'x'); // extra
    let state = process_char(state, 'y'); // extra
    let state = process_ctrl_backspace(state);
    assert_eq!(state.words[0].typed, "");
    assert_eq!(state.words[0].chars.len(), 0);
}

// ── processKeystroke ────────────────────────────────────────────

#[test]
fn delegates_backspace_correctly() {
    let state = create_input_state(&["hi".into()]);
    let state = process_keystroke(state, "h", false, false);
    let state = process_keystroke(state, "", true, false);
    assert_eq!(state.words[0].typed, "");
}

#[test]
fn delegates_ctrl_backspace_correctly() {
    let state = create_input_state(&["hi".into()]);
    let state = process_keystroke(state, "h", false, false);
    let state = process_keystroke(state, "i", false, false);
    let state = process_keystroke(state, "", true, true);
    assert_eq!(state.words[0].typed, "");
}

#[test]
fn delegates_space_correctly() {
    let state = create_input_state(&["a".into(), "b".into()]);
    let state = process_keystroke(state, "a", false, false);
    let state = process_keystroke(state, " ", false, false);
    assert_eq!(state.current_word_index, 1);
}

#[test]
fn ignores_control_characters() {
    let state = create_input_state(&["hi".into()]);
    let next = process_keystroke(state, "\x01", false, false);
    assert_eq!(next.words[0].typed, "");
}

#[test]
fn ignores_empty_string_input() {
    let state = create_input_state(&["hi".into()]);
    let next = process_keystroke(state, "", false, false);
    assert_eq!(next.words[0].typed, "");
}

#[test]
fn ignores_multi_char_input() {
    let state = create_input_state(&["hi".into()]);
    let next = process_keystroke(state, "abc", false, false);
    assert_eq!(next.words[0].typed, "");
}

// ── Keypress counters ───────────────────────────────────────────

#[test]
fn increments_correct_counter_for_correct_chars() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'h');
    assert_eq!(state.keypress_correct, 1);
    assert_eq!(state.keypress_incorrect, 0);
}

#[test]
fn increments_incorrect_counter_for_wrong_chars() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'x');
    assert_eq!(state.keypress_correct, 0);
    assert_eq!(state.keypress_incorrect, 1);
}

#[test]
fn increments_correct_counter_for_correct_space() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let state = process_char(state, 'h');
    let state = process_char(state, 'i');
    let state = process_space(state);
    // 2 correct chars + 1 correct space = 3
    assert_eq!(state.keypress_correct, 3);
    assert_eq!(state.keypress_incorrect, 0);
}

#[test]
fn increments_incorrect_counter_for_space_on_wrong_word() {
    let state = create_input_state(&["hi".into(), "ok".into()]);
    let state = process_char(state, 'x');
    let state = process_char(state, 'i');
    let state = process_space(state);
    // x=incorrect, i=correct, space=incorrect (word wrong)
    assert_eq!(state.keypress_correct, 1);
    assert_eq!(state.keypress_incorrect, 2);
}

#[test]
fn does_not_decrement_counters_on_backspace() {
    let state = create_input_state(&["hi".into()]);
    let state = process_char(state, 'x'); // incorrect
    assert_eq!(state.keypress_incorrect, 1);
    let state = process_backspace(state);
    // Counter should stay at 1
    assert_eq!(state.keypress_incorrect, 1);
    let state = process_char(state, 'h'); // correct
    assert_eq!(state.keypress_correct, 1);
    assert_eq!(state.keypress_incorrect, 1);
}
