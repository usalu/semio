/** ⏱️ The clock is deterministic: elapsed-time accumulation, exact frame stepping, loops, reduced motion; two figures on one clock show one instant.
 * @see ./🥒️.feature — the scenarios */
import { act, cleanup, render } from "@testing-library/react";
import { afterEach, describe, expect, it } from "vitest";
import { StaffageClock, StaffageFigure } from "@teaching/staffage-react";
import type { ClockScheduler } from "@teaching/staffage-react";
import { walker } from "../🧰️support/🧰️.ts";

function manual(): ClockScheduler & { tick: (nowMs: number) => void } {
  let next: ((nowMs: number) => void) | null = null;
  return {
    request: (tick) => ((next = tick), 1),
    cancel: () => void (next = null),
    tick: (nowMs) => {
      const run = next;
      next = null;
      run?.(nowMs);
    },
  };
}

afterEach(cleanup);

describe("⏱️ clock", () => {
  it("accumulates elapsed time times rate and wraps loops", () => {
    const scheduler = manual();
    const clock = new StaffageClock({ scheduler, fps: 30, durationS: 1, playing: true, rate: 2, reducedMotion: false });
    scheduler.tick(1000);
    scheduler.tick(1250);
    expect(clock.timeS).toBeCloseTo(0.5, 12);
    scheduler.tick(1500);
    expect(clock.timeS).toBeCloseTo(0, 9);
    scheduler.tick(1750);
    expect(clock.timeS).toBeCloseTo(0.5, 9);
  });

  it("steps and seeks onto exact frames and clamps non-looping clips", () => {
    const clock = new StaffageClock({ fps: 30, durationS: 1, loop: false, playing: false, reducedMotion: false });
    clock.step(3);
    expect(clock.timeS).toBe(3 / 30);
    clock.step(-5);
    expect(clock.timeS).toBe(0);
    clock.seek(7.5);
    expect(clock.timeS).toBe(1);
    clock.setLoop(true);
    clock.seek(1.25);
    expect(clock.timeS).toBeCloseTo(0.25, 12);
    clock.dispose();
  });

  it("replays the same wall-clock trace to the same state", () => {
    const run = () => {
      const scheduler = manual();
      const clock = new StaffageClock({ scheduler, fps: 24, durationS: 4, playing: true, reducedMotion: false });
      for (const now of [0, 16.7, 33.4, 50, 120, 121, 500]) scheduler.tick(now);
      return clock.state;
    };
    expect(run()).toEqual(run());
  });

  it("starts paused under reduced motion and plays only when asked", () => {
    const clock = new StaffageClock({ scheduler: manual(), reducedMotion: true });
    expect(clock.state.playing).toBe(false);
    clock.play();
    expect(clock.state.playing).toBe(true);
  });

  it("renders two figures on one clock at the identical instant", () => {
    const clock = new StaffageClock({ fps: 30, durationS: 1, playing: false, reducedMotion: false });
    const props = { asset: walker, clock, scale: { denominator: 100 }, language: "en" as const, live: true };
    const { container } = render(
      <>
        <StaffageFigure {...props} view="dark-line" />
        <StaffageFigure {...props} view="light-solid" />
      </>,
    );
    const times = () => [...container.querySelectorAll("[data-staffage-figure]")].map((figure) => figure.getAttribute("data-time"));
    expect(new Set(times()).size).toBe(1);
    act(() => clock.seek(10 / 30));
    expect(new Set(times()).size).toBe(1);
    expect(Number(times()[0])).toBeCloseTo(10 / 30, 9);
    act(() => clock.step(5));
    expect(Number(times()[1])).toBeCloseTo(15 / 30, 9);
  });
});
