import { describe, it, expect } from "vitest";
import {
  createInputState,
  processChar,
  processSpace,
  processBackspace,
  processCtrlBackspace,
  processKeystroke,
} from "../../src/engine/input-processor.js";

describe("createInputState", () => {
  it("initializes with correct defaults", () => {
    const state = createInputState(["hello", "world"]);
    expect(state.words).toHaveLength(2);
    expect(state.currentWordIndex).toBe(0);
    expect(state.cursorPosition).toBe(0);
    expect(state.finished).toBe(false);
    expect(state.words[0].word).toBe("hello");
    expect(state.words[0].typed).toBe("");
    expect(state.words[0].completed).toBe(false);
    expect(state.historicalErrorChars).toBe(0);
  });
});

describe("processChar", () => {
  it("adds a correct character", () => {
    const state = createInputState(["hello"]);
    const next = processChar(state, "h");
    expect(next.words[0].typed).toBe("h");
    expect(next.words[0].chars).toHaveLength(1);
    expect(next.words[0].chars[0].correct).toBe(true);
    expect(next.words[0].chars[0].extra).toBe(false);
    expect(next.cursorPosition).toBe(1);
  });

  it("adds an incorrect character", () => {
    const state = createInputState(["hello"]);
    const next = processChar(state, "x");
    expect(next.words[0].typed).toBe("x");
    expect(next.words[0].chars[0].correct).toBe(false);
    expect(next.words[0].chars[0].extra).toBe(false);
    expect(next.historicalErrorChars).toBe(1);
  });

  it("marks extra characters beyond word length", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processChar(state, "x"); // extra
    expect(state.words[0].chars[2].extra).toBe(true);
    expect(state.words[0].chars[2].correct).toBe(false);
    expect(state.cursorPosition).toBe(3);
    expect(state.historicalErrorChars).toBe(1);
  });

  it("does nothing when finished", () => {
    const state = createInputState(["a"]);
    const finished = { ...state, finished: true };
    const next = processChar(finished, "a");
    expect(next.words[0].typed).toBe("");
  });
});

describe("processSpace", () => {
  it("advances to the next word", () => {
    let state = createInputState(["hello", "world"]);
    state = processChar(state, "h");
    state = processChar(state, "e");
    state = processChar(state, "l");
    state = processChar(state, "l");
    state = processChar(state, "o");
    state = processSpace(state);
    expect(state.words[0].completed).toBe(true);
    expect(state.currentWordIndex).toBe(1);
    expect(state.cursorPosition).toBe(0);
  });

  it("does not advance if nothing typed", () => {
    const state = createInputState(["hello", "world"]);
    const next = processSpace(state);
    expect(next.currentWordIndex).toBe(0);
  });

  it("sets finished on last word", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processSpace(state);
    expect(state.finished).toBe(true);
    expect(state.words[0].completed).toBe(true);
  });

  it("allows advancing with incomplete word", () => {
    let state = createInputState(["hello", "world"]);
    state = processChar(state, "h");
    state = processSpace(state);
    expect(state.words[0].completed).toBe(true);
    expect(state.currentWordIndex).toBe(1);
  });
});

