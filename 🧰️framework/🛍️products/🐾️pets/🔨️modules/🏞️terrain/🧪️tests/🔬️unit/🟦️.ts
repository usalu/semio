/** 🏞️ Unit suite of the terrain module: perches against polygon-clipping's boolean operations, strides, falls, landings and hops against their closed forms and their per-tick sums.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see https://github.com/mfogel/polygon-clipping — the JavaScript oracle of the perch interval subtraction
 */
import { readFileSync } from "node:fs";
import polygonClipping from "polygon-clipping";
import { describe, expect, it } from "vitest";
import { type Perch, type Point, type Rect, type Surface, TICKS_PER_SECOND } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT, HOP_STEEPNESS, HOP_TICKS, type Hop, fallStep, hopLanding, hopOf, hopStep, landingOf, nearestPerch, perchAt, perchesOf, strideTo } from "../../🟦️.ts";

type Box = [number, number][][];

/** 🗺️ How many generated layouts are compared with polygon-clipping at the level of the run. */
const LAYOUTS = sampled(30, 300, 3000);

/** 🚶️ How many generated walks are followed tick by tick at the level of the run. */
const WALKS = sampled(6, 200, 600);

/** 🕸️ The mesh of the grid of hop targets at the level of the run, in steps of 6.5 px across and 5.25 px down: the number of targets, and with it the least number of granted and of refused hops, goes with 1 ÷ mesh². */
const MESH = sampled(4, 1, 0.5);

/** 📦️ An axis-aligned box as a closed polygon ring. */
function box(x0: number, y0: number, x1: number, y1: number): Box {
  return [[[x0, y0], [x1, y0], [x1, y1], [x0, y1], [x0, y0]]];
}

/** 🪚️ The perches polygon-clipping finds: the surface strip intersected with the viewport, minus the column of every keep-out whose box overlaps the headroom band above the strip with an area. */
function clippedPerches(surfaces: readonly Surface[], keepouts: readonly Rect[], width: number, height: number, clearance: number, minimum: number): Perch[] {
  const perches: Perch[] = [];
  for (const surface of surfaces) {
    if (surface.y < clearance || surface.y > height || surface.x1 <= surface.x0 || width <= 0) continue;
    const strip = polygonClipping.intersection(box(surface.x0, 0, surface.x1, 1), box(0, 0, width, 1));
    if (strip.length === 0) continue;
    const band = clearance > 0 ? polygonClipping.intersection(box(surface.x0, surface.y - clearance, surface.x1, surface.y), box(0, surface.y - clearance, width, surface.y)) : [];
    const columns = keepouts.filter((keepout) => keepout.width > 0 && keepout.height > 0 && band.length > 0 && polygonClipping.intersection(band, box(keepout.x, keepout.y, keepout.x + keepout.width, keepout.y + keepout.height)).length > 0).map((keepout) => box(keepout.x, -1, keepout.x + keepout.width, 2));
    const free = columns.length === 0 ? strip : polygonClipping.difference(strip, ...columns);
    const stretches = free.map((polygon) => polygon[0]!.map((pair) => pair[0])).map((xs) => [Math.min(...xs), Math.max(...xs)] as const);
    for (const [x0, x1] of stretches.sort((left, right) => left[0] - right[0])) if (x1 - x0 >= minimum) perches.push({ surface: surface.id, x0, x1, y: surface.y });
  }
  return perches;
}

/** 🎰️ A deterministic stream of integers in `[0, bound)` (a 32-bit linear congruential generator). */
function stream(seed: number): (bound: number) => number {
  let state = seed >>> 0;
  return (bound) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return Math.floor((state / 4294967296) * bound);
  };
}

/** 🛤️ The feet of a hop after every tick, flown with `hopStep`. */
function flown(from: Point, to: Point, hop: Hop): Point[] {
  const path: Point[] = [];
  let flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
  for (let left = hop.ticks; left >= 1; left--) {
    flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
    path.push({ x: flight.x, y: flight.y });
  }
  return path;
}

