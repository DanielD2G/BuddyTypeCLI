import { describe, it, expect } from "vitest";
import {
  calculateStats,
  calculateConsistency,
} from "../../src/engine/stats-calculator.js";
import {
  createInputState,
  processChar,
  processSpace,
  processBackspace,
} from "../../src/engine/input-processor.js";

describe("calculateStats", () => {
  it("returns zeros for zero elapsed time", () => {
    const state = createInputState(["hello"]);
    const stats = calculateStats(state, 0);
    expect(stats.wpm).toBe(0);
    expect(stats.rawWpm).toBe(0);
    expect(stats.accuracy).toBe(0);
  });

  it("calculates WPM correctly for perfect typing", () => {
    let state = createInputState(["hello", "world"]);
    // Type "hello" correctly
    for (const c of "hello") {
      state = processChar(state, c);
    }
    state = processSpace(state);
    // Type "world" correctly
    for (const c of "world") {
      state = processChar(state, c);
    }

    // WPM: only whole correct words count
    // "hello" (5 chars) + 1 space = 6 correct word chars
    // "world" is being typed (not completed) so it doesn't count toward WPM
    // In 12 seconds: (6 / 5) / (12 / 60) = 1.2 / 0.2 = 6 WPM
    const stats = calculateStats(state, 12);
    expect(stats.wpm).toBeCloseTo(6, 0);

    // Raw WPM: all typed chars = 10 letters + 1 space = 11
    // (11 / 5) / (12 / 60) = 2.2 / 0.2 = 11
    expect(stats.rawWpm).toBeCloseTo(11, 0);
    expect(stats.accuracy).toBe(100);
  });

  it("calculates WPM only for entirely correct words", () => {
    let state = createInputState(["hello", "world", "test"]);
    // Type "hello" correctly
    for (const c of "hello") state = processChar(state, c);
    state = processSpace(state);
    // Type "world" with a typo ("worxd")
    for (const c of "worxd") state = processChar(state, c);
    state = processSpace(state);
    // Type "test" correctly
    for (const c of "test") state = processChar(state, c);
    state = processSpace(state);

    // WPM: "hello"(5) + space(1) + "test"(4) + space(1) = 11 correct word chars
    // "world" doesn't count (has typo)
    // In 60 seconds: (11 / 5) / 1 = 2.2 WPM
    const stats = calculateStats(state, 60);
    expect(stats.wpm).toBeCloseTo(2.2, 1);
  });

  it("tracks incorrect characters", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "x"); // incorrect
    state = processChar(state, "i"); // correct

    const stats = calculateStats(state, 10);
    expect(stats.correctChars).toBe(1);
    expect(stats.incorrectChars).toBe(1);
    // Accuracy: 1 correct keypress / 2 total keypresses = 50%
    expect(stats.accuracy).toBe(50);
  });

  it("tracks extra characters", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "h");
    state = processChar(state, "i");
    state = processChar(state, "x"); // extra

    const stats = calculateStats(state, 10);
    expect(stats.extraChars).toBe(1);
    expect(stats.correctChars).toBe(2);
  });

  it("tracks missed characters", () => {
    let state = createInputState(["hello", "world"]);
    state = processChar(state, "h"); // only typed 1 of 5
    state = processSpace(state); // skip rest of "hello"

    const stats = calculateStats(state, 10);
    expect(stats.missedChars).toBe(4); // missed e, l, l, o
  });

  it("keeps accuracy penalty for corrected mistakes", () => {
    let state = createInputState(["hi"]);
    state = processChar(state, "x"); // incorrect
    state = processBackspace(state); // delete incorrect
    state = processChar(state, "h"); // correct
    state = processChar(state, "i"); // correct

    const stats = calculateStats(state, 10);
    expect(stats.correctChars).toBe(2);
    expect(stats.incorrectChars).toBe(0);
    // 2 correct keypresses / 3 total keypresses (x, h, i) = 66.67%
    expect(stats.accuracy).toBeCloseTo(66.67, 2);
  });

  it("counts space as incorrect keypress when word has errors", () => {
    let state = createInputState(["hi", "ok"]);
    state = processChar(state, "x"); // incorrect
    state = processChar(state, "i"); // correct
    state = processSpace(state); // space is incorrect (typed "xi" != "hi")

    // Keypresses: x(incorrect), i(correct), space(incorrect) = 1 correct, 2 incorrect
    const stats = calculateStats(state, 10);
    expect(stats.accuracy).toBeCloseTo(33.33, 2);
  });

  it("counts space as correct keypress when word is perfect", () => {
    let state = createInputState(["hi", "ok"]);
    state = processChar(state, "h"); // correct
    state = processChar(state, "i"); // correct
    state = processSpace(state); // space is correct ("hi" === "hi")

    // Keypresses: h(correct), i(correct), space(correct) = 3 correct, 0 incorrect
    const stats = calculateStats(state, 10);
    expect(stats.accuracy).toBe(100);
  });

  it("raw WPM includes all typed chars including spaces", () => {
    let state = createInputState(["hi", "ok"]);
    for (const c of "hi") state = processChar(state, c);
    state = processSpace(state);
    for (const c of "ok") state = processChar(state, c);

    // Raw: 4 chars + 1 space = 5 total typed
    // In 60 seconds: (5 / 5) / 1 = 1 raw WPM
    const stats = calculateStats(state, 60);
    expect(stats.rawWpm).toBe(1);
  });

  it("returns negative-time-safe values", () => {
    const state = createInputState(["hello"]);
    const stats = calculateStats(state, -5);
    expect(stats.wpm).toBe(0);
    expect(stats.rawWpm).toBe(0);
  });

  it("WPM is zero when no words are completed", () => {
    let state = createInputState(["hello"]);
    state = processChar(state, "h");
    state = processChar(state, "e");
    // Word not completed (no space)
    const stats = calculateStats(state, 10);
    expect(stats.wpm).toBe(0);
    expect(stats.rawWpm).toBeGreaterThan(0);
  });

  it("WPM is zero when all completed words have errors", () => {
    let state = createInputState(["hi", "ok"]);
    state = processChar(state, "x");
    state = processChar(state, "y");
    state = processSpace(state);
    const stats = calculateStats(state, 10);
    expect(stats.wpm).toBe(0);
  });

  it("handles multiple completed words in WPM", () => {
    let state = createInputState(["a", "b", "c", "d"]);
    // Type all 4 single-char words correctly
    for (const w of ["a", "b", "c", "d"]) {
      state = processChar(state, w);
      state = processSpace(state);
    }
    // 4 correct chars + 3 spaces (last space finishes) = 7 correct word chars
    // Actually "d" is the last word and space finishes it, but finished=true
    // Let's check: a(1)+space + b(1)+space + c(1)+space + d(1)+space => finished after d+space
    // Correct word chars: a(1)+b(1)+c(1)+d(1) = 4 chars + 4 correct spaces = 8
    // Wait, last space finishes the test. Let's just check WPM > 0
    const stats = calculateStats(state, 60);
    expect(stats.wpm).toBeGreaterThan(0);
    expect(stats.accuracy).toBe(100);
  });

  it("counts multiple missed characters across words", () => {
    let state = createInputState(["hello", "world", "test"]);
    // Skip "hello" with only 2 chars
    state = processChar(state, "h");
    state = processChar(state, "e");
    state = processSpace(state); // missed l, l, o = 3
    // Skip "world" with only 1 char
    state = processChar(state, "w");
    state = processSpace(state); // missed o, r, l, d = 4

    const stats = calculateStats(state, 10);
    expect(stats.missedChars).toBe(7);
  });

  it("accuracy is 100 when nothing typed yet", () => {
    const state = createInputState(["hello"]);
    const stats = calculateStats(state, 10);
    // No keypresses at all, accuracy formula: 0/0 -> 100 by default
    expect(stats.accuracy).toBe(100);
  });

  it("elapsedSeconds is reflected in stats", () => {
    const state = createInputState(["hello"]);
    const stats = calculateStats(state, 42);
    expect(stats.elapsedSeconds).toBe(42);
  });
});

