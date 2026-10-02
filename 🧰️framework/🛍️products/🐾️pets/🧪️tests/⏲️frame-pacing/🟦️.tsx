/** ⏲️ How the pet layer keeps time: animation-frame timestamps become whole ticks (sixty-four a second, at most eight
 * a frame), a calmer stage skips frames, a stage at rest asks for no frame at all and waits on a single timer, a hidden
 * document runs nothing and its absence is never caught up. A hand-driven clock comes in through the seams; the
 * browser's own frames and timers are stood in for by the fake clock of `@sinonjs/fake-timers` (vitest's).
 *
 * @see ../../🎯️targets/⚛️react/🔨️modules/⏲️pacing/🟦️.ts
 * @see https://gafferongames.com/post/fix_your_timestep/
 */

import { afterEach, describe, expect, it, vi } from "vitest";
import { TICKS_PER_SECOND } from "@semio-tech/pets";
import { FRAME_TICKS, TICK_MILLISECONDS, createPacer, documentHidden, watchVisibility, type Pace, type PacerSeams } from "@semio-tech/pets-react";

/** 🕰️ A clock moved by hand: frames and timers wait in ledgers until the case lets time pass. */
function clock(): {
  readonly seams: PacerSeams;
  readonly frames: Map<number, (time: number) => void>;
  readonly timers: Map<number, { readonly callback: () => void; readonly delay: number; readonly at: number }>;
  readonly asked: { frames: number; timers: number };
  readonly pass: (milliseconds: number) => void;
  readonly frame: (milliseconds?: number) => void;
} {
  let time = 1000;
  let issued = 0;
  const frames = new Map<number, (time: number) => void>();
  const timers = new Map<number, { readonly callback: () => void; readonly delay: number; readonly at: number }>();
  const asked = { frames: 0, timers: 0 };
  const pass = (milliseconds: number): void => {
    time += milliseconds;
    for (const [handle, timer] of [...timers]) {
      if (timer.at > time) continue;
      timers.delete(handle);
      timer.callback();
    }
  };
  return {
    seams: {
      now: () => time,
      requestFrame: (callback) => {
        asked.frames += 1;
        issued += 1;
        frames.set(issued, callback);
        return issued;
      },
      cancelFrame: (handle) => {
        frames.delete(handle);
      },
      setTimer: (callback, delay) => {
        asked.timers += 1;
        issued += 1;
        timers.set(issued, { callback, delay, at: time + delay });
        return issued;
      },
      clearTimer: (handle) => {
        timers.delete(handle as number);
      },
    },
    frames,
    timers,
    asked,
    pass,
    frame: (milliseconds = 16) => {
      pass(milliseconds);
      const due = [...frames.values()];
      frames.clear();
      for (const callback of due) callback(time);
    },
  };
}

/** 🎪️ A stage that only counts: it keeps the ticks of every step and answers with the pace the case sets. */
function stage(rate: Pace["rate"], wake: number | null = null): { readonly steps: number[]; readonly step: (ticks: number) => Pace; tick: number; rate: Pace["rate"]; wake: number | null } {
  const counted = {
    steps: [] as number[],
    tick: 0,
    rate,
    wake,
    step: (ticks: number): Pace => {
      counted.steps.push(ticks);
      counted.tick += ticks;
      return { tick: counted.tick, rate: counted.rate, wake: counted.wake };
    },
  };
  return counted;
}

const sum = (values: readonly number[]): number => values.reduce((total, value) => total + value, 0);

afterEach(() => {
  vi.useRealTimers();
  vi.restoreAllMocks();
});

