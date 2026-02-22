use crate::types::TimerState;
use std::time::Instant;

pub fn create_timer(limit_seconds: Option<u32>) -> TimerState {
    TimerState {
        start_time: None,
        elapsed_ms: 0.0,
        limit_ms: limit_seconds
            .filter(|&s| s > 0)
            .map(|s| s as f64 * 1000.0),
        running: false,
        expired: false,
    }
}

pub fn start_timer(state: TimerState, now: Instant) -> TimerState {
    TimerState {
        start_time: Some(now),
        running: true,
        ..state
    }
}

pub fn tick_timer(state: TimerState, now: Instant) -> TimerState {
    if !state.running {
        return state;
    }
    let start = match state.start_time {
        Some(t) => t,
        None => return state,
    };

    let elapsed_ms = now.duration_since(start).as_secs_f64() * 1000.0;
    let expired = state
        .limit_ms
        .is_some_and(|limit| elapsed_ms >= limit);

    TimerState {
        elapsed_ms,
        expired,
        running: !expired,
        ..state
    }
}

pub fn get_elapsed_seconds(state: &TimerState) -> f64 {
    state.elapsed_ms / 1000.0
}

pub fn get_remaining_seconds(state: &TimerState) -> f64 {
    match state.limit_ms {
        None => f64::INFINITY,
        Some(limit) => ((limit - state.elapsed_ms) / 1000.0).max(0.0),
    }
}