/** 🧮️ Where the plain per-tick sum ends without the last tick's snap: `x ← x + vx ÷ 64` and `fallStep`, `ticks` times. */
function summed(from: Point, hop: Hop): { x: number; y: number; vy: number } {
  let x = from.x;
  let fall = { y: from.y, vy: hop.vy };
  for (let tick = 0; tick < hop.ticks; tick++) {
    x = x + hop.vx / TICKS_PER_SECOND;
    fall = fallStep(fall.y, fall.vy);
  }
  return { x, y: fall.y, vy: fall.vy };
}

/** 🪵️ A surface. */
function surface(id: string, x0: number, x1: number, y: number): Surface {
  return { id, x0, x1, y };
}

/** 🚧️ A keep-out box. */
function keepout(x: number, y: number, width: number, height: number): Rect {
  return { x, y, width, height };
}

/** 🪺️ A perch. */
function perch(id: string, x0: number, x1: number, y: number): Perch {
  return { surface: id, x0, x1, y };
}

describe("perchesOf", () => {
  it("keeps free surfaces whole, in surface order", () => {
    const surfaces = [surface("b", 300, 500, 200), surface("a", 20, 220, 120), surface("floor", 0, 640, 480)];
    expect(perchesOf(surfaces, [], 640, 480, 56, 72)).toEqual([perch("b", 300, 500, 200), perch("a", 20, 220, 120), perch("floor", 0, 640, 480)]);
  });

  it("clips a surface to the viewport and drops what lies outside or only touches it", () => {
    const surfaces = [surface("left", -80, 120, 200), surface("right", 560, 720, 200), surface("wide", -50, 700, 300), surface("out", 640, 800, 200), surface("gone", -200, 0, 200), surface("far", 900, 1000, 200)];
    expect(perchesOf(surfaces, [], 640, 480, 56, 72)).toEqual([perch("left", 0, 120, 200), perch("right", 560, 640, 200), perch("wide", 0, 640, 300)]);
  });

  it("skips surfaces above the clearance line and below the viewport, and keeps both limits", () => {
    const surfaces = [surface("high", 0, 200, 55.5), surface("line", 0, 200, 56), surface("floor", 0, 640, 480), surface("below", 0, 640, 480.5)];
    expect(perchesOf(surfaces, [], 640, 480, 56, 72)).toEqual([perch("line", 0, 200, 56), perch("floor", 0, 640, 480)]);
  });

  it("subtracts a keep-out only when it reaches into the band [y − clearance, y)", () => {
    const card = [surface("card", 100, 400, 200)];
    const cut = [perch("card", 100, 200, 200), perch("card", 260, 400, 200)];
    const whole = [perch("card", 100, 400, 200)];
    expect(perchesOf(card, [keepout(200, 150, 60, 20)], 640, 480, 56, 0)).toEqual(cut);
    expect(perchesOf(card, [keepout(200, 100, 60, 44)], 640, 480, 56, 0)).toEqual(whole);
    expect(perchesOf(card, [keepout(200, 100, 60, 44.5)], 640, 480, 56, 0)).toEqual(cut);
    expect(perchesOf(card, [keepout(200, 200, 60, 80)], 640, 480, 56, 0)).toEqual(whole);
    expect(perchesOf(card, [keepout(200, 199.5, 60, 80)], 640, 480, 56, 0)).toEqual(cut);
    expect(perchesOf(card, [keepout(200, 20, 60, 400)], 640, 480, 56, 0)).toEqual(cut);
    expect(perchesOf(card, [keepout(200, 20, 60, 400)], 640, 480, 0, 0)).toEqual(whole);
  });

  it("takes nothing away for keep-outs that only touch an end or have no area", () => {
    const card = [surface("card", 100, 400, 200)];
    const whole = [perch("card", 100, 400, 200)];
    expect(perchesOf(card, [keepout(40, 150, 60, 40), keepout(400, 150, 60, 40)], 640, 480, 56, 0)).toEqual(whole);
    expect(perchesOf(card, [keepout(250, 150, 0, 40), keepout(250, 170, 40, 0), keepout(250, 170, -40, 20), keepout(250, 190, 40, -20)], 640, 480, 56, 0)).toEqual(whole);
    expect(perchesOf(card, [keepout(40, 150, 61, 40), keepout(399, 150, 60, 40)], 640, 480, 56, 0)).toEqual([perch("card", 101, 399, 200)]);
  });

  it("merges nested, overlapping and abutting keep-outs in any order", () => {
    const card = [surface("card", 0, 600, 200)];
    const keepouts = [keepout(100, 160, 200, 30), keepout(150, 170, 50, 10), keepout(250, 150, 100, 60), keepout(350, 190, 50, 5), keepout(500, 180, 40, 40)];
    const expected = [perch("card", 0, 100, 200), perch("card", 400, 500, 200), perch("card", 540, 600, 200)];
    expect(perchesOf(card, keepouts, 640, 480, 56, 0)).toEqual(expected);
    expect(perchesOf(card, [...keepouts].reverse(), 640, 480, 56, 0)).toEqual(expected);
    expect(perchesOf(card, [keepouts[2]!, keepouts[4]!, keepouts[0]!, keepouts[3]!, keepouts[1]!], 640, 480, 56, 0)).toEqual(expected);
  });

  it("keeps stretches of exactly the minimum, drops narrower ones and never yields a zero-width perch", () => {
    const card = [surface("card", 0, 300, 200)];
    const keepouts = [keepout(72, 180, 28, 10), keepout(171.5, 180, 28.5, 10), keepout(200, 180, 100, 10)];
    expect(perchesOf(card, keepouts, 640, 480, 56, 72)).toEqual([perch("card", 0, 72, 200)]);
    expect(perchesOf(card, keepouts, 640, 480, 56, 71.5)).toEqual([perch("card", 0, 72, 200), perch("card", 100, 171.5, 200)]);
    expect(perchesOf(card, [keepout(0, 180, 150, 10), keepout(150, 180, 150, 10)], 640, 480, 56, 0)).toEqual([]);
    expect(perchesOf([surface("edge", -100, 0, 200), surface("point", 50, 50, 200)], [], 640, 480, 56, 0)).toEqual([]);
    expect(perchesOf(card, [], 640, 480, 56, 300)).toEqual([perch("card", 0, 300, 200)]);
    expect(perchesOf(card, [], 640, 480, 56, 300.5)).toEqual([]);
  });

  it("agrees with polygon-clipping on generated layouts", () => {
    const next = stream(20261002);
    let perches = 0;
    for (let layout = 0; layout < LAYOUTS; layout++) {
      const width = 320 + next(8) * 40;
      const height = 240 + next(6) * 40;
      const clearance = [0, 40.5, 48, 56, 61.7][next(5)]!;
      const minimum = [0, 24, 60.5, 72][next(4)]!;
      const grain = [1, 4, 10][layout % 3]!;
      const surfaces = Array.from({ length: 1 + next(5) }, (_, index) => {
        const x0 = next(2 * width * grain) / grain - width / 2;
        return surface(`s${index}`, x0, x0 + 8 + next(width * grain) / grain, next((height + 40) * grain) / grain - 10);
      });
      const keepouts = Array.from({ length: next(9) }, () => keepout(next((width + 80) * grain) / grain - 40, next((height + 80) * grain) / grain - 40, next(6) === 0 ? 0 : next(160 * grain) / grain, next(7) === 0 ? 0 : next(120 * grain) / grain));
      const found = perchesOf(surfaces, keepouts, width, height, clearance, minimum);
      expect(found, `layout ${layout}`).toEqual(clippedPerches(surfaces, keepouts, width, height, clearance, minimum));
      perches += found.length;
    }
    expect(perches).toBeGreaterThan(LAYOUTS);
  });
});

