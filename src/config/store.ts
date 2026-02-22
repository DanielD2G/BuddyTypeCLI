import Conf from "conf";
import type { TestConfig, TestResult, ScoreEntry } from "../types/test.js";

const config = new Conf<{ settings: Partial<TestConfig>; scores: ScoreEntry[] }>({
  projectName: "buddytype",
  defaults: {
    settings: {},
    scores: [],
  },
});

export function loadSettings(): Partial<TestConfig> {
  return config.get("settings", {});
}

export function saveSettings(settings: TestConfig): void {
  config.set("settings", settings);
}

export function saveScore(result: TestResult): void {
  const entry: ScoreEntry = {
    wpm: result.wpm,
    rawWpm: result.rawWpm,
    accuracy: result.accuracy,
    consistency: result.consistency,
    language: result.config.language,
    mode: result.config.mode,
    duration:
      result.config.mode === "time"
        ? result.config.timeLimit
        : result.config.wordCount,
    date: new Date().toISOString(),
  };

  const scores = config.get("scores", []);
  scores.unshift(entry);
  // Keep only last 100 scores
  config.set("scores", scores.slice(0, 100));
}

export function getScores(): ScoreEntry[] {
  return config.get("scores", []);
}
