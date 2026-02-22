import { describe, it, expect, vi, beforeEach, afterEach } from "vitest";
import {
  createTimer,
  startTimer,
  tickTimer,
  getElapsedSeconds,
  getRemainingSeconds,
} from "../../src/engine/timer.js";

describe("createTimer", () => {
  it("creates a timer with time limit", () => {
    const timer = createTimer(30);
    expect(timer.startTime).toBeNull();
    expect(timer.elapsedMs).toBe(0);
    expect(timer.limitMs).toBe(30000);
    expect(timer.running).toBe(false);
    expect(timer.expired).toBe(false);
  });

  it("creates a count-up timer when limit is null", () => {
    const timer = createTimer(null);
    expect(timer.limitMs).toBeNull();
    expect(timer.running).toBe(false);
    expect(timer.expired).toBe(false);
  });

  it("handles zero limit", () => {
    const timer = createTimer(0);
    // 0 is falsy, so limitMs should be null
    expect(timer.limitMs).toBeNull();
  });
});

describe("startTimer", () => {
  it("sets running to true and records start time", () => {
    const timer = createTimer(30);
    const started = startTimer(timer);
    expect(started.running).toBe(true);
    expect(started.startTime).toBeTypeOf("number");
    expect(started.startTime).toBeGreaterThan(0);
  });

  it("preserves other state", () => {
    const timer = createTimer(60);
    const started = startTimer(timer);
    expect(started.limitMs).toBe(60000);
    expect(started.elapsedMs).toBe(0);
    expect(started.expired).toBe(false);
  });
});

describe("tickTimer", () => {
  let mockNow: number;

  beforeEach(() => {
    mockNow = 1000;
    vi.spyOn(performance, "now").mockImplementation(() => mockNow);
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  it("returns unchanged state if not running", () => {
    const timer = createTimer(30);
    const ticked = tickTimer(timer);
    expect(ticked).toBe(timer);
  });

  it("returns unchanged state if startTime is null", () => {
    const timer = { ...createTimer(30), running: true };
    const ticked = tickTimer(timer);
    expect(ticked).toBe(timer);
  });

  it("updates elapsed time", () => {
    const timer = createTimer(30);
    const started = startTimer(timer);
    // startTimer captured performance.now() = 1000

    // Advance mock time by 5 seconds
    mockNow = 6000;
    const ticked = tickTimer(started);

    expect(ticked.elapsedMs).toBe(5000);
    expect(ticked.running).toBe(true);
    expect(ticked.expired).toBe(false);
  });

  it("expires when elapsed exceeds limit", () => {
    const timer = createTimer(1); // 1 second limit = 1000ms
    const started = startTimer(timer);

    // Advance past the limit
    mockNow = 2500;
    const ticked = tickTimer(started);

    expect(ticked.expired).toBe(true);
    expect(ticked.running).toBe(false);
  });

  it("does not expire right before limit", () => {
    const timer = createTimer(1);
    const started = startTimer(timer);

    mockNow = 1500; // 500ms elapsed, limit is 1000ms
    const ticked = tickTimer(started);

    expect(ticked.expired).toBe(false);
    expect(ticked.running).toBe(true);
  });

  it("never expires for count-up timer (null limit)", () => {
    const timer = createTimer(null);
    const started = startTimer(timer);

    mockNow = 999999;
    const ticked = tickTimer(started);

    expect(ticked.expired).toBe(false);
    expect(ticked.running).toBe(true);
  });
});

describe("getElapsedSeconds", () => {
  it("converts milliseconds to seconds", () => {
    const timer = { ...createTimer(30), elapsedMs: 5000 };
    expect(getElapsedSeconds(timer)).toBe(5);
  });

  it("returns 0 for fresh timer", () => {
    const timer = createTimer(30);
    expect(getElapsedSeconds(timer)).toBe(0);
  });

  it("handles fractional seconds", () => {
    const timer = { ...createTimer(30), elapsedMs: 1500 };
    expect(getElapsedSeconds(timer)).toBe(1.5);
  });
});

describe("getRemainingSeconds", () => {
  it("returns Infinity for count-up timer", () => {
    const timer = createTimer(null);
    expect(getRemainingSeconds(timer)).toBe(Infinity);
  });

  it("returns full time for fresh timer", () => {
    const timer = createTimer(30);
    expect(getRemainingSeconds(timer)).toBe(30);
  });

  it("calculates remaining correctly", () => {
    const timer = { ...createTimer(30), elapsedMs: 10000 };
    expect(getRemainingSeconds(timer)).toBe(20);
  });

  it("clamps to 0 when elapsed exceeds limit", () => {
    const timer = { ...createTimer(30), elapsedMs: 35000 };
    expect(getRemainingSeconds(timer)).toBe(0);
  });
});