describe("perchAt", () => {
  const perches = [perch("card", 0, 100, 200), perch("card", 100, 180, 200), perch("other", 0, 300, 200), perch("card", 240, 300, 200)];

  it("finds the perch of a surface that carries x, ends included, the first one where two touch", () => {
    expect(perchAt(perches, "card", 0)).toBe(perches[0]);
    expect(perchAt(perches, "card", 100)).toBe(perches[0]);
    expect(perchAt(perches, "card", 100.5)).toBe(perches[1]);
    expect(perchAt(perches, "card", 300)).toBe(perches[3]);
    expect(perchAt(perches, "other", 200)).toBe(perches[2]);
  });

  it("answers null in a gap, beyond the ends and for an unknown surface", () => {
    expect(perchAt(perches, "card", 200)).toBeNull();
    expect(perchAt(perches, "card", -0.5)).toBeNull();
    expect(perchAt(perches, "card", 300.5)).toBeNull();
    expect(perchAt(perches, "none", 50)).toBeNull();
    expect(perchAt([], "card", 50)).toBeNull();
  });
});

describe("nearestPerch", () => {
  const perches = [perch("a", 0, 100, 200), perch("b", 200, 300, 120), perch("floor", 0, 640, 480), perch("c", 200, 300, 280)];

  it("measures to the nearest point of each perch", () => {
    expect(nearestPerch(perches, 50, 190)).toBe(perches[0]);
    expect(nearestPerch(perches, 150, 200)).toBe(perches[0]);
    expect(nearestPerch(perches, 190, 130)).toBe(perches[1]);
    expect(nearestPerch(perches, 600, 300)).toBe(perches[2]);
    expect(nearestPerch(perches, 250, 201)).toBe(perches[3]);
    expect(nearestPerch(perches, -400, -400)).toBe(perches[0]);
  });

  it("prefers the first perch among equally near ones and answers null without perches", () => {
    expect(nearestPerch(perches, 250, 200)).toBe(perches[1]);
    expect(nearestPerch([perches[3]!, perches[1]!], 250, 200)).toBe(perches[3]);
    expect(nearestPerch(perches, 150, 160)).toBe(perches[0]);
    expect(nearestPerch([], 10, 10)).toBeNull();
  });
});