describe("processBackspace", () => {
  it("removes the last character", () => {
    let state = createInputState(["hello"]);
    state = processChar(state, "h");
    state = processChar(state, "e");
    state = processBackspace(state);
    expect(state.words[0].typed).toBe("h");
    expect(state.words[0].chars).toHaveLength(1);
    expect(state.cursorPosition).toBe(1);
  });

  it("goes back to previous completed word", () => {
    let state = createInputState(["hi", "there"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processSpace(state);
    expect(state.currentWordIndex).toBe(1);

    state = processBackspace(state);
    expect(state.currentWordIndex).toBe(0);
    expect(state.words[0].completed).toBe(false);
    expect(state.cursorPosition).toBe(2);
  });

  it("does nothing at start of first word", () => {
    const state = createInputState(["hello"]);
    const next = processBackspace(state);
    expect(next).toEqual(state);
  });

  it("keeps historical errors after deleting an incorrect character", () => {
    let state = createInputState(["hello"]);
    state = processChar(state, "x"); // incorrect
    expect(state.historicalErrorChars).toBe(1);

    state = processBackspace(state);
    expect(state.words[0].typed).toBe("");
    expect(state.words[0].chars).toHaveLength(0);
    expect(state.historicalErrorChars).toBe(1);
  });
});

describe("processCtrlBackspace", () => {
  it("clears the entire current word", () => {
    let state = createInputState(["hello"]);
    state = processChar(state, "h");
    state = processChar(state, "e");
    state = processChar(state, "l");
    state = processCtrlBackspace(state);
    expect(state.words[0].typed).toBe("");
    expect(state.words[0].chars).toHaveLength(0);
    expect(state.cursorPosition).toBe(0);
  });

  it("does nothing on empty word", () => {
    const state = createInputState(["hello"]);
    const next = processCtrlBackspace(state);
    expect(next.words[0].typed).toBe("");
    expect(next.cursorPosition).toBe(0);
  });

  it("does nothing when finished", () => {
    let state = createInputState(["a"]);
    state = processChar(state, "a");
    state = processSpace(state);
    expect(state.finished).toBe(true);
    const next = processCtrlBackspace(state);
    expect(next.finished).toBe(true);
  });

  it("clears extra characters too", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processChar(state, "x"); // extra
    state = processChar(state, "y"); // extra
    state = processCtrlBackspace(state);
    expect(state.words[0].typed).toBe("");
    expect(state.words[0].chars).toHaveLength(0);
  });
});

describe("processKeystroke", () => {
  it("delegates backspace correctly", () => {
    let state = createInputState(["hi"]);
    state = processKeystroke(state, "h", {});
    state = processKeystroke(state, "", { backspace: true });
    expect(state.words[0].typed).toBe("");
  });

  it("delegates ctrl+backspace correctly", () => {
    let state = createInputState(["hi"]);
    state = processKeystroke(state, "h", {});
    state = processKeystroke(state, "i", {});
    state = processKeystroke(state, "", { backspace: true, ctrl: true });
    expect(state.words[0].typed).toBe("");
  });

  it("delegates space correctly", () => {
    let state = createInputState(["a", "b"]);
    state = processKeystroke(state, "a", {});
    state = processKeystroke(state, " ", {});
    expect(state.currentWordIndex).toBe(1);
  });

  it("ignores control characters", () => {
    const state = createInputState(["hi"]);
    const next = processKeystroke(state, "\x01", {}); // ctrl+a
    expect(next.words[0].typed).toBe("");
  });

  it("ignores empty string input", () => {
    const state = createInputState(["hi"]);
    const next = processKeystroke(state, "", {});
    expect(next.words[0].typed).toBe("");
  });

  it("ignores multi-char input", () => {
    const state = createInputState(["hi"]);
    const next = processKeystroke(state, "abc", {});
    expect(next.words[0].typed).toBe("");
  });
});

describe("createInputState", () => {
  it("initializes keypress counters at zero", () => {
    const state = createInputState(["hello"]);
    expect(state.keypressCorrect).toBe(0);
    expect(state.keypressIncorrect).toBe(0);
  });

  it("handles single-character words", () => {
    const state = createInputState(["a", "b", "c"]);
    expect(state.words).toHaveLength(3);
    expect(state.words[0].word).toBe("a");
  });
});

describe("keypress counters", () => {
  it("increments correct counter for correct chars", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    expect(state.keypressCorrect).toBe(1);
    expect(state.keypressIncorrect).toBe(0);
  });

  it("increments incorrect counter for wrong chars", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "x");
    expect(state.keypressCorrect).toBe(0);
    expect(state.keypressIncorrect).toBe(1);
  });

  it("increments correct counter for correct space", () => {
    let state = createInputState(["hi", "ok"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processSpace(state);
    // 2 correct chars + 1 correct space = 3
    expect(state.keypressCorrect).toBe(3);
    expect(state.keypressIncorrect).toBe(0);
  });

  it("increments incorrect counter for space on wrong word", () => {
    let state = createInputState(["hi", "ok"]);
    state = processChar(state, "x");
    state = processChar(state, "i");
    state = processSpace(state);
    // x=incorrect, i=correct, space=incorrect (word wrong)
    expect(state.keypressCorrect).toBe(1);
    expect(state.keypressIncorrect).toBe(2);
  });

  it("does not decrement counters on backspace", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "x"); // incorrect
    expect(state.keypressIncorrect).toBe(1);
    state = processBackspace(state);
    // Counter should stay at 1
    expect(state.keypressIncorrect).toBe(1);
    state = processChar(state, "h"); // correct
    expect(state.keypressCorrect).toBe(1);
    expect(state.keypressIncorrect).toBe(1);
  });
});

describe("processBackspace edge cases", () => {
  it("does nothing when finished", () => {
    let state = createInputState(["a"]);
    state = processChar(state, "a");
    state = processSpace(state);
    expect(state.finished).toBe(true);
    const next = processBackspace(state);
    expect(next.finished).toBe(true);
  });

  it("handles multiple backspaces in a row", () => {
    let state = createInputState(["hello"]);
    state = processChar(state, "h");
    state = processChar(state, "e");
    state = processChar(state, "l");
    state = processBackspace(state);
    state = processBackspace(state);
    state = processBackspace(state);
    expect(state.words[0].typed).toBe("");
    expect(state.cursorPosition).toBe(0);
    // One more backspace at start should do nothing
    state = processBackspace(state);
    expect(state.cursorPosition).toBe(0);
  });

  it("removes extra characters with backspace", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processChar(state, "x"); // extra
    state = processBackspace(state);
    expect(state.words[0].typed).toBe("hi");
    expect(state.words[0].chars).toHaveLength(2);
  });
});

describe("processSpace edge cases", () => {
  it("does nothing when finished", () => {
    let state = createInputState(["a"]);
    state = processChar(state, "a");
    state = processSpace(state);
    expect(state.finished).toBe(true);
    const next = processSpace(state);
    expect(next.finished).toBe(true);
  });

  it("handles word with only wrong characters", () => {
    let state = createInputState(["hi", "ok"]);
    state = processChar(state, "x");
    state = processChar(state, "y");
    state = processSpace(state);
    expect(state.words[0].completed).toBe(true);
    expect(state.currentWordIndex).toBe(1);
  });
});