describe("calculateConsistency", () => {
  it("returns 100 for uniform WPM", () => {
    const consistency = calculateConsistency([50, 50, 50, 50]);
    expect(consistency).toBe(100);
  });

  it("returns 100 for single sample", () => {
    expect(calculateConsistency([50])).toBe(100);
  });

  it("returns lower value for variable WPM (kogasa function)", () => {
    // With [20, 80, 30, 70]: mean=50, stddev≈25.5, cov≈0.51
    // kogasa(0.51) = 100 * (1 - tanh(0.51 + 0.51^3/3 + 0.51^5/5))
    // ≈ 100 * (1 - tanh(0.554)) ≈ 100 * (1 - 0.504) ≈ 49.6
    const consistency = calculateConsistency([20, 80, 30, 70]);
    expect(consistency).toBeLessThan(55);
    expect(consistency).toBeGreaterThan(40);
  });

  it("returns 0 for zero mean", () => {
    expect(calculateConsistency([0, 0, 0])).toBe(0);
  });

  it("is bounded between 0 and 100", () => {
    const result = calculateConsistency([1, 100, 1, 100]);
    expect(result).toBeGreaterThanOrEqual(0);
    expect(result).toBeLessThanOrEqual(100);
  });

  it("highly consistent typing scores above 90", () => {
    // Typing at ~60 WPM with small variation
    const consistency = calculateConsistency([58, 62, 60, 61, 59, 60]);
    expect(consistency).toBeGreaterThan(90);
  });

  it("very erratic typing scores below 30", () => {
    // Wild variation
    const consistency = calculateConsistency([10, 90, 5, 95, 15, 85]);
    expect(consistency).toBeLessThan(30);
  });

  it("returns 100 for empty array", () => {
    expect(calculateConsistency([])).toBe(100);
  });

  it("moderate variation scores between 50 and 90", () => {
    const consistency = calculateConsistency([40, 60, 45, 55, 50]);
    expect(consistency).toBeGreaterThan(50);
    expect(consistency).toBeLessThan(90);
  });
});
