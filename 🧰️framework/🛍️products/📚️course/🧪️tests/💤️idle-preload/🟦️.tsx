/** 💤️ What the learner will open next is loaded while the browser has nothing better to do, so opening it later is not the moment its code is compiled: the work waits for idle time, runs once per wish, and stops with the screen that wished it.
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/💤️idle-preload/🟦️.ts
 * @see https://developer.mozilla.org/en-US/docs/Web/API/Window/requestIdleCallback
 */

import { renderHook } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { IDLE_FALLBACK_MS, usePreload, whenIdle } from "@semio-tech/course-react";

beforeEach(() => {
  vi.useFakeTimers();
});

afterEach(() => {
  vi.useRealTimers();
  vi.unstubAllGlobals();
});

describe("whenIdle", () => {
  it("waits for the browser's idle time where there is one, and can be called off", () => {
    const callbacks: (() => void)[] = [];
    const cancelled: number[] = [];
    vi.stubGlobal("requestIdleCallback", (callback: () => void) => callbacks.push(callback));
    vi.stubGlobal("cancelIdleCallback", (handle: number) => cancelled.push(handle));
    const work = vi.fn();
    const cancel = whenIdle(work);
    vi.advanceTimersByTime(10_000);
    expect(work).not.toHaveBeenCalled();
    callbacks[0]!();
    expect(work).toHaveBeenCalledTimes(1);
    cancel();
    expect(cancelled).toEqual([1]);
  });

  it("falls back to a timer without idle callbacks, and a call-off before it runs keeps the work from running", () => {
    vi.stubGlobal("requestIdleCallback", undefined);
    const work = vi.fn();
    whenIdle(work);
    vi.advanceTimersByTime(IDLE_FALLBACK_MS - 1);
    expect(work).not.toHaveBeenCalled();
    vi.advanceTimersByTime(1);
    expect(work).toHaveBeenCalledTimes(1);
    const never = vi.fn();
    whenIdle(never)();
    vi.advanceTimersByTime(IDLE_FALLBACK_MS * 2);
    expect(never).not.toHaveBeenCalled();
  });
});

describe("usePreload", () => {
  it("loads every wish once the screen is active and idle, not again on a render, and not for a screen that leaves first", () => {
    vi.stubGlobal("requestIdleCallback", undefined);
    const loads = [vi.fn(), vi.fn()];
    const { rerender, unmount } = renderHook(({ active }) => usePreload(active, loads), { initialProps: { active: true } });
    expect(loads[0]).not.toHaveBeenCalled();
    vi.advanceTimersByTime(IDLE_FALLBACK_MS);
    expect(loads.map((load) => load.mock.calls.length)).toEqual([1, 1]);
    rerender({ active: true });
    vi.advanceTimersByTime(IDLE_FALLBACK_MS * 3);
    expect(loads.map((load) => load.mock.calls.length)).toEqual([1, 1]);
    unmount();
    const late = vi.fn();
    const left = renderHook(() => usePreload(true, [late]));
    left.unmount();
    vi.advanceTimersByTime(IDLE_FALLBACK_MS * 3);
    expect(late).not.toHaveBeenCalled();
  });

  it("wishes nothing while the screen is not active", () => {
    vi.stubGlobal("requestIdleCallback", undefined);
    const load = vi.fn();
    renderHook(() => usePreload(false, [load]));
    vi.advanceTimersByTime(IDLE_FALLBACK_MS * 3);
    expect(load).not.toHaveBeenCalled();
  });
});