describe("strideTo", () => {
  it("advances speed ÷ 64 pixels per tick in either direction", () => {
    expect(strideTo(10, 100, 64)).toBe(11);
    expect(strideTo(10, -100, 64)).toBe(9);
    expect(strideTo(10, 100, 48)).toBe(10.75);
    expect(strideTo(-3.5, -100, 40)).toBe(-4.125);
  });

  it("never overshoots: the goal is returned as soon as it is within one step", () => {
    expect(strideTo(99.5, 100, 64)).toBe(100);
    expect(strideTo(99, 100, 64)).toBe(100);
    expect(strideTo(100.25, 100, 64)).toBe(100);
    expect(strideTo(100, 100, 64)).toBe(100);
    expect(strideTo(98.5, 100, 64)).toBe(99.5);
  });

  it("stands still without a positive speed unless it is on its goal already", () => {
    expect(strideTo(10, 100, 0)).toBe(10);
    expect(strideTo(10, 100, -64)).toBe(10);
    expect(strideTo(10, 10, 0)).toBe(10);
  });

  it("arrives after ceil(distance ÷ step) ticks and follows the closed form on the way", () => {
    const next = stream(7);
    for (let walk = 0; walk < WALKS; walk++) {
      const start = next(1280) / 2 - 160;
      const goal = next(1280) / 2 - 160;
      const speed = 8 + next(120);
      const step = speed / TICKS_PER_SECOND;
      const ticks = Math.ceil(Math.abs(goal - start) / step);
      let x = start;
      for (let tick = 1; tick <= ticks + 2; tick++) {
        x = strideTo(x, goal, speed);
        const expected = start + Math.sign(goal - start) * Math.min(tick * step, Math.abs(goal - start));
        expect(Math.abs(x - expected), `walk ${walk} tick ${tick}`).toBeLessThan(1e-9);
        if (tick >= ticks) expect(x).toBe(goal);
        else expect(x).not.toBe(goal);
      }
    }
  });
});

