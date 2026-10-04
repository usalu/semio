/** 🪢️ Unit suite of the swing module: the constrained step against its own invariants, the committed answers of scipy and numpy, gl-matrix's rotations and the drags of MECH §1.3; the follow spring, the cone, the release, the reel and the parachute; the ban on platform transcendentals.
 *
 * @see ../../🟦️.ts — the module under test
 * @see ../../../../🧫️fixtures/🪢️swing-dynamics/🔣️.json — the restated swings scipy and numpy confirmed (case 🪢️swing-dynamics)
 * @see ../../../../🧫️fixtures/🪂️parachute-descent/🔣️.json — the restated descents scipy and numpy confirmed (case 🪂️parachute-descent)
 * @see https://glmatrix.net/docs/module-mat2d.html — `mat2d.fromRotation`, the JavaScript oracle of the lean
 */
import { mat2d, vec2 } from "gl-matrix";
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Grip, Hang, Point } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { randomUnit } from "../../../🎲️randomness/🟦️.ts";
import { FALL_SPEED, GRAVITY } from "../../../🏞️terrain/🟦️.ts";
import {
  canopyOf,
  CHUTE_FACTOR,
  CHUTE_GRAVITY,
  CHUTE_HEADROOM,
  CHUTE_OPENING,
  CHUTE_REFLEX,
  CHUTE_WIND,
  chuteOf,
  chuteOpens,
  chuteStep,
  chuteWind,
  coneClamp,
  flareOf,
  FOLLOW_DAMPING,
  FOLLOW_STIFFNESS,
  followStep,
  HANG_CONE,
  HANG_GRAVITY,
  HANG_ROD,
  hangOf,
  hangStep,
  HARD_LANDING,
  impactSpeed,
  leanOf,
  REEL_CAP,
  REEL_SPEED,
  reelStep,
  RELEASE_DIVISOR,
  RELEASE_WEIGHTS,
  releaseVelocity,
  ringVelocity,
  swingStep,
  THROW_LEAST,
  THROW_MOST,
  THROW_RISE,
  throwOf,
  throwVelocity,
} from "../../🟦️.ts";

type Swing = { readonly id: string; readonly anchor: Point; readonly length: number; readonly gravity: number; readonly bob: Point; readonly previous: Point };
type Swings = {
  readonly constants: Record<string, number | readonly number[]>;
  readonly rods: readonly (Swing & { readonly damping: number; readonly ticks: readonly number[]; readonly expected: readonly Point[] })[];
  readonly anchors: readonly { readonly id: string; readonly length: number; readonly gravity: number; readonly path: readonly Point[]; readonly ticks: readonly number[]; readonly expected: readonly Point[] }[];
  readonly cones: readonly { readonly id: string; readonly anchor: Point; readonly bob: Point; readonly length: number; readonly expected: Point }[];
  readonly follows: readonly { readonly id: string; readonly grip: Grip; readonly target: Point; readonly ticks: readonly number[]; readonly expected: readonly Grip[] }[];
  readonly drags: readonly { readonly id: string; readonly length: number; readonly feet: Point; readonly pointer: readonly Point[]; readonly ticks: readonly number[]; readonly expected: { readonly leans: readonly number[]; readonly peak: number; readonly settled: number } }[];
  readonly releases: readonly { readonly id: string; readonly samples: readonly Point[]; readonly expected: { readonly ring: Point; readonly release: Point } }[];
  readonly throws: readonly { readonly id: string; readonly hang: Hang; readonly samples: readonly Point[]; readonly expected: Point }[];
  readonly guards: readonly { readonly id: string; readonly anchor: Point; readonly bob: Point; readonly previous: Point; readonly length: number; readonly least: number; readonly ticks: readonly number[]; readonly expected: { readonly states: readonly { readonly x: number; readonly y: number; readonly length: number }[]; readonly fastest: number } }[];
  readonly steps: readonly { readonly id: string; readonly before: Point; readonly now: Point; readonly bob: Point; readonly previous: Point; readonly length: number; readonly gravity: number; readonly damping: number; readonly rope: boolean; readonly expected: { readonly x: string; readonly y: string } }[];
};
type Descents = {
  readonly impacts: readonly { readonly id: string; readonly vy: number; readonly height: number; readonly expected: number }[];
  readonly triggers: readonly { readonly id: string; readonly vy: number; readonly height: number; readonly expected: boolean }[];
  readonly descents: readonly { readonly id: string; readonly height: number; readonly feet: Point; readonly vx: number; readonly vy: number; readonly target: number; readonly remaining: number; readonly phase: number | null; readonly ticks: readonly number[]; readonly expected: readonly { readonly x: number; readonly y: number; readonly vx: number; readonly vy: number; readonly bob: Point }[] }[];
  readonly drops: readonly { readonly id: string; readonly drop: number; readonly vy: number; readonly chute: boolean; readonly expected: { readonly opened: number; readonly landed: number; readonly touch: number } }[];
};

