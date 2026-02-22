use buddytype::engine::timer::*;
use std::time::{Duration, Instant};

#[test]
fn creates_timer_with_time_limit() {
    let timer = create_timer(Some(30));
    assert!(timer.start_time.is_none());
    assert_eq!(timer.elapsed_ms, 0.0);
    assert_eq!(timer.limit_ms, Some(30000.0));
    assert!(!timer.running);
    assert!(!timer.expired);
}

#[test]
fn creates_count_up_timer_when_limit_is_none() {
    let timer = create_timer(None);
    assert!(timer.limit_ms.is_none());
    assert!(!timer.running);
    assert!(!timer.expired);
}

#[test]
fn handles_zero_limit() {
    let timer = create_timer(Some(0));
    // 0 is treated like null
    assert!(timer.limit_ms.is_none());
}

#[test]
fn start_sets_running_and_records_start_time() {
    let timer = create_timer(Some(30));
    let now = Instant::now();
    let started = start_timer(timer, now);
    assert!(started.running);
    assert!(started.start_time.is_some());
}

#[test]
fn start_preserves_other_state() {
    let timer = create_timer(Some(60));
    let now = Instant::now();
    let started = start_timer(timer, now);
    assert_eq!(started.limit_ms, Some(60000.0));
    assert_eq!(started.elapsed_ms, 0.0);
    assert!(!started.expired);
}

#[test]
fn tick_returns_unchanged_if_not_running() {
    let timer = create_timer(Some(30));
    let now = Instant::now();
    let ticked = tick_timer(timer.clone(), now);
    assert_eq!(ticked.elapsed_ms, timer.elapsed_ms);
}

#[test]
fn tick_returns_unchanged_if_start_time_is_none() {
    let mut timer = create_timer(Some(30));
    timer.running = true;
    let now = Instant::now();
    let ticked = tick_timer(timer.clone(), now);
    assert_eq!(ticked.elapsed_ms, timer.elapsed_ms);
}

#[test]
fn tick_updates_elapsed_time() {
    let timer = create_timer(Some(30));
    let start = Instant::now();
    let started = start_timer(timer, start);

    // Simulate 5 seconds later
    let later = start + Duration::from_secs(5);
    let ticked = tick_timer(started, later);

    assert!((ticked.elapsed_ms - 5000.0).abs() < 10.0);
    assert!(ticked.running);
    assert!(!ticked.expired);
}

#[test]
fn tick_expires_when_elapsed_exceeds_limit() {
    let timer = create_timer(Some(1)); // 1 second limit
    let start = Instant::now();
    let started = start_timer(timer, start);

    let later = start + Duration::from_millis(1500);
    let ticked = tick_timer(started, later);

    assert!(ticked.expired);
    assert!(!ticked.running);
}

#[test]
fn tick_does_not_expire_before_limit() {
    let timer = create_timer(Some(1));
    let start = Instant::now();
    let started = start_timer(timer, start);

    let later = start + Duration::from_millis(500);
    let ticked = tick_timer(started, later);

    assert!(!ticked.expired);
    assert!(ticked.running);
}

#[test]
fn tick_never_expires_for_count_up_timer() {
    let timer = create_timer(None);
    let start = Instant::now();
    let started = start_timer(timer, start);

    let later = start + Duration::from_secs(999);
    let ticked = tick_timer(started, later);

    assert!(!ticked.expired);
    assert!(ticked.running);
}

#[test]
fn get_elapsed_converts_ms_to_seconds() {
    let mut timer = create_timer(Some(30));
    timer.elapsed_ms = 5000.0;
    assert_eq!(get_elapsed_seconds(&timer), 5.0);
}

#[test]
fn get_elapsed_returns_zero_for_fresh_timer() {
    let timer = create_timer(Some(30));
    assert_eq!(get_elapsed_seconds(&timer), 0.0);
}

#[test]
fn get_elapsed_handles_fractional_seconds() {
    let mut timer = create_timer(Some(30));
    timer.elapsed_ms = 1500.0;
    assert_eq!(get_elapsed_seconds(&timer), 1.5);
}

#[test]
fn get_remaining_returns_infinity_for_count_up() {
    let timer = create_timer(None);
    assert!(get_remaining_seconds(&timer).is_infinite());
}

#[test]
fn get_remaining_returns_full_time_for_fresh_timer() {
    let timer = create_timer(Some(30));
    assert_eq!(get_remaining_seconds(&timer), 30.0);
}

#[test]
fn get_remaining_calculates_correctly() {
    let mut timer = create_timer(Some(30));
    timer.elapsed_ms = 10000.0;
    assert_eq!(get_remaining_seconds(&timer), 20.0);
}

#[test]
fn get_remaining_clamps_to_zero() {
    let mut timer = create_timer(Some(30));
    timer.elapsed_ms = 35000.0;
    assert_eq!(get_remaining_seconds(&timer), 0.0);
}
