/** 🔸️ The cancels frame timer while tests of the ⏲️ frame pacing suite. */
import { expect, it, vi } from "vitest";
import { TICK_MILLISECONDS, createPacer, documentHidden, watchVisibility } from "@semio-tech/pets-react";
import { clock, stage, sum } from "../🟠️harness/🟦️.tsx";

/** 🔹 Registers the cancels frame timer while tests of the ⏲️ frame pacing suite. */
export function registerCancelsFrameTimerWhileTests(): void {
  it("cancels every frame and timer while hidden and never simulates the time away", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    for (let frame = 0; frame < 5; frame++) time.frame(16);
    expect(time.frames.size).toBe(1);
    pacer.hide();
    expect([time.frames.size, time.timers.size]).toEqual([0, 0]);
    const before = { steps: pets.steps.length, frames: time.asked.frames, timers: time.asked.timers };
    pacer.wake();
    time.pass(60_000);
    time.frame(16);
    pacer.hide();
    expect({ steps: pets.steps.length, frames: time.asked.frames, timers: time.asked.timers }).toEqual(before);
    pacer.show();
    pacer.show();
    expect([time.frames.size, time.asked.frames]).toEqual([1, before.frames + 1]);
    time.frame(16);
    time.frame(16);
    expect(pets.steps.slice(before.steps)).toEqual([1, 1]);
    pacer.stop();
  });
  it("resumes a running stage after a hidden stretch without a wake, a resting one only if it waits for a tick", () => {
    const running = clock();
    const busy = stage(32);
    const first = createPacer(busy.step, running.seams);
    first.wake();
    running.frame(16);
    first.hide();
    running.pass(10_000);
    first.show();
    running.frame(16);
    running.frame(16);
    expect(busy.steps).toEqual([1, 2]);
    first.stop();

    const waiting = clock();
    const dozing = stage(0, 641);
    const second = createPacer(dozing.step, waiting.seams);
    second.wake();
    waiting.frame(16);
    expect(waiting.timers.size).toBe(1);
    second.hide();
    expect([waiting.frames.size, waiting.timers.size]).toEqual([0, 0]);
    waiting.pass(3_600_000);
    second.show();
    expect([waiting.frames.size, waiting.timers.size]).toEqual([1, 0]);
    waiting.frame(16);
    expect(dozing.steps).toEqual([1, 1]);
    expect([waiting.frames.size, waiting.timers.size]).toEqual([0, 1]);
    expect([...waiting.timers.values()][0]!.delay).toBe((641 - 2) * TICK_MILLISECONDS - 0.375);
    second.stop();

    const resting = clock();
    const still = stage(0, null);
    const third = createPacer(still.step, resting.seams);
    third.wake();
    resting.frame(16);
    third.hide();
    third.show();
    expect([resting.frames.size, resting.timers.size, resting.asked.frames]).toEqual([0, 0, 1]);
    third.stop();
  });
  it("starts hidden when told so before the first wake", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.hide();
    pacer.wake();
    expect(time.asked.frames).toBe(0);
    time.pass(30_000);
    pacer.show();
    time.frame(16);
    expect(pets.steps).toEqual([1]);
    pacer.stop();
  });
  it("ends for good when stopped, even inside a step", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    time.frame(16);
    pacer.stop();
    expect([time.frames.size, time.timers.size]).toEqual([0, 0]);
    pacer.wake();
    pacer.show();
    time.frame(16);
    expect([time.frames.size, time.timers.size, pets.steps]).toEqual([0, 0, [1]]);

    const other = clock();
    let handle: { readonly stop: () => void } | undefined;
    const steps: number[] = [];
    handle = createPacer((ticks) => {
      steps.push(ticks);
      handle?.stop();
      return { tick: ticks, rate: 0, wake: 50 };
    }, other.seams);
    (handle as unknown as { readonly wake: () => void }).wake();
    other.frame(16);
    expect([steps, other.frames.size, other.timers.size]).toEqual([[1], 0, 0]);
  });
  it("runs on the browser's own frames and timers when no seam is given", () => {
    vi.useFakeTimers({ toFake: ["setTimeout", "clearTimeout", "requestAnimationFrame", "cancelAnimationFrame", "performance"] });
    const pets = stage(64);
    const pacer = createPacer(pets.step);
    expect(vi.getTimerCount()).toBe(0);
    pacer.wake();
    expect(vi.getTimerCount()).toBe(1);
    vi.advanceTimersByTime(4000);
    expect(sum(pets.steps)).toBe(256);
    pets.rate = 0;
    pets.wake = pets.tick + 640;
    vi.advanceTimersByTime(64);
    const rested = pets.steps.length;
    expect(vi.getTimerCount()).toBe(1);
    vi.advanceTimersByTime(9000);
    expect(pets.steps).toHaveLength(rested);
    expect(vi.getTimerCount()).toBe(1);
    pets.wake = null;
    vi.advanceTimersByTime(2000);
    expect(pets.steps).toHaveLength(rested + 1);
    expect(pets.tick).toBeGreaterThanOrEqual(256 + 640);
    expect(vi.getTimerCount()).toBe(0);
    pacer.wake();
    expect(vi.getTimerCount()).toBe(1);
    pacer.hide();
    expect(vi.getTimerCount()).toBe(0);
    pacer.stop();
  });
  it("tells when the document is hidden or on its way out and leaves no listener behind", () => {
    let state: DocumentVisibilityState = "visible";
    vi.spyOn(document, "visibilityState", "get").mockImplementation(() => state);
    expect(documentHidden(window)).toBe(false);
    const changes: boolean[] = [];
    const added: string[] = [];
    const removed: string[] = [];
    for (const target of [document, window] as const) {
      const add = target.addEventListener.bind(target);
      const remove = target.removeEventListener.bind(target);
      vi.spyOn(target, "addEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject, options?: boolean | AddEventListenerOptions) => {
        added.push(type);
        add(type, listener, options);
      });
      vi.spyOn(target, "removeEventListener").mockImplementation((type: string, listener: EventListenerOrEventListenerObject, options?: boolean | EventListenerOptions) => {
        removed.push(type);
        remove(type, listener, options);
      });
    }
    const stop = watchVisibility(window, (hidden) => changes.push(hidden));
    expect(added.sort()).toEqual(["pagehide", "pageshow", "visibilitychange"]);
    state = "hidden";
    document.dispatchEvent(new Event("visibilitychange"));
    expect(documentHidden(window)).toBe(true);
    state = "visible";
    document.dispatchEvent(new Event("visibilitychange"));
    window.dispatchEvent(new Event("pagehide"));
    window.dispatchEvent(new Event("pageshow"));
    expect(changes).toEqual([true, false, true, false]);
    stop();
    expect(removed.sort()).toEqual(["pagehide", "pageshow", "visibilitychange"]);
    document.dispatchEvent(new Event("visibilitychange"));
    window.dispatchEvent(new Event("pagehide"));
    expect(changes).toHaveLength(4);
  });
}
