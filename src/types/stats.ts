export interface CharResult {
  char: string;
  expected: string;
  correct: boolean;
  extra: boolean;
  timestamp: number;
}

export interface WordState {
  word: string;
  typed: string;
  chars: CharResult[];
  completed: boolean;
}

export interface StatsSnapshot {
  wpm: number;
  rawWpm: number;
  accuracy: number;
  correctChars: number;
  incorrectChars: number;
  extraChars: number;
  missedChars: number;
  elapsedSeconds: number;
}