describe("fallStep", () => {
  it("adds gravity to the speed first and moves by the new speed", () => {
    expect(GRAVITY / TICKS_PER_SECOND).toBe(28.125);
    expect(fallStep(100, 0)).toEqual({ y: 100.439453125, vy: 28.125 });
    expect(fallStep(100, -225)).toEqual({ y: 96.923828125, vy: -196.875 });
  });

  it("follows y + vy·k ÷ 64 + GRAVITY·k·(k + 1) ÷ 8192 below the terminal speed", () => {
    const starts: readonly (readonly [number, number])[] = [[100, 0], [320.5, -410.25], [12, 33.3], [0, -550]];
    for (const [y, vy] of starts) {
      let fall = { y, vy };
      for (let tick = 1; tick <= 30 && vy + tick * 28.125 <= FALL_SPEED; tick++) {
        fall = fallStep(fall.y, fall.vy);
        expect(Math.abs(fall.vy - (vy + tick * 28.125))).toBeLessThan(1e-9);
        expect(Math.abs(fall.y - (y + (vy * tick) / 64 + (GRAVITY * tick * (tick + 1)) / 8192))).toBeLessThan(1e-9);
      }
    }
  });

  it("reaches the terminal speed after 32 ticks from rest and keeps it", () => {
    let fall = { y: 0, vy: 0 };
    for (let tick = 0; tick < 32; tick++) fall = fallStep(fall.y, fall.vy);
    expect(fall).toEqual({ y: (GRAVITY * 32 * 33) / 8192, vy: FALL_SPEED });
    const next = fallStep(fall.y, fall.vy);
    expect(next).toEqual({ y: fall.y + FALL_SPEED / 64, vy: FALL_SPEED });
    expect(fallStep(0, 5000)).toEqual({ y: FALL_SPEED / 64, vy: FALL_SPEED });
  });
});

describe("landingOf", () => {
  const perches = [perch("floor", 0, 640, 480), perch("low", 100, 300, 300), perch("high", 150, 250, 200), perch("twin", 0, 640, 200), perch("side", 400, 500, 250)];

  it("answers the highest perch crossed at x, the first one among equals", () => {
    expect(landingOf(perches, 200, 100, 500)).toBe(perches[2]);
    expect(landingOf(perches, 120, 100, 500)).toBe(perches[3]);
    expect(landingOf(perches, 200, 201, 500)).toBe(perches[1]);
    expect(landingOf(perches, 200, 301, 500)).toBe(perches[0]);
    expect(landingOf(perches, 450, 210, 260)).toBe(perches[4]);
  });

  it("includes both heights and both ends of a perch", () => {
    expect(landingOf(perches, 200, 300, 310)).toBe(perches[1]);
    expect(landingOf(perches, 200, 290, 300)).toBe(perches[1]);
    expect(landingOf(perches, 100, 290, 310)).toBe(perches[1]);
    expect(landingOf(perches, 300, 290, 310)).toBe(perches[1]);
    expect(landingOf(perches, 300.5, 290, 310)).toBeNull();
    expect(landingOf(perches, 99.5, 290, 310)).toBeNull();
    expect(landingOf(perches, 200, 300, 300)).toBe(perches[1]);
  });

  it("answers null between perches, beside them and while rising", () => {
    expect(landingOf(perches, 200, 210, 290)).toBeNull();
    expect(landingOf(perches, 200, 481, 600)).toBeNull();
    expect(landingOf(perches, 700, 0, 600)).toBeNull();
    expect(landingOf(perches, 200, 310, 190)).toBeNull();
    expect(landingOf([], 200, 0, 600)).toBeNull();
  });

  it("stops a fall on the first perch below, however fast, and lets it pass the ones beside", () => {
    let fall = { y: 40, vy: 0 };
    let landing: Perch | null = null;
    let ticks = 0;
    while (landing === null && ticks < 200) {
      const next = fallStep(fall.y, fall.vy);
      landing = landingOf([perches[0]!, perches[1]!, perches[4]!], 350, fall.y, next.y);
      fall = next;
      ticks++;
    }
    expect(landing).toBe(perches[0]);
    expect(fall.vy).toBe(FALL_SPEED);
    expect(ticks).toBe(47);
    expect(fall.y).toBeGreaterThan(480);
    expect(fall.y).toBeLessThan(480 + FALL_SPEED / 64);
  });
});

