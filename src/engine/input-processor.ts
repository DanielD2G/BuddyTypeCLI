import type { CharResult, WordState } from "../types/stats.js";

export interface InputState {
  words: WordState[];
  currentWordIndex: number;
  cursorPosition: number;
  finished: boolean;
  historicalErrorChars: number;
  /** Total correct keypresses (never decremented by backspace) */
  keypressCorrect: number;
  /** Total incorrect keypresses (never decremented by backspace) */
  keypressIncorrect: number;
}

export function createInputState(words: string[]): InputState {
  return {
    words: words.map((word) => ({
      word,
      typed: "",
      chars: [],
      completed: false,
    })),
    currentWordIndex: 0,
    cursorPosition: 0,
    finished: false,
    historicalErrorChars: 0,
    keypressCorrect: 0,
    keypressIncorrect: 0,
  };
}

export function processChar(state: InputState, char: string): InputState {
  if (state.finished) return state;

  const words = [...state.words];
  const idx = state.currentWordIndex;
  const current = { ...words[idx] };
  const pos = current.typed.length;
  const expected = current.word[pos] ?? "";
  const isExtra = pos >= current.word.length;

  const result: CharResult = {
    char,
    expected,
    correct: !isExtra && char === expected,
    extra: isExtra,
    timestamp: performance.now(),
  };

  current.typed = current.typed + char;
  current.chars = [...current.chars, result];
  words[idx] = current;

  const isCorrectKeypress = result.correct;

  return {
    ...state,
    words,
    cursorPosition: current.typed.length,
    historicalErrorChars:
      state.historicalErrorChars + (isCorrectKeypress ? 0 : 1),
    keypressCorrect: state.keypressCorrect + (isCorrectKeypress ? 1 : 0),
    keypressIncorrect: state.keypressIncorrect + (isCorrectKeypress ? 0 : 1),
  };
}

export function processSpace(state: InputState): InputState {
  if (state.finished) return state;

  const words = [...state.words];
  const idx = state.currentWordIndex;

  // Don't advance if nothing typed in current word
  if (words[idx].typed.length === 0) return state;

  const current = { ...words[idx] };
  current.completed = true;
  words[idx] = current;

  const nextIndex = idx + 1;
  const finished = nextIndex >= words.length;

  // Space is a correct keypress only if the word was typed correctly
  const spaceCorrect = current.typed === current.word;

  return {
    ...state,
    words,
    currentWordIndex: finished ? idx : nextIndex,
    cursorPosition: 0,
    finished,
    keypressCorrect: state.keypressCorrect + (spaceCorrect ? 1 : 0),
    keypressIncorrect: state.keypressIncorrect + (spaceCorrect ? 0 : 1),
  };
}

export function processBackspace(state: InputState): InputState {
  if (state.finished) return state;

  const words = [...state.words];
  const idx = state.currentWordIndex;
  const current = { ...words[idx] };

  if (current.typed.length > 0) {
    // Delete last character in current word
    current.typed = current.typed.slice(0, -1);
    current.chars = current.chars.slice(0, -1);
    words[idx] = current;

    return {
      ...state,
      words,
      cursorPosition: current.typed.length,
    };
  }

  // If at start of word and not the first word, go back to previous word
  if (idx > 0) {
    const prev = { ...words[idx - 1] };
    // Only allow going back to a word that was just completed
    if (prev.completed) {
      prev.completed = false;
      words[idx - 1] = prev;

      return {
        ...state,
        words,
        currentWordIndex: idx - 1,
        cursorPosition: prev.typed.length,
      };
    }
  }

  return state;
}

export function processCtrlBackspace(state: InputState): InputState {
  if (state.finished) return state;

  const words = [...state.words];
  const idx = state.currentWordIndex;
  const current = { ...words[idx] };

  current.typed = "";
  current.chars = [];
  words[idx] = current;

  return {
    ...state,
    words,
    cursorPosition: 0,
  };
}

export function processKeystroke(
  state: InputState,
  input: string,
  key: { backspace?: boolean; ctrl?: boolean },
): InputState {
  if (key.backspace) {
    if (key.ctrl) {
      return processCtrlBackspace(state);
    }
    return processBackspace(state);
  }

  if (input === " ") {
    return processSpace(state);
  }

  // Only process printable characters (single chars)
  if (input.length === 1 && input >= " ") {
    return processChar(state, input);
  }

  return state;
}