const SWINGS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🪢️swing-dynamics/🔣️.json", import.meta.url), "utf8")) as Swings;
const DESCENTS = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🪂️parachute-descent/🔣️.json", import.meta.url), "utf8")) as Descents;
const SOURCE = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
const ANCHOR = { x: 320, y: 120 };
const VIEW = new DataView(new ArrayBuffer(8));

/** 🎲️ How many random states the invariants of a step are tried on at the level of the run. */
const STATES = sampled(300, 3000, 30000);

/** 🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits. */
function bits(value: number): string {
  VIEW.setFloat64(0, value);
  return VIEW.getBigUint64(0).toString(16).padStart(16, "0");
}

/** 🎰️ A number in [low, high) drawn from the counter-based stream of this suite. */
function drawn(index: number, slot: number, low: number, high: number): number {
  return low + (high - low) * randomUnit([20261003, index, slot]);
}

/** 📏️ How far a point lies from another. */
function distance(from: Point, to: Point): number {
  return Math.sqrt((to.x - from.x) * (to.x - from.x) + (to.y - from.y) * (to.y - from.y));
}

/** 📐️ The angle in degrees of a point below its anchor, from straight down towards the positive x axis. */
function degrees(anchor: Point, bob: Point): number {
  return (Math.atan2(bob.x - anchor.x, bob.y - anchor.y) * 180) / Math.PI;
}

/** 🏃️ The positions of a point on a fixed rod after each of `ticks` ticks, with the step given. */
function swung(bob: Point, previous: Point, ticks: number, step: (bob: Point, previous: Point) => Point): Point[] {
  const path: Point[] = [];
  let now = bob;
  let before = previous;
  for (let tick = 0; tick < ticks; tick++) {
    const next = step(now, before);
    before = now;
    now = next;
    path.push(now);
  }
  return path;
}

/** ⛰️ The largest angle in degrees a path reaches within its last `window` ticks. */
function amplitude(path: readonly Point[], anchor: Point, window: number): number {
  let widest = 0;
  for (const bob of path.slice(path.length - window)) widest = Math.max(widest, Math.abs(degrees(anchor, bob)));
  return widest;
}

