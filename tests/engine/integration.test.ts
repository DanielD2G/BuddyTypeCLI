import { describe, it, expect } from "vitest";
import { generateWords } from "../../src/engine/word-generator.js";
import {
  createInputState,
  processChar,
  processSpace,
  processBackspace,
  processCtrlBackspace,
} from "../../src/engine/input-processor.js";
import {
  calculateStats,
  calculateConsistency,
} from "../../src/engine/stats-calculator.js";
import {
  createTimer,
  startTimer,
  getElapsedSeconds,
  getRemainingSeconds,
} from "../../src/engine/timer.js";

describe("full typing test flow", () => {
  it("simulates a complete word-mode test", () => {
    // 1. Generate words
    const words = generateWords({
      language: "english",
      count: 3,
      punctuation: false,
      numbers: false,
    });
    expect(words).toHaveLength(3);

    // 2. Create input state
    let state = createInputState(words);
    expect(state.finished).toBe(false);

    // 3. Type each word perfectly + space
    for (let w = 0; w < words.length; w++) {
      for (const ch of words[w]) {
        state = processChar(state, ch);
      }
      state = processSpace(state);
    }

    // 4. Should be finished
    expect(state.finished).toBe(true);

    // 5. Calculate stats
    const stats = calculateStats(state, 10);
    expect(stats.wpm).toBeGreaterThan(0);
    expect(stats.accuracy).toBe(100);
    expect(stats.incorrectChars).toBe(0);
    expect(stats.extraChars).toBe(0);
    expect(stats.missedChars).toBe(0);
  });

  it("simulates a test with errors and corrections", () => {
    const words = ["hello", "world"];
    let state = createInputState(words);

    // Type "hello" with a mistake, then fix it
    state = processChar(state, "h");
    state = processChar(state, "x"); // typo
    state = processBackspace(state);  // fix it
    state = processChar(state, "e");
    state = processChar(state, "l");
    state = processChar(state, "l");
    state = processChar(state, "o");
    state = processSpace(state);

    // Type "world" correctly
    for (const ch of "world") {
      state = processChar(state, ch);
    }
    state = processSpace(state);

    expect(state.finished).toBe(true);

    const stats = calculateStats(state, 30);
    // Both words typed correctly (after correction)
    expect(stats.wpm).toBeGreaterThan(0);
    // Accuracy should be less than 100 because of the corrected mistake
    expect(stats.accuracy).toBeLessThan(100);
    expect(stats.accuracy).toBeGreaterThan(0);
  });

  it("simulates ctrl+backspace during typing", () => {
    let state = createInputState(["test", "word"]);

    // Type half a word, then clear it
    state = processChar(state, "t");
    state = processChar(state, "x");
    state = processChar(state, "y");
    state = processCtrlBackspace(state);

    expect(state.words[0].typed).toBe("");
    expect(state.cursorPosition).toBe(0);
    expect(state.currentWordIndex).toBe(0);

    // Now type it correctly
    for (const ch of "test") {
      state = processChar(state, ch);
    }
    state = processSpace(state);
    expect(state.currentWordIndex).toBe(1);
  });

  it("calculates consistency from WPM history", () => {
    // Simulate a steady typer
    const steadyHistory = [60, 62, 58, 61, 59, 60];
    const steadyConsistency = calculateConsistency(steadyHistory);
    expect(steadyConsistency).toBeGreaterThan(90);

    // Simulate an erratic typer
    const erraticHistory = [20, 80, 15, 90, 25, 75];
    const erraticConsistency = calculateConsistency(erraticHistory);
    expect(erraticConsistency).toBeLessThan(40);

    // Steady should beat erratic
    expect(steadyConsistency).toBeGreaterThan(erraticConsistency);
  });

  it("timer works with the full test flow", () => {
    // Create and start a 30-second timer
    const timer = createTimer(30);
    const started = startTimer(timer);

    expect(started.running).toBe(true);
    expect(getElapsedSeconds(started)).toBe(0);
    expect(getRemainingSeconds(started)).toBe(30);

    // Simulate some elapsed time
    const midway = { ...started, elapsedMs: 15000 };
    expect(getElapsedSeconds(midway)).toBe(15);
    expect(getRemainingSeconds(midway)).toBe(15);
  });

  it("handles extra characters in stats", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processChar(state, "x"); // extra
    state = processChar(state, "y"); // extra
    state = processSpace(state);

    const stats = calculateStats(state, 10);
    expect(stats.extraChars).toBe(2);
    // Word has extra chars so it's not counted as correct for WPM
    expect(stats.wpm).toBe(0);
    expect(stats.rawWpm).toBeGreaterThan(0);
  });

  it("handles missed characters in stats", () => {
    let state = createInputState(["hello", "ok"]);
    // Type only "he" then skip to next word
    state = processChar(state, "h");
    state = processChar(state, "e");
    state = processSpace(state);

    const stats = calculateStats(state, 10);
    expect(stats.missedChars).toBe(3); // l, l, o
  });

  it("going back to previous word preserves state", () => {
    let state = createInputState(["hello", "world"]);

    // Type "hello" and advance
    for (const ch of "hello") {
      state = processChar(state, ch);
    }
    state = processSpace(state);
    expect(state.currentWordIndex).toBe(1);

    // Go back to "hello"
    state = processBackspace(state);
    expect(state.currentWordIndex).toBe(0);
    expect(state.words[0].typed).toBe("hello");
    expect(state.words[0].completed).toBe(false);
    expect(state.cursorPosition).toBe(5);
  });
});
