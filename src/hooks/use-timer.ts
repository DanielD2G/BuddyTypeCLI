import { useState, useEffect, useCallback, useRef } from "react";
import {
  createTimer,
  startTimer,
  tickTimer,
  getElapsedSeconds,
  getRemainingSeconds,
  type TimerState,
} from "../engine/timer.js";

export interface UseTimerReturn {
  elapsedSeconds: number;
  remainingSeconds: number;
  running: boolean;
  expired: boolean;
  start: () => void;
  reset: (limitSeconds: number | null) => void;
}

export function useTimer(limitSeconds: number | null): UseTimerReturn {
  const [timerState, setTimerState] = useState<TimerState>(() =>
    createTimer(limitSeconds),
  );
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const start = useCallback(() => {
    setTimerState((prev) => startTimer(prev));
  }, []);

  const reset = useCallback((newLimit: number | null) => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    setTimerState(createTimer(newLimit));
  }, []);

  useEffect(() => {
    if (timerState.running && !intervalRef.current) {
      intervalRef.current = setInterval(() => {
        setTimerState((prev) => {
          const next = tickTimer(prev);
          if (next.expired && intervalRef.current) {
            clearInterval(intervalRef.current);
            intervalRef.current = null;
          }
          return next;
        });
      }, 100);
    }

    return () => {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
    };
  }, [timerState.running]);

  return {
    elapsedSeconds: getElapsedSeconds(timerState),
    remainingSeconds: getRemainingSeconds(timerState),
    running: timerState.running,
    expired: timerState.expired,
    start,
    reset,
  };
}
