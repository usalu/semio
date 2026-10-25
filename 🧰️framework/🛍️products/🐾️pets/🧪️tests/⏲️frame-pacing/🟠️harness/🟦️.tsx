/** 🟠️ The shared declarations of the ⏲️ frame pacing suite tests. */
import { type Pace, type PacerSeams } from "@semio-tech/pets-react";

/** 🕰️ A clock moved by hand: frames and timers wait in ledgers until the case lets time pass. */
export function clock(): {
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
export function stage(rate: Pace["rate"], wake: number | null = null): { readonly steps: number[]; readonly step: (ticks: number) => Pace; tick: number; rate: Pace["rate"]; wake: number | null } {
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

export const sum = (values: readonly number[]): number => values.reduce((total, value) => total + value, 0);