describe("swingStep", () => {
  it("leaves a point that hangs at rest where it is", () => {
    const bob = { x: 320, y: 220 };
    for (let tick = 0, now = bob; tick < 64; tick++) {
      now = swingStep(ANCHOR, ANCHOR, now, now, 100, GRAVITY, 1, false);
      expect(Math.abs(now.x - bob.x) + Math.abs(now.y - bob.y)).toBeLessThanOrEqual(1e-12);
    }
  });

  it("puts the point exactly at its rod's length from an anchor that moved, whenever the move and the step are shorter than the rod", () => {
    for (let index = 0; index < STATES; index++) {
      const before = { x: drawn(index, 0, 0, 800), y: drawn(index, 1, 0, 600) };
      const length = drawn(index, 2, 20, 160);
      const angle = drawn(index, 3, -3, 3);
      const bob = { x: before.x + length * Math.sin(angle), y: before.y + length * Math.cos(angle) };
      const previous = { x: bob.x + drawn(index, 4, -3, 3), y: bob.y + drawn(index, 5, -3, 3) };
      const now = { x: before.x + drawn(index, 6, -2, 2), y: before.y + drawn(index, 7, -2, 2) };
      const next = swingStep(before, now, bob, previous, length, drawn(index, 8, 0, 7200), drawn(index, 9, 0.9, 1), false);
      expect(Math.abs(distance(now, next) - length), `${index}`).toBeLessThanOrEqual(1e-9 * length);
    }
  });

  it("lets a point on a slack rope fly freely and holds it on a taut one", () => {
    const bob = { x: 320, y: 180 };
    const previous = { x: 320 - 120 / 64, y: 180 + 300 / 64 };
    const slack = swingStep(ANCHOR, ANCHOR, bob, previous, 100, GRAVITY, 1, true);
    expect(slack).toEqual({ x: bob.x + (bob.x - previous.x), y: bob.y + (bob.y - previous.y) + GRAVITY / 4096 });
    const taut = swingStep(ANCHOR, ANCHOR, { x: 380, y: 200 }, { x: 378.5, y: 198.25 }, 100, GRAVITY, 1, true);
    expect(Math.abs(distance(ANCHOR, taut) - 100)).toBeLessThanOrEqual(1e-12);
  });

  it("reproduces every committed swing, every swing below a moving anchor and every committed bit pattern", () => {
    for (const vector of SWINGS.rods) {
      const path = swung(vector.bob, vector.previous, Math.max(...vector.ticks), (bob, previous) => swingStep(vector.anchor, vector.anchor, bob, previous, vector.length, vector.gravity, vector.damping, false));
      vector.ticks.forEach((tick, index) => expect(path[tick - 1], vector.id).toEqual(vector.expected[index]));
    }
    for (const vector of SWINGS.anchors) {
      let bob = { x: vector.path[0]!.x, y: vector.path[0]!.y + vector.length };
      let previous = bob;
      const path: Point[] = [];
      for (let tick = 1; tick < vector.path.length; tick++) {
        const next = swingStep(vector.path[tick - 1]!, vector.path[tick]!, bob, previous, vector.length, vector.gravity, 1, false);
        previous = bob;
        bob = next;
        path.push(bob);
      }
      vector.ticks.forEach((tick, index) => expect(path[tick - 1], vector.id).toEqual(vector.expected[index]));
    }
    for (const vector of SWINGS.steps) {
      const next = swingStep(vector.before, vector.now, vector.bob, vector.previous, vector.length, vector.gravity, vector.damping, vector.rope);
      expect({ x: bits(next.x), y: bits(next.y) }, vector.id).toEqual(vector.expected);
    }
  });

  it("keeps the amplitude of a free swing for a minute, where the shortcut that pulls the point back onto the circle loses it", () => {
    const length = 100;
    const start = { x: ANCHOR.x + length * Math.SQRT1_2, y: ANCHOR.y + length * Math.SQRT1_2 };
    const exact = SWINGS.rods.find((vector) => vector.id === "gentle")!;
    const kept = swung(exact.bob, exact.previous, 3840, (bob, previous) => swingStep(ANCHOR, ANCHOR, bob, previous, length, GRAVITY, 1, false));
    const shortcut = swung(start, start, 3840, (bob, previous) => {
      const free = { x: bob.x + (bob.x - previous.x), y: bob.y + (bob.y - previous.y) + GRAVITY / 4096 };
      const reach = distance(ANCHOR, free);
      return { x: ANCHOR.x + ((free.x - ANCHOR.x) * length) / reach, y: ANCHOR.y + ((free.y - ANCHOR.y) * length) / reach };
    });
    expect(Math.abs(amplitude(kept, ANCHOR, 128) - 45)).toBeLessThanOrEqual(0.05);
    expect(amplitude(shortcut, ANCHOR, 128)).toBeLessThan(35);
  });

  it("keeps the angular momentum about the anchor exactly while the rope shortens without gravity", () => {
    let bob = { x: 420, y: 120 };
    let previous = { x: 420, y: 118 };
    const momentum = (from: Point, to: Point): number => (from.x - ANCHOR.x) * (to.y - ANCHOR.y) - (from.y - ANCHOR.y) * (to.x - ANCHOR.x);
    const first = momentum(previous, bob);
    for (let tick = 1; tick <= 96; tick++) {
      const next = swingStep(ANCHOR, ANCHOR, bob, previous, 100 - (60 * tick) / 64, 0, 1, false);
      previous = bob;
      bob = next;
      expect(Math.abs(momentum(previous, bob) - first)).toBeLessThanOrEqual(1e-9 * Math.abs(first));
    }
  });

  it("carries a hanging point along with an anchor in uniform motion without a swing", () => {
    const drift = { x: 3.25, y: -1.5 };
    let anchor = { x: 100, y: 100 };
    let bob = { x: 100, y: 140 };
    let previous = { x: bob.x - drift.x, y: bob.y - drift.y };
    for (let tick = 0; tick < 128; tick++) {
      const next = { x: anchor.x + drift.x, y: anchor.y + drift.y };
      const moved = swingStep(anchor, next, bob, previous, 40, GRAVITY, 1, false);
      previous = bob;
      bob = moved;
      anchor = next;
      expect(Math.abs(bob.x - anchor.x)).toBeLessThanOrEqual(1e-9);
      expect(Math.abs(bob.y - anchor.y - 40)).toBeLessThanOrEqual(1e-9);
    }
  });
});

