use crate::types::{CharResult, InputState, WordState};
use std::time::Instant;

pub fn create_input_state(words: &[String]) -> InputState {
    InputState {
        words: words
            .iter()
            .map(|w| WordState {
                word: w.clone(),
                typed: String::new(),
                chars: Vec::new(),
                completed: false,
            })
            .collect(),
        current_word_index: 0,
        cursor_position: 0,
        finished: false,
        historical_error_chars: 0,
        keypress_correct: 0,
        keypress_incorrect: 0,
    }
}

pub fn process_char(mut state: InputState, ch: char) -> InputState {
    if state.finished {
        return state;
    }

    let idx = state.current_word_index;
    let current = &mut state.words[idx];
    let pos = current.typed.len();
    let expected = current.word.chars().nth(pos).unwrap_or('\0');
    let is_extra = pos >= current.word.len();

    let correct = !is_extra && ch == expected;

    let result = CharResult {
        char: ch,
        expected,
        correct,
        extra: is_extra,
        timestamp: Instant::now(),
    };

    current.typed.push(ch);
    current.chars.push(result);

    state.cursor_position = current.typed.len();
    if !correct {
        state.historical_error_chars += 1;
    }
    if correct {
        state.keypress_correct += 1;
    } else {
        state.keypress_incorrect += 1;
    }

    state
}

pub fn process_space(mut state: InputState) -> InputState {
    if state.finished {
        return state;
    }

    let idx = state.current_word_index;

    // Don't advance if nothing typed in current word
    if state.words[idx].typed.is_empty() {
        return state;
    }

    let space_correct = state.words[idx].typed == state.words[idx].word;
    state.words[idx].completed = true;

    let next_index = idx + 1;
    let finished = next_index >= state.words.len();

    state.current_word_index = if finished { idx } else { next_index };
    state.cursor_position = 0;
    state.finished = finished;

    if space_correct {
        state.keypress_correct += 1;
    } else {
        state.keypress_incorrect += 1;
    }

    state
}

pub fn process_backspace(mut state: InputState) -> InputState {
    if state.finished {
        return state;
    }

    let idx = state.current_word_index;

    if !state.words[idx].typed.is_empty() {
        // Delete last character in current word
        state.words[idx].typed.pop();
        state.words[idx].chars.pop();
        state.cursor_position = state.words[idx].typed.len();
        return state;
    }

    // If at start of word and not the first word, go back to previous word
    if idx > 0 && state.words[idx - 1].completed {
        state.words[idx - 1].completed = false;
        state.current_word_index = idx - 1;
        state.cursor_position = state.words[idx - 1].typed.len();
    }

    state
}

pub fn process_ctrl_backspace(mut state: InputState) -> InputState {
    if state.finished {
        return state;
    }

    let idx = state.current_word_index;
    state.words[idx].typed.clear();
    state.words[idx].chars.clear();
    state.cursor_position = 0;

    state
}

pub fn process_keystroke(state: InputState, input: &str, backspace: bool, ctrl: bool) -> InputState {
    if backspace {
        if ctrl {
            return process_ctrl_backspace(state);
        }
        return process_backspace(state);
    }

    if input == " " {
        return process_space(state);
    }

    // Only process printable characters (single chars)
    let chars: Vec<char> = input.chars().collect();
    if chars.len() == 1 && chars[0] >= ' ' {
        return process_char(state, chars[0]);
    }

    state
}
