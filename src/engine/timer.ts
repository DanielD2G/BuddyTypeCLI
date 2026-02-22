export interface TimerState {
  startTime: number | null;
  elapsedMs: number;
  limitMs: number | null; // null for count-up (word mode)
  running: boolean;
  expired: boolean;
}

export function createTimer(limitSeconds: number | null): TimerState {
  return {
    startTime: null,
    elapsedMs: 0,
    limitMs: limitSeconds ? limitSeconds * 1000 : null,
    running: false,
    expired: false,
  };
}

export function startTimer(state: TimerState): TimerState {
  return {
    ...state,
    startTime: performance.now(),
    running: true,
  };
}

export function tickTimer(state: TimerState): TimerState {
  if (!state.running || !state.startTime) return state;

  const now = performance.now();
  const elapsedMs = now - state.startTime;
  const expired =
    state.limitMs !== null ? elapsedMs >= state.limitMs : false;

  return {
    ...state,
    elapsedMs,
    expired,
    running: !expired,
  };
}

export function getElapsedSeconds(state: TimerState): number {
  return state.elapsedMs / 1000;
}

export function getRemainingSeconds(state: TimerState): number {
  if (state.limitMs === null) return Infinity;
  const remaining = (state.limitMs - state.elapsedMs) / 1000;
  return Math.max(0, remaining);
}