describe("the hand: coneClamp, followStep, hangStep, leanOf", () => {
  it("keeps a held pet on its rod and inside the cone, on its own side, and leaves every other point alone", () => {
    for (const vector of SWINGS.cones) expect(coneClamp(vector.anchor, vector.bob, vector.length), vector.id).toEqual(vector.expected);
    const grip = { x: 0, y: 0 };
    for (let step = -180; step <= 180; step += 5) {
      const turned = { x: 50 * Math.sin((step * Math.PI) / 180), y: 50 * Math.cos((step * Math.PI) / 180) };
      const held = coneClamp(grip, turned, 40);
      expect(Math.abs(distance(grip, held) - 40)).toBeLessThanOrEqual(1e-9);
      expect(held.y).toBeGreaterThanOrEqual(HANG_CONE * 40 - 1e-9);
      if (turned.x !== 0 && Math.abs(step) < 180) expect(Math.sign(held.x)).toBe(Math.sign(turned.x));
    }
    const inside = { x: 5, y: 20 };
    expect(coneClamp(grip, inside, 40)).toBe(inside);
  });

  it("carries the grip to the pointer without overshoot, half way in 4 ticks, 70 ms behind a pointer in steady motion", () => {
    for (const vector of SWINGS.follows) {
      let grip = vector.grip;
      let reached = 0;
      vector.ticks.forEach((ticks, index) => {
        for (; reached < ticks; reached++) grip = followStep(grip, vector.target);
        expect(grip, vector.id).toEqual(vector.expected[index]);
      });
    }
    let grip: Grip = { x: 0, y: 0, vx: 0, vy: 0 };
    for (let tick = 1; tick <= 64; tick++) {
      grip = followStep(grip, { x: 100, y: -50 });
      expect(grip.x).toBeLessThanOrEqual(100);
      expect(grip.y).toBeGreaterThanOrEqual(-50);
      if (tick === 3) expect(grip.x).toBeLessThan(50);
      if (tick === 4) expect(grip.x).toBeGreaterThanOrEqual(50);
    }
    grip = { x: 0, y: 0, vx: 0, vy: 0 };
    for (let tick = 1; tick <= 512; tick++) grip = followStep(grip, { x: 10 * tick, y: 0 });
    expect(Math.abs((10 * 512 - grip.x) / 640 - (FOLLOW_DAMPING / FOLLOW_STIFFNESS - 1 / 64))).toBeLessThanOrEqual(1e-9);
  });

  it("reproduces every committed drag and leans as MECH §1.3 measured: 40° to 51° for a short drag, the cone's 60° for a long drag and a flick, 14° to 24° for a slow drag, settled within 2.5 s", () => {
    const peaks: Record<string, number> = {};
    for (const vector of SWINGS.drags) {
      let hang = hangOf(vector.feet, vector.length);
      const leans: number[] = [];
      let peak = 0;
      for (const target of vector.pointer) {
        hang = hangStep(hang, target, vector.length);
        expect(Math.abs(distance(hang.grip, hang.bob) - vector.length)).toBeLessThanOrEqual(1e-6 * vector.length);
        const lean = leanOf(hang.grip, hang.bob, vector.length);
        leans.push(lean);
        peak = Math.max(peak, Math.abs(lean));
      }
      vector.ticks.forEach((tick, index) => expect(leans[tick - 1], vector.id).toBe(vector.expected.leans[index]));
      expect(peak, vector.id).toBe(vector.expected.peak);
      expect(vector.expected.settled / 64, vector.id).toBeLessThan(2.5);
      peaks[vector.id] = peak * 360;
    }
    expect(peaks["short-drag"]).toBeGreaterThanOrEqual(40);
    expect(peaks["short-drag"]).toBeLessThanOrEqual(51);
    expect(Math.abs(peaks["short-drag-to-the-left"]! - peaks["short-drag"]!)).toBeLessThanOrEqual(1e-9);
    expect(Math.abs(peaks["long-drag"]! - 60)).toBeLessThan(0.001);
    expect(Math.abs(peaks["flick"]! - 60)).toBeLessThan(0.001);
    expect(peaks["slow-drag"]).toBeGreaterThanOrEqual(14);
    expect(peaks["slow-drag"]).toBeLessThanOrEqual(24);
  });

  it("gives the rotation that carries a body hanging straight down onto its feet, as gl-matrix rotates it", () => {
    const rotation = mat2d.create();
    const carried = vec2.create();
    for (let step = -60; step <= 60; step += 3) {
      const bob = { x: 200 + 38.4 * Math.sin((step * Math.PI) / 180), y: 100 + 38.4 * Math.cos((step * Math.PI) / 180) };
      const lean = leanOf({ x: 200, y: 100 }, bob, 38.4);
      mat2d.fromRotation(rotation, 2 * Math.PI * lean);
      vec2.transformMat2d(carried, [0, 38.4], rotation);
      expect(Math.abs(carried[0]! - (bob.x - 200))).toBeLessThanOrEqual(38.4 * 2 * Math.PI * 2e-6);
      expect(Math.abs(carried[1]! - (bob.y - 100))).toBeLessThanOrEqual(38.4 * 2 * Math.PI * 2e-6);
      if (step > 0) expect(lean).toBeLessThan(0);
    }
    expect(leanOf({ x: 7, y: 9 }, { x: 7, y: 47.4 }, 38.4)).toBe(0);
  });

  it("picks a pet up at rest with its grip a rod above its feet", () => {
    const hang = hangOf({ x: 300, y: 400 }, HANG_ROD * 48);
    expect(hang).toEqual({ grip: { x: 300, y: 400 - HANG_ROD * 48, vx: 0, vy: 0 }, bob: { x: 300, y: 400 }, previous: { x: 300, y: 400 } });
    const rested = hangStep(hang, { x: 300, y: 400 - HANG_ROD * 48 }, HANG_ROD * 48);
    expect(rested.grip).toEqual(hang.grip);
    expect(rested.previous).toBe(hang.bob);
    expect(distance(rested.bob, hang.bob)).toBeLessThanOrEqual(1e-12);
  });
});