describe("hopOf", () => {
  it("launches a hop on the spot with the clearance as its apex", () => {
    expect(hopOf({ x: 50, y: 200 }, { x: 50, y: 200 })).toEqual({ vx: 0, vy: -225, ticks: 15 });
  });

  it("is out of reach when too high, too far, too long or landing faster than a fall", () => {
    const from = { x: 100, y: 300 };
    expect(hopOf(from, { x: 100, y: 300 - (HOP_HEIGHT - HOP_CLEARANCE) })).not.toBeNull();
    expect(hopOf(from, { x: 100, y: 300 - (HOP_HEIGHT - HOP_CLEARANCE) - 0.5 })).toBeNull();
    expect(hopOf(from, { x: 100 + HOP_DISTANCE, y: 300 })).not.toBeNull();
    expect(hopOf(from, { x: 100 - HOP_DISTANCE, y: 300 })).not.toBeNull();
    expect(hopOf(from, { x: 100 + HOP_DISTANCE + 0.5, y: 300 })).toBeNull();
    expect(hopOf(from, { x: 100 - HOP_DISTANCE - 0.5, y: 300 })).toBeNull();
    expect(hopOf(from, { x: 260, y: 455 })!.ticks).toBe(HOP_TICKS);
    expect(hopOf(from, { x: 260, y: 465 })).toBeNull();
    const fast = hopOf(from, { x: 100, y: 510 })!;
    expect(fast.ticks).toBeLessThan(HOP_TICKS);
    expect(fast.vy + fast.ticks * 28.125).toBeLessThanOrEqual(FALL_SPEED);
    expect(hopOf(from, { x: 100, y: 520 })).toBeNull();
  });

  it("is out of reach for points that are not finite numbers", () => {
    const from = { x: 100, y: 300 };
    for (const value of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
      expect(hopOf(from, { x: value, y: 300 })).toBeNull();
      expect(hopOf(from, { x: 100, y: value })).toBeNull();
      expect(hopOf({ x: value, y: 300 }, from)).toBeNull();
      expect(hopOf({ x: 100, y: value }, from)).toBeNull();
    }
  });

  it("mirrors with the direction and does not depend on where the hop starts", () => {
    const right = hopOf({ x: 100, y: 300 }, { x: 196, y: 276 })!;
    const left = hopOf({ x: 100, y: 300 }, { x: 4, y: 276 })!;
    const moved = hopOf({ x: 612, y: 44 }, { x: 708, y: 20 })!;
    expect(left).toEqual({ vx: -right.vx, vy: right.vy, ticks: right.ticks });
    expect(moved).toEqual(right);
  });

  it("lands every granted hop on its target after its ticks, with an apex above both ends", () => {
    let granted = 0;
    let refused = 0;
    for (let dx = -168; dx <= 168; dx += 6.5 * MESH) {
      for (let dy = -80; dy <= 230; dy += 5.25 * MESH) {
        const from = { x: 217.3, y: 301.7 };
        const to = { x: from.x + dx, y: from.y + dy };
        const hop = hopOf(from, to);
        if (hop === null) {
          refused++;
          continue;
        }
        granted++;
        const label = `dx ${dx} dy ${dy}`;
        expect(Number.isInteger(hop.ticks) && hop.ticks >= 15 && hop.ticks <= HOP_TICKS, label).toBe(true);
        expect(Math.abs(dx), label).toBeLessThanOrEqual(HOP_DISTANCE);
        const path = flown(from, to, hop);
        expect(path.length, label).toBe(hop.ticks);
        expect(path[path.length - 1], label).toEqual(to);
        const end = summed(from, hop);
        expect(Math.abs(end.x - to.x), label).toBeLessThan(1e-9);
        expect(Math.abs(end.y - to.y), label).toBeLessThan(1e-9);
        expect(end.vy, label).toBeLessThanOrEqual(FALL_SPEED + 1e-9);
        expect(Math.abs(end.vy - (hop.vy + hop.ticks * 28.125)), label).toBeLessThan(1e-9);
        const apex = Math.min(...path.map((point) => point.y));
        const rise = Math.max(HOP_CLEARANCE - Math.min(dy, 0), Math.abs(dx) * HOP_STEEPNESS);
        expect(Math.min(from.y, to.y) - apex, label).toBeGreaterThan(HOP_CLEARANCE - 3);
        expect(Math.abs(from.y - apex - rise), label).toBeLessThan(3);
        expect(from.y - apex, label).toBeLessThan(HOP_HEIGHT + 3);
        expect(path[0]!.y, label).toBeLessThan(from.y);
        expect(path[path.length - 2]!.y, label).toBeLessThan(to.y);
        for (let tick = 1; tick < path.length; tick++) expect(Math.sign(path[tick]!.x - path[tick - 1]!.x), label).toBe(Math.sign(dx));
      }
    }
    expect(granted).toBeGreaterThan(1500 / (MESH * MESH));
    expect(refused).toBeGreaterThan(500 / (MESH * MESH));
  });
});

