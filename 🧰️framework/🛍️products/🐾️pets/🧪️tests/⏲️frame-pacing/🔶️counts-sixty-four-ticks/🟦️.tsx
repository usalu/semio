/** 🔶️ The counts sixty four ticks tests of the ⏲️ frame pacing suite. */
import { expect, it } from "vitest";
import { TICKS_PER_SECOND } from "@semio-tech/pets";
import { FRAME_TICKS, TICK_MILLISECONDS, createPacer } from "@semio-tech/pets-react";
import { clock, stage, sum } from "../🟠️shared/🟦️.tsx";

/** 🔷 Registers the counts sixty four ticks tests of the ⏲️ frame pacing suite. */
export function registerCountsSixtyFourTicksTests(): void {
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
}