describe("the release: ringVelocity, releaseVelocity, throwVelocity", () => {
  it("weighs the last seven samples with weights that add up to nothing and fit a parabola exactly", () => {
    expect(RELEASE_WEIGHTS.reduce((sum, weight) => sum + weight, 0)).toBe(0);
    for (const [velocity, thrust] of [
      [3, 0],
      [-7.5, 0.25],
      [12, -0.75],
    ] as const) {
      const samples = Array.from({ length: 7 }, (_, tick) => ({ x: 400 + velocity * tick + thrust * tick * tick, y: 50 - velocity * tick }));
      expect(ringVelocity(samples).x).toBeCloseTo((velocity + 2 * thrust * 6) * 64, 9);
      expect(ringVelocity(samples).y).toBeCloseTo(-velocity * 64, 9);
    }
    const resting = Array.from({ length: 7 }, () => ({ x: 123.456, y: -98.7654321 }));
    expect(ringVelocity(resting)).toEqual({ x: 0, y: 0 });
    expect(ringVelocity([])).toEqual({ x: 0, y: 0 });
    expect(RELEASE_DIVISOR).toBe(28);
  });

  it("reproduces every committed release and throw", () => {
    for (const vector of SWINGS.releases) {
      expect(ringVelocity(vector.samples), vector.id).toEqual(vector.expected.ring);
      expect(releaseVelocity(vector.samples), vector.id).toEqual(vector.expected.release);
    }
    for (const vector of SWINGS.throws) expect(throwVelocity(vector.hang, vector.samples), vector.id).toEqual(vector.expected);
  });

  it("throws nothing below the least speed, at most the most speed along the same direction, and never rises faster than allowed", () => {
    expect(throwOf({ x: 60, y: 30 })).toEqual({ x: 0, y: 0 });
    expect(throwOf({ x: 70, y: 0 })).toEqual({ x: 70, y: 0 });
    const fast = throwOf({ x: 1200, y: 500 });
    expect(Math.abs(Math.sqrt(fast.x * fast.x + fast.y * fast.y) - THROW_MOST)).toBeLessThanOrEqual(1e-9);
    expect(Math.abs(fast.y / fast.x - 500 / 1200)).toBeLessThanOrEqual(1e-12);
    expect(throwOf({ x: 0, y: -630 })).toEqual({ x: 0, y: -THROW_RISE });
    expect(throwOf({ x: 0, y: 630 })).toEqual({ x: 0, y: 630 });
    expect(THROW_LEAST).toBeLessThan(THROW_RISE);
    const stopped = [0, 5, 15, 30, 40, 40, 40].map((x) => ({ x, y: 0 }));
    expect(releaseVelocity(stopped)).toEqual({ x: 0, y: 0 });
    expect(ringVelocity(stopped).x).toBe((45 * 64) / 28);
  });
});