describe("⏲️ frame pacing", () => {
  it("counts sixty-four ticks to the second, at most eight to the frame", () => {
    expect(TICK_MILLISECONDS).toBe(15.625);
    expect(TICK_MILLISECONDS * TICKS_PER_SECOND).toBe(1000);
    expect(FRAME_TICKS).toBe(8);
  });

  it("asks for nothing until it is woken, and for one frame however often", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    time.pass(5000);
    expect([time.frames.size, time.timers.size, pets.steps]).toEqual([0, 0, []]);
    pacer.wake();
    pacer.wake();
    pacer.wake();
    expect([time.frames.size, time.asked.frames]).toEqual([1, 1]);
    pacer.stop();
  });

  it("steps with every frame at rate 64 and hands out exactly the ticks that passed", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    for (let frame = 0; frame < 250; frame++) time.frame(16);
    expect(pets.steps).toHaveLength(250);
    expect(sum(pets.steps)).toBe(256);
    expect(new Set(pets.steps)).toEqual(new Set([1, 2]));
    expect(time.frames.size).toBe(1);
    expect(time.timers.size).toBe(0);
    pacer.stop();
  });

  it("steps only when a tick is due on a display faster than the stage", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    time.frame(8);
    expect(pets.steps).toEqual([0]);
    for (let frame = 1; frame < 500; frame++) time.frame(8);
    expect(sum(pets.steps)).toBe(256);
    expect(pets.steps.slice(1).every((ticks) => ticks === 1)).toBe(true);
    expect(pets.steps).toHaveLength(257);
    expect(time.asked.frames).toBe(501);
    pacer.stop();
  });

  it("drops what a late frame owes beyond eight ticks instead of catching up", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    time.frame(16);
    time.frame(500);
    time.frame(16);
    time.frame(16);
    expect(pets.steps).toEqual([1, FRAME_TICKS, 1, 1]);
    pacer.stop();
  });

  it("skips every other frame at rate 32 and three of four at rate 16", () => {
    for (const [rate, stride] of [[32, 2], [16, 4]] as const) {
      const time = clock();
      const pets = stage(rate);
      const pacer = createPacer(pets.step, time.seams);
      pacer.wake();
      time.frame(16);
      expect(pets.steps, `rate ${rate}`).toEqual([1]);
      for (let frame = 1; frame < 250; frame++) time.frame(16);
      expect(sum(pets.steps), `rate ${rate}`).toBe(256);
      expect(pets.steps.slice(1).every((ticks) => ticks >= stride && ticks <= stride + 1), `rate ${rate}`).toBe(true);
      expect(pets.steps.length, `rate ${rate}`).toBeGreaterThanOrEqual(256 / (stride + 1));
      expect(pets.steps.length, `rate ${rate}`).toBeLessThanOrEqual(256 / stride + 1);
      expect(time.asked.frames, `rate ${rate}`).toBe(251);
      pacer.stop();
    }
  });

  it("steps at once when something happened, however calm the stage", () => {
    const time = clock();
    const pets = stage(16);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    time.frame(16);
    time.frame(16);
    expect(pets.steps).toEqual([1]);
    pacer.wake();
    time.frame(16);
    expect(pets.steps).toEqual([1, 2]);
    time.frame(16);
    time.frame(16);
    expect(pets.steps).toEqual([1, 2]);
    pacer.stop();
  });

  it("follows the stage when its rate changes", () => {
    const time = clock();
    const pets = stage(64);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    for (let frame = 0; frame < 10; frame++) time.frame(16);
    expect(pets.steps).toHaveLength(10);
    pets.rate = 32;
    for (let frame = 0; frame < 10; frame++) time.frame(16);
    expect(pets.steps).toHaveLength(15);
    pets.rate = 0;
    time.frame(16);
    time.frame(16);
    const rested = pets.steps.length;
    for (let frame = 0; frame < 10; frame++) time.frame(16);
    expect(pets.steps).toHaveLength(rested);
    expect([time.frames.size, time.timers.size]).toEqual([0, 0]);
    pets.rate = 64;
    pacer.wake();
    time.frame(16);
    time.frame(16);
    expect(pets.steps.length).toBe(rested + 2);
    pacer.stop();
  });

  it("requests no frame at rate 0 and arms exactly one timer for the wake tick", () => {
    const time = clock();
    const pets = stage(0, 193);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    time.frame(16);
    expect(pets.steps).toEqual([1]);
    expect([time.frames.size, time.timers.size]).toEqual([0, 1]);
    expect([...time.timers.values()][0]!.delay).toBe(192 * TICK_MILLISECONDS - 0.375);
    for (let frame = 0; frame < 60; frame++) time.frame(16);
    expect([time.asked.frames, time.asked.timers, pets.steps.length]).toEqual([1, 1, 1]);

    pacer.wake();
    expect([time.frames.size, time.timers.size]).toEqual([1, 1]);
    time.frame(16);
    expect(pets.steps).toEqual([1, 62]);
    expect([time.frames.size, time.timers.size, time.asked.timers]).toEqual([0, 1, 2]);
    expect([...time.timers.values()][0]!.delay).toBe((193 - 63) * TICK_MILLISECONDS - 7.625);

    pets.wake = 1000;
    time.pass(2100);
    expect([time.frames.size, time.timers.size]).toEqual([1, 0]);
    time.frame(16);
    expect(pets.steps).toEqual([1, 62, 135]);
    expect(pets.tick).toBeGreaterThanOrEqual(193);
    expect([time.frames.size, time.timers.size]).toEqual([0, 1]);
    pacer.stop();
    expect([time.frames.size, time.timers.size]).toEqual([0, 0]);
  });

  it("schedules nothing at all while the stage rests without a wake tick", () => {
    const time = clock();
    const pets = stage(0, null);
    const pacer = createPacer(pets.step, time.seams);
    pacer.wake();
    time.frame(16);
    expect(pets.steps).toEqual([1]);
    time.pass(600_000);
    expect([time.frames.size, time.timers.size, time.asked.frames, time.asked.timers]).toEqual([0, 0, 1, 0]);
    pacer.wake();
    time.frame(16);
    expect(pets.steps).toEqual([1, 38_401]);
    expect([time.frames.size, time.timers.size]).toEqual([0, 0]);
    pacer.stop();
  });

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
});
