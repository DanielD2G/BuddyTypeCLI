import { useReducer, useCallback, useEffect, useRef } from "react";
import type { TestConfig, TestPhase, TestResult } from "../types/test.js";
import type { StatsSnapshot } from "../types/stats.js";
import {
  createInputState,
  processKeystroke,
  type InputState,
} from "../engine/input-processor.js";
import {
  calculateStats,
  calculateConsistency,
} from "../engine/stats-calculator.js";
import { generateWords } from "../engine/word-generator.js";
import { useTimer } from "./use-timer.js";

interface TypingTestState {
  phase: TestPhase;
  config: TestConfig;
  inputState: InputState;
  words: string[];
  currentStats: StatsSnapshot;
  wpmHistory: number[];
  result: TestResult | null;
}

type TypingTestAction =
  | { type: "KEYSTROKE"; input: string; key: { backspace?: boolean; ctrl?: boolean } }
  | { type: "TICK"; elapsedSeconds: number }
  | { type: "FINISH"; elapsedSeconds: number }
  | { type: "RESTART" };

function createInitialState(config: TestConfig): TypingTestState {
  const count = config.mode === "words" ? config.wordCount : 100;
  const words = generateWords({
    language: config.language,
    count,
    punctuation: config.punctuation,
    numbers: config.numbers,
  });

  return {
    phase: "idle",
    config,
    inputState: createInputState(words),
    words,
    currentStats: {
      wpm: 0,
      rawWpm: 0,
      accuracy: 0,
      correctChars: 0,
      incorrectChars: 0,
      extraChars: 0,
      missedChars: 0,
      elapsedSeconds: 0,
    },
    wpmHistory: [],
    result: null,
  };
}

function computeResult(state: TypingTestState, elapsedSeconds: number): TestResult {
  const stats = calculateStats(state.inputState, elapsedSeconds);
  const consistency = calculateConsistency(state.wpmHistory);

  let correctWords = 0;
  let totalWords = 0;
  for (const w of state.inputState.words) {
    if (w.completed) {
      totalWords++;
      if (w.typed === w.word) correctWords++;
    }
  }

  return {
    wpm: Math.round(stats.wpm),
    rawWpm: Math.round(stats.rawWpm),
    accuracy: stats.accuracy,
    consistency,
    correctChars: stats.correctChars,
    incorrectChars: stats.incorrectChars,
    extraChars: stats.extraChars,
    missedChars: stats.missedChars,
    totalWords,
    correctWords,
    elapsedSeconds,
    config: state.config,
  };
}

function reducer(state: TypingTestState, action: TypingTestAction): TypingTestState {
  switch (action.type) {
    case "KEYSTROKE": {
      if (state.phase === "finished") return state;

      const newInputState = processKeystroke(
        state.inputState,
        action.input,
        action.key,
      );

      // Start on first keystroke
      const newPhase: TestPhase =
        state.phase === "idle" ? "active" : state.phase;

      // In words mode, check if all words are completed
      if (
        state.config.mode === "words" &&
        newInputState.finished
      ) {
        return {
          ...state,
          phase: "finished",
          inputState: newInputState,
        };
      }

      return {
        ...state,
        phase: newPhase,
        inputState: newInputState,
      };
    }

    case "TICK": {
      if (state.phase !== "active") return state;

      const stats = calculateStats(state.inputState, action.elapsedSeconds);
      const newHistory = [...state.wpmHistory, stats.rawWpm];

      return {
        ...state,
        currentStats: stats,
        wpmHistory: newHistory,
      };
    }

    case "FINISH": {
      if (state.phase !== "active") return state;
      const result = computeResult(state, action.elapsedSeconds);

      return {
        ...state,
        phase: "finished",
        result,
      };
    }

    case "RESTART": {
      return createInitialState(state.config);
    }

    default:
      return state;
  }
}

export interface UseTypingTestReturn {
  phase: TestPhase;
  inputState: InputState;
  currentStats: StatsSnapshot;
  wpmHistory: number[];
  result: TestResult | null;
  elapsedSeconds: number;
  remainingSeconds: number;
  handleInput: (input: string, key: { backspace?: boolean; ctrl?: boolean }) => void;
  restart: () => void;
}

export function useTypingTest(config: TestConfig): UseTypingTestReturn {
  const [state, dispatch] = useReducer(reducer, config, createInitialState);

  const timeLimit = config.mode === "time" ? config.timeLimit : null;
  const timer = useTimer(timeLimit);
  const tickRef = useRef<ReturnType<typeof setInterval> | null>(null);
  const elapsedRef = useRef(0);

  // Keep ref in sync so interval closure always reads the latest value
  useEffect(() => {
    elapsedRef.current = timer.elapsedSeconds;
  }, [timer.elapsedSeconds]);

  const handleInput = useCallback(
    (input: string, key: { backspace?: boolean; ctrl?: boolean }) => {
      dispatch({ type: "KEYSTROKE", input, key });
    },
    [],
  );

  const restart = useCallback(() => {
    timer.reset(timeLimit);
    dispatch({ type: "RESTART" });
  }, [timer, timeLimit]);

  // Start timer on first keystroke
  useEffect(() => {
    if (state.phase === "active" && !timer.running) {
      timer.start();
    }
  }, [state.phase, timer.running, timer]);

  // Tick stats every second — only depend on phase to avoid recreating interval
  useEffect(() => {
    if (state.phase === "active") {
      tickRef.current = setInterval(() => {
        dispatch({ type: "TICK", elapsedSeconds: elapsedRef.current });
      }, 1000);
    }

    return () => {
      if (tickRef.current) {
        clearInterval(tickRef.current);
        tickRef.current = null;
      }
    };
  }, [state.phase]);

  // Handle timer expiry (time mode)
  useEffect(() => {
    if (timer.expired && state.phase === "active") {
      dispatch({ type: "FINISH", elapsedSeconds: elapsedRef.current });
    }
  }, [timer.expired, state.phase]);

  // Handle words mode completion — read elapsed from ref to avoid stale closure
  useEffect(() => {
    if (state.phase === "finished" && !state.result) {
      dispatch({ type: "FINISH", elapsedSeconds: elapsedRef.current });
    }
  }, [state.phase, state.result]);

  return {
    phase: state.phase,
    inputState: state.inputState,
    currentStats: state.currentStats,
    wpmHistory: state.wpmHistory,
    result: state.result,
    elapsedSeconds: timer.elapsedSeconds,
    remainingSeconds: timer.remainingSeconds,
    handleInput,
    restart,
  };
}