describe("the reel: reelStep", () => {
  it("reproduces every committed reeled swing", () => {
    for (const vector of SWINGS.guards) {
      let bob = vector.bob;
      let previous = vector.previous;
      let length = vector.length;
      let fastest = 0;
      const states: { x: number; y: number; length: number }[] = [];
      for (let age = 0; age < Math.max(...vector.ticks); age++) {
        const reel = reelStep(vector.anchor, bob, previous, length, vector.least, age);
        fastest = Math.max(fastest, distance(bob, reel.bob) * 64);
        previous = bob;
        bob = reel.bob;
        length = reel.length;
        states.push({ x: bob.x, y: bob.y, length });
      }
      vector.ticks.forEach((tick, index) => expect(states[tick - 1], vector.id).toEqual(vector.expected.states[index]));
      expect(fastest, vector.id).toBe(vector.expected.fastest);
    }
  });

  it("winds the rope in at the ramped speed down to its least, never lengthens it, and keeps the pet within the cap plus what the reel takes in", () => {
    let bob = { x: ANCHOR.x + 154 * Math.sin(1.4), y: ANCHOR.y + 154 * Math.cos(1.4) };
    let previous = bob;
    let length = 154;
    for (let age = 0; age < 400; age++) {
      const reel = reelStep(ANCHOR, bob, previous, length, 43.2, age);
      const wound = Math.min(age + 1, 10) * (REEL_SPEED / 10 / 64);
      expect(reel.length).toBe(Math.max(43.2, length - wound));
      expect(distance(bob, reel.bob) * 64).toBeLessThanOrEqual(REEL_CAP + REEL_SPEED);
      expect(distance(ANCHOR, reel.bob)).toBeLessThanOrEqual(reel.length + 1e-9);
      previous = bob;
      bob = reel.bob;
      length = reel.length;
    }
    expect(length).toBe(43.2);
    expect(reelStep(ANCHOR, { x: 320, y: 160 }, { x: 320, y: 160 }, 40, 43.2, 0).length).toBe(40);
  });
});

