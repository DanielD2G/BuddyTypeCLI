import type { StatsSnapshot } from "../types/stats.js";
import type { InputState } from "./input-processor.js";

export function calculateStats(
  state: InputState,
  elapsedSeconds: number,
): StatsSnapshot {
  if (elapsedSeconds <= 0) {
    return {
      wpm: 0,
      rawWpm: 0,
      accuracy: 0,
      correctChars: 0,
      incorrectChars: 0,
      extraChars: 0,
      missedChars: 0,
      elapsedSeconds: 0,
    };
  }

  let correctWordChars = 0;
  let correctSpaces = 0;
  let allCorrectChars = 0;
  let incorrectChars = 0;
  let extraChars = 0;
  let missedChars = 0;
  let spaces = 0;

  for (let i = 0; i <= state.currentWordIndex && i < state.words.length; i++) {
    const wordState = state.words[i];

    for (const charResult of wordState.chars) {
      if (charResult.extra) {
        extraChars++;
      } else if (charResult.correct) {
        allCorrectChars++;
      } else {
        incorrectChars++;
      }
    }

    // Count missed characters (word was completed but not fully typed)
    if (wordState.completed && wordState.typed.length < wordState.word.length) {
      missedChars += wordState.word.length - wordState.typed.length;
    }

    if (wordState.completed) {
      spaces++;

      // WPM: only count chars from entirely correct words (MonkeyType formula)
      if (wordState.typed === wordState.word) {
        correctWordChars += wordState.word.length;
        correctSpaces++;
      }
    }
  }

  const minutes = elapsedSeconds / 60;

  // Net WPM: only correctly-typed whole words + their spaces (MonkeyType formula)
  const wpm = roundTo2((correctWordChars + correctSpaces) / 5 / minutes);

  // Raw WPM: all typed characters (correct + incorrect + extra + spaces)
  const totalTyped = allCorrectChars + incorrectChars + extraChars + spaces;
  const rawWpm = roundTo2(totalTyped / 5 / minutes);

  // Accuracy: per-keypress correct / (correct + incorrect) (MonkeyType formula)
  const totalKeypresses = state.keypressCorrect + state.keypressIncorrect;
  const accuracy =
    totalKeypresses > 0
      ? roundTo2((state.keypressCorrect / totalKeypresses) * 100)
      : 100;

  return {
    wpm: Math.max(0, wpm),
    rawWpm: Math.max(0, rawWpm),
    accuracy,
    correctChars: allCorrectChars,
    incorrectChars,
    extraChars,
    missedChars,
    elapsedSeconds,
  };
}

/**
 * MonkeyType's "kogasa" consistency function.
 * Maps the coefficient of variation (COV) from [0, +∞) to [100, 0).
 * Uses a modified tanh that approximates tanh(arctanh(x)) in [0, 1),
 * making it more sensitive in the typical typing range.
 */
function kogasa(cov: number): number {
  return 100 * (1 - Math.tanh(cov + Math.pow(cov, 3) / 3 + Math.pow(cov, 5) / 5));
}

export function calculateConsistency(wpmHistory: number[]): number {
  if (wpmHistory.length < 2) return 100;

  const mean = wpmHistory.reduce((a, b) => a + b, 0) / wpmHistory.length;
  if (mean === 0) return 0;

  const variance =
    wpmHistory.reduce((sum, val) => sum + (val - mean) ** 2, 0) /
    wpmHistory.length;
  const stddev = Math.sqrt(variance);
  const cov = stddev / mean;

  return roundTo2(kogasa(cov));
}

function roundTo2(n: number): number {
  return Math.round(n * 100) / 100;
}
