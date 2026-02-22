export type TestMode = "time" | "words";

export interface TestConfig {
  mode: TestMode;
  timeLimit: number; // seconds (for time mode)
  wordCount: number; // number of words (for words mode)
  language: string; // language file name (e.g. "english")
  theme: string;
  oneLine: boolean;
  punctuation: boolean;
  numbers: boolean;
  backspace: boolean; // allow backspace to correct mistakes
}

export type TestPhase = "idle" | "active" | "finished";

export interface TestResult {
  wpm: number;
  rawWpm: number;
  accuracy: number;
  consistency: number;
  correctChars: number;
  incorrectChars: number;
  extraChars: number;
  missedChars: number;
  totalWords: number;
  correctWords: number;
  elapsedSeconds: number;
  config: TestConfig;
}

export interface ScoreEntry {
  wpm: number;
  rawWpm: number;
  accuracy: number;
  consistency: number;
  language: string;
  mode: TestMode;
  duration: number;
  date: string;
}

export const DEFAULT_CONFIG: TestConfig = {
  mode: "time",
  timeLimit: 30,
  wordCount: 25,
  language: "english",
  theme: "dark",
  oneLine: false,
  punctuation: false,
  numbers: false,
  backspace: true,
};