describe("the parachute", () => {
  it("predicts the impact of a fall by its closed form, never beyond the terminal speed, and reproduces every committed prediction", () => {
    expect(impactSpeed(0, 100)).toBe(HARD_LANDING);
    expect(impactSpeed(0, 1000)).toBe(FALL_SPEED);
    expect(impactSpeed(-250, -5)).toBe(250);
    for (const vector of DESCENTS.impacts) expect(impactSpeed(vector.vy, vector.height), vector.id).toBe(vector.expected);
  });

  it("opens only for a fast fall from high enough that would land hard, and from rest for every drop of 102 px or more", () => {
    for (const vector of DESCENTS.triggers) expect(chuteOpens(vector.vy, vector.height), vector.id).toBe(vector.expected);
    expect(chuteOpens(CHUTE_OPENING, CHUTE_HEADROOM)).toBe(false);
    expect(chuteOpens(CHUTE_OPENING, 85)).toBe(true);
    for (const drop of [101, 102]) {
      let y = 0;
      let vy = 0;
      let opened = false;
      while (y < drop && !opened) {
        opened = chuteOpens(vy, drop - y);
        vy = Math.min(vy + GRAVITY / 64, FALL_SPEED);
        y = y + vy / 64;
      }
      expect(opened, `${drop}`).toBe(drop >= 102);
    }
  });

  it("approaches the terminal speed by the exact factor of the lag and reproduces every committed descent", () => {
    expect(Math.abs(CHUTE_FACTOR - (1 - 1 / 11.52 + 1 / (2 * 11.52 * 11.52) - 1 / (6 * 11.52 ** 3) + 1 / (24 * 11.52 ** 4)))).toBeLessThan(1e-6);
    for (const vector of DESCENTS.descents) {
      const chute = chuteOf(vector.height);
      let canopy = canopyOf(vector.feet, vector.vx, vector.vy, chute);
      const states: { x: number; y: number; vx: number; vy: number; bob: Point }[] = [];
      for (let tick = 1; tick <= Math.max(...vector.ticks); tick++) {
        canopy = chuteStep(canopy, chute, vector.target, vector.remaining - (canopy.bob.y - vector.feet.y), vector.phase === null ? 0 : chuteWind(tick, vector.phase));
        states.push({ x: canopy.x, y: canopy.y, vx: canopy.vx, vy: canopy.vy, bob: canopy.bob });
        expect(Math.abs(distance(canopy, canopy.bob) - chute.length)).toBeLessThanOrEqual(1e-9 * chute.length);
      }
      vector.ticks.forEach((tick, index) => expect(states[tick - 1], vector.id).toEqual(vector.expected[index]));
    }
  });

  it("flares evenly to half of its speed, blows a wind of at most its strength, and sways under half the gravity of a fall", () => {
    expect(flareOf(12, 12)).toBe(1);
    expect(flareOf(6, 12)).toBe(0.75);
    expect(flareOf(0, 12)).toBe(0.5);
    expect(flareOf(-4, 12)).toBe(0.5);
    for (let tick = 0; tick < 2000; tick += 7) expect(Math.abs(chuteWind(tick, 0.3))).toBeLessThanOrEqual(CHUTE_WIND);
    expect(CHUTE_GRAVITY).toBe(GRAVITY / 2);
    expect(HANG_GRAVITY).toBe(4 * GRAVITY);
  });

  it("lands every committed drop as committed, softly with a parachute: under 120 px/s from 150 px and under 60 px/s from 200 px", () => {
    for (const vector of DESCENTS.drops) expect(vector.expected.touch, vector.id).toBeGreaterThan(0);
    const touch = (id: string): number => DESCENTS.drops.find((vector) => vector.id === id)!.expected.touch;
    expect(touch("from-150")).toBeLessThan(120);
    expect(touch("from-200")).toBeLessThan(60);
    expect(touch("from-150-without-a-parachute")).toBeGreaterThan(HARD_LANDING);
    expect(DESCENTS.drops.find((vector) => vector.id === "from-102")!.expected.opened).toBe(10);
    expect(CHUTE_REFLEX).toBe(6);
  });
});

describe("determinism", () => {
  it("the module calls no platform transcendental, random source or clock", () => {
    const banned = ["sin", "cos", "tan", "atan2", "exp", "pow", "hypot", "log", "random"].map((name) => `Math.${name}(`).concat(["Date", "performance"]);
    for (const call of banned) expect(SOURCE.includes(call), call).toBe(false);
  });

  it("projects the constants the committed vectors were generated with", () => {
    expect(SWINGS.constants["releaseWeights"]).toEqual([...RELEASE_WEIGHTS]);
    expect(SWINGS.constants["hangGravity"]).toBe(HANG_GRAVITY);
    expect(SWINGS.constants["followStiffness"]).toBe(FOLLOW_STIFFNESS);
  });
});