describe("hopStep", () => {
  it("moves by the fall step and vx ÷ 64 while more than one tick is left", () => {
    const fallen = fallStep(200, -225);
    expect(hopStep(50, 200, 128, -225, { x: 80, y: 200 }, 15)).toEqual({ x: 52, y: fallen.y, vx: 128, vy: fallen.vy });
    expect(hopStep(50, 200, 128, -225, { x: 80, y: 200 }, 2)).toEqual({ x: 52, y: fallen.y, vx: 128, vy: fallen.vy });
  });

  it("puts the last tick on the target and keeps the speed of the fall step", () => {
    const fallen = fallStep(190, 170);
    expect(hopStep(78, 190, 128, 170, { x: 80, y: 200 }, 1)).toEqual({ x: 80, y: 200, vx: 128, vy: fallen.vy });
    expect(hopStep(78, 190, 128, 170, { x: 80, y: 200 }, 0)).toEqual({ x: 80, y: 200, vx: 128, vy: fallen.vy });
  });
});

describe("hopLanding", () => {
  const launch = perch("launch", 0, 120, 300);
  const from = { x: 100, y: 300 };

  it("ends on the perch of the target when nothing lies in the way", () => {
    const level = perch("level", 150, 300, 300);
    const higher = perch("higher", 150, 300, 240);
    const lower = perch("lower", 150, 300, 420);
    expect(hopLanding([launch, level], from, { x: 180, y: 300 }, hopOf(from, { x: 180, y: 300 })!)).toBe(level);
    expect(hopLanding([launch, higher], from, { x: 180, y: 240 }, hopOf(from, { x: 180, y: 240 })!)).toBe(higher);
    expect(hopLanding([launch, lower], from, { x: 180, y: 420 }, hopOf(from, { x: 180, y: 420 })!)).toBe(lower);
    expect(hopLanding([launch], from, { x: 40, y: 300 }, hopOf(from, { x: 40, y: 300 })!)).toBe(launch);
    expect(hopLanding([level, perch("end", 150, 180, 300)], from, { x: 180, y: 300 }, hopOf(from, { x: 180, y: 300 })!)).toBe(level);
  });

  it("ends on whatever is crossed first on the way down", () => {
    const lower = perch("lower", 90, 300, 420);
    const shelf = perch("shelf", 170, 195, 360);
    const down = { x: 110, y: 420 };
    expect(hopLanding([launch, lower], from, down, hopOf(from, down)!)).toBe(launch);
    const far = { x: 200, y: 420 };
    expect(hopLanding([launch, lower, shelf], from, far, hopOf(from, far)!)).toBe(shelf);
    expect(hopLanding([launch, lower], from, far, hopOf(from, far)!)).toBe(lower);
  });

  it("passes through a perch from below and answers null when the target carries nothing", () => {
    const roof = perch("roof", 0, 300, 290);
    const up = { x: 180, y: 240 };
    expect(hopLanding([launch, roof, perch("higher", 150, 300, 240)], from, up, hopOf(from, up)!)?.surface).toBe("higher");
    expect(hopLanding([launch], from, { x: 180, y: 300 }, hopOf(from, { x: 180, y: 300 })!)).toBeNull();
    expect(hopLanding([], from, from, hopOf(from, from)!)).toBeNull();
  });
});

describe("determinism", () => {
  it("uses nothing but exact operations and has no side effect", () => {
    const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
    const used = [...source.matchAll(/Math\.(\w+)/g)].map((match) => match[1]!);
    expect([...new Set(used)].sort()).toEqual(["abs", "floor", "max", "min", "sqrt"]);
    expect(source).not.toMatch(/\b(Date|performance|console|globalThis|process)\b/);
    expect(source).not.toMatch(/\S \*\* \S/);
  });
});
