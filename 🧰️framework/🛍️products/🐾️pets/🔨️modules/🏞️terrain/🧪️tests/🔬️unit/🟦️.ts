/** 🏞️ Unit suite of the terrain module: perches and pitches against polygon-clipping's boolean operations, strides, falls, landings and hops against their closed forms and their per-tick sums, lines of sight against the separating axes of a segment and a box.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see https://github.com/mfogel/polygon-clipping — the JavaScript oracle of the perch and pitch interval subtraction
 */
import { readFileSync } from "node:fs";
import polygonClipping from "polygon-clipping";
import { describe, expect, it } from "vitest";
import { type Perch, type Pitch, type Point, type Rect, type Surface, TICKS_PER_SECOND } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { FALL_SPEED, GRAVITY, HOP_CLEARANCE, HOP_DISTANCE, HOP_HEIGHT, HOP_STEEPNESS, HOP_TICKS, type Hop, WALL_LIP, fallStep, hopLanding, hopOf, hopStep, landingOf, nearestPerch, nearestWall, perchAt, perchesOf, segmentClear, segmentHits, strideTo, wallAt, wallsOf } from "../../🟦️.ts";

type Box = [number, number][][];
type Wall = Parameters<typeof wallsOf>[0][number];

/** 🗺️ How many generated layouts are compared with polygon-clipping at the level of the run. */
const LAYOUTS = sampled(30, 300, 3000);

/** 🔦️ How many generated segments are judged by the separating axes at the level of the run. */
const SIGHTS = sampled(400, 4000, 40000);

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

/** 🏯️ A wall. */
function wall(id: string, surface: string, side: 1 | -1, x: number, y0: number, y1: number): Wall {
  return { id, surface, side, x, y0, y1 };
}

/** 🧗️ A pitch. */
function pitch(id: string, surface: string, side: 1 | -1, x: number, y0: number, y1: number): Pitch {
  return { wall: id, surface, side, x, y0, y1 };
}

/** 🪓️ The pitches polygon-clipping finds: the wall as a strip intersected with the viewport, minus the row of every keep-out whose box overlaps the band on its air side, from the lip to the clearance, with an area. */
function clippedPitches(walls: readonly Wall[], keepouts: readonly Rect[], width: number, height: number, clearance: number, minimum: number): Pitch[] {
  const pitches: Pitch[] = [];
  for (const edge of walls) {
    const low = edge.side < 0 ? edge.x - clearance : edge.x + WALL_LIP;
    const high = edge.side < 0 ? edge.x - WALL_LIP : edge.x + clearance;
    if (Math.min(edge.x, edge.x + edge.side * clearance) < 0 || Math.max(edge.x, edge.x + edge.side * clearance) > width || edge.y1 <= edge.y0 || height <= 0) continue;
    const strip = polygonClipping.intersection(box(0, edge.y0, 1, edge.y1), box(0, 0, 1, height));
    if (strip.length === 0) continue;
    const band = low < high ? polygonClipping.intersection(box(low, edge.y0, high, edge.y1), box(low, 0, high, height)) : [];
    const rows = keepouts.filter((keepout) => keepout.width > 0 && keepout.height > 0 && band.length > 0 && polygonClipping.intersection(band, box(keepout.x, keepout.y, keepout.x + keepout.width, keepout.y + keepout.height)).length > 0).map((keepout) => box(-1, keepout.y, 2, keepout.y + keepout.height));
    const free = rows.length === 0 ? strip : polygonClipping.difference(strip, ...rows);
    const stretches = free.map((polygon) => polygon[0]!.map((pair) => pair[1])).map((ys) => [Math.min(...ys), Math.max(...ys)] as const);
    for (const [y0, y1] of stretches.sort((upper, lower) => upper[0] - lower[0])) if (y1 - y0 >= minimum) pitches.push(pitch(edge.id, edge.surface, edge.side, edge.x, y0, y1));
  }
  return pitches;
}

/** ⚔️ Whether a segment meets the inside of a box grown by a margin, by the separating axes of the two: their extents overlap on both axes of the box, and the corners of the box lie on both sides of the line of the segment (a segment of no length is a point inside). */
function crossed(from: Point, to: Point, rect: Rect, margin: number): boolean {
  const left = rect.x - margin;
  const right = rect.x + rect.width + margin;
  const top = rect.y - margin;
  const bottom = rect.y + rect.height + margin;
  if (!(rect.width > 0 && rect.height > 0 && left < right && top < bottom)) return false;
  if (!(Math.min(from.x, to.x) < right && Math.max(from.x, to.x) > left && Math.min(from.y, to.y) < bottom && Math.max(from.y, to.y) > top)) return false;
  const sides = [[left, top], [right, top], [right, bottom], [left, bottom]].map(([x, y]) => (to.x - from.x) * (y! - from.y) - (to.y - from.y) * (x! - from.x));
  return (from.x === to.x && from.y === to.y) || (Math.min(...sides) < 0 && Math.max(...sides) > 0);
}

/** 🧵️ Whether one of `count + 1` evenly spaced points of a segment lies inside a box grown by a margin, by more than a millionth of a pixel. */
function threaded(from: Point, to: Point, rect: Rect, margin: number, count: number): boolean {
  for (let index = 0; index <= count; index++) {
    const x = from.x + ((to.x - from.x) * index) / count;
    const y = from.y + ((to.y - from.y) * index) / count;
    if (rect.width > 0 && rect.height > 0 && x - (rect.x - margin) > 1e-6 && rect.x + rect.width + margin - x > 1e-6 && y - (rect.y - margin) > 1e-6 && rect.y + rect.height + margin - y > 1e-6) return true;
  }
  return false;
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

describe("wallsOf", () => {
  const left = [wall("left", "card", -1, 300, 100, 400)];
  const right = [wall("right", "card", 1, 300, 100, 400)];

  it("keeps free walls whole, in wall order, on either side", () => {
    const walls = [wall("b-right", "b", 1, 500, 200, 400), wall("a-left", "a", -1, 100, 120, 300), wall("a-right", "a", 1, 220, 120, 300)];
    expect(wallsOf(walls, [], 640, 480, 48, 72)).toEqual([pitch("b-right", "b", 1, 500, 200, 400), pitch("a-left", "a", -1, 100, 120, 300), pitch("a-right", "a", 1, 220, 120, 300)]);
  });

  it("drops a wall whose band leaves the viewport and clips the others to its height", () => {
    const walls = [wall("near-left", "a", -1, 47.5, 100, 300), wall("on-left", "a", -1, 48, 100, 300), wall("near-right", "b", 1, 592.5, 100, 300), wall("on-right", "b", 1, 592, 100, 300), wall("beyond", "c", -1, 640.5, 100, 300), wall("edge", "c", -1, 640, 100, 300), wall("before", "d", 1, -0.5, 100, 300), wall("origin", "d", 1, 0, 100, 300), wall("tall", "e", -1, 300, -80, 560), wall("above", "f", 1, 300, -200, 0), wall("below", "f", 1, 300, 480, 600)];
    expect(wallsOf(walls, [], 640, 480, 48, 72)).toEqual([pitch("on-left", "a", -1, 48, 100, 300), pitch("on-right", "b", 1, 592, 100, 300), pitch("edge", "c", -1, 640, 100, 300), pitch("origin", "d", 1, 0, 100, 300), pitch("tall", "e", -1, 300, 0, 480)]);
  });

  it("subtracts a keep-out only when it reaches into the band from the lip to the clearance on the air side", () => {
    const cut = [100, 200, 250, 400];
    const found = (walls: readonly Wall[], blocker: Rect, clearance: number) => wallsOf(walls, [blocker], 640, 480, clearance, 0).flatMap((free) => [free.y0, free.y1]);
    expect(WALL_LIP).toBe(6);
    expect(found(left, keepout(260, 200, 20, 50), 48)).toEqual(cut);
    expect(found(left, keepout(296, 100, 208, 300), 48)).toEqual([100, 400]);
    expect(found(left, keepout(294, 200, 100, 50), 48)).toEqual([100, 400]);
    expect(found(left, keepout(293.5, 200, 100, 50), 48)).toEqual(cut);
    expect(found(left, keepout(200, 200, 52, 50), 48)).toEqual([100, 400]);
    expect(found(left, keepout(200, 200, 52.5, 50), 48)).toEqual(cut);
    expect(found(left, keepout(310, 200, 30, 50), 48)).toEqual([100, 400]);
    expect(found(left, keepout(260, 200, 20, 50), 6)).toEqual([100, 400]);
    expect(found(left, keepout(260, 200, 20, 50), 0)).toEqual([100, 400]);
    expect(found(right, keepout(320, 200, 20, 50), 48)).toEqual(cut);
    expect(found(right, keepout(96, 100, 208, 300), 48)).toEqual([100, 400]);
    expect(found(right, keepout(200, 200, 106, 50), 48)).toEqual([100, 400]);
    expect(found(right, keepout(200, 200, 106.5, 50), 48)).toEqual(cut);
    expect(found(right, keepout(348, 200, 40, 50), 48)).toEqual([100, 400]);
    expect(found(right, keepout(347.5, 200, 40, 50), 48)).toEqual(cut);
    expect(found(right, keepout(260, 200, 20, 50), 48)).toEqual([100, 400]);
  });

  it("takes nothing away for keep-outs that only touch an end or have no area", () => {
    const whole = [pitch("left", "card", -1, 300, 100, 400)];
    expect(wallsOf(left, [keepout(260, 50, 20, 50), keepout(260, 400, 20, 50)], 640, 480, 48, 0)).toEqual(whole);
    expect(wallsOf(left, [keepout(260, 200, 0, 50), keepout(260, 200, 20, 0), keepout(280, 200, -20, 50), keepout(260, 250, 20, -50)], 640, 480, 48, 0)).toEqual(whole);
    expect(wallsOf(left, [keepout(260, 50, 20, 50.5), keepout(260, 399.5, 20, 50)], 640, 480, 48, 0)).toEqual([pitch("left", "card", -1, 300, 100.5, 399.5)]);
  });

  it("merges nested, overlapping and abutting keep-outs in any order", () => {
    const tall = [wall("left", "card", -1, 300, 0, 600)];
    const keepouts = [keepout(260, 100, 30, 200), keepout(270, 150, 10, 50), keepout(250, 250, 60, 100), keepout(290, 350, 5, 50), keepout(280, 500, 40, 40)];
    const expected = [pitch("left", "card", -1, 300, 0, 100), pitch("left", "card", -1, 300, 400, 500), pitch("left", "card", -1, 300, 540, 600)];
    expect(wallsOf(tall, keepouts, 640, 640, 48, 0)).toEqual(expected);
    expect(wallsOf(tall, [...keepouts].reverse(), 640, 640, 48, 0)).toEqual(expected);
    expect(wallsOf(tall, [keepouts[2]!, keepouts[4]!, keepouts[0]!, keepouts[3]!, keepouts[1]!], 640, 640, 48, 0)).toEqual(expected);
  });

  it("keeps stretches of exactly the minimum, drops shorter ones and never yields a pitch of no length", () => {
    const short = [wall("left", "card", -1, 300, 0, 300)];
    const keepouts = [keepout(260, 72, 10, 28), keepout(260, 171.5, 10, 28.5), keepout(260, 200, 10, 100)];
    expect(wallsOf(short, keepouts, 640, 480, 48, 72)).toEqual([pitch("left", "card", -1, 300, 0, 72)]);
    expect(wallsOf(short, keepouts, 640, 480, 48, 71.5)).toEqual([pitch("left", "card", -1, 300, 0, 72), pitch("left", "card", -1, 300, 100, 171.5)]);
    expect(wallsOf(short, [keepout(260, 0, 10, 150), keepout(260, 150, 10, 150)], 640, 480, 48, 0)).toEqual([]);
    expect(wallsOf([wall("point", "card", -1, 300, 200, 200), wall("backwards", "card", 1, 300, 300, 200), wall("floor", "card", 1, 300, 480, 480)], [], 640, 480, 48, 0)).toEqual([]);
    expect(wallsOf(short, [], 640, 480, 48, 300)).toEqual([pitch("left", "card", -1, 300, 0, 300)]);
    expect(wallsOf(short, [], 640, 480, 48, 300.5)).toEqual([]);
  });

  it("agrees with polygon-clipping on generated layouts", () => {
    const next = stream(20261003);
    let pitches = 0;
    for (let layout = 0; layout < LAYOUTS; layout++) {
      const width = 320 + next(8) * 40;
      const height = 240 + next(6) * 40;
      const clearance = [0, 6, 28.5, 40, 48, 61.7][next(6)]!;
      const minimum = [0, 24, 60.5, 72][next(4)]!;
      const grain = [1, 4, 8][layout % 3]!;
      const walls = Array.from({ length: 1 + next(5) }, (_, index) => {
        const y0 = next(2 * height * grain) / grain - height / 2;
        return wall(`w${index}`, `s${index >> 1}`, next(2) === 0 ? -1 : 1, next((width + 80) * grain) / grain - 40, y0, y0 + 8 + next(height * grain) / grain);
      });
      const keepouts = Array.from({ length: next(9) }, () => keepout(next((width + 80) * grain) / grain - 40, next((height + 80) * grain) / grain - 40, next(6) === 0 ? 0 : next(160 * grain) / grain, next(7) === 0 ? 0 : next(120 * grain) / grain));
      const found = wallsOf(walls, keepouts, width, height, clearance, minimum);
      expect(found, `layout ${layout}`).toEqual(clippedPitches(walls, keepouts, width, height, clearance, minimum));
      pitches += found.length;
    }
    expect(pitches).toBeGreaterThan(LAYOUTS);
  });
});

describe("wallAt", () => {
  const pitches = [pitch("left", "card", -1, 300, 100, 200), pitch("left", "card", -1, 300, 200, 280), pitch("right", "card", 1, 500, 100, 400), pitch("left", "card", -1, 300, 340.5, 400)];

  it("finds the pitch of a wall that carries y, ends included, the first one where two touch", () => {
    expect(wallAt(pitches, "left", 100)).toBe(pitches[0]);
    expect(wallAt(pitches, "left", 200)).toBe(pitches[0]);
    expect(wallAt(pitches, "left", 200.5)).toBe(pitches[1]);
    expect(wallAt(pitches, "left", 400)).toBe(pitches[3]);
    expect(wallAt(pitches, "right", 300)).toBe(pitches[2]);
  });

  it("answers null in a gap, beyond the ends and for an unknown wall", () => {
    expect(wallAt(pitches, "left", 300)).toBeNull();
    expect(wallAt(pitches, "left", 99.5)).toBeNull();
    expect(wallAt(pitches, "left", 400.5)).toBeNull();
    expect(wallAt(pitches, "none", 150)).toBeNull();
    expect(wallAt([], "left", 150)).toBeNull();
  });
});

describe("nearestWall", () => {
  const pitches = [pitch("a", "a", -1, 100, 200, 300), pitch("b", "b", 1, 300, 120, 200), pitch("c", "c", -1, 500, 0, 480), pitch("d", "d", 1, 300, 280, 400)];

  it("measures to the nearest point of each pitch", () => {
    expect(nearestWall(pitches, 110, 250)).toBe(pitches[0]);
    expect(nearestWall(pitches, 100, 150)).toBe(pitches[0]);
    expect(nearestWall(pitches, 290, 130)).toBe(pitches[1]);
    expect(nearestWall(pitches, 420, 300)).toBe(pitches[2]);
    expect(nearestWall(pitches, 301, 250)).toBe(pitches[3]);
    expect(nearestWall(pitches, -400, -400)).toBe(pitches[0]);
  });

  it("prefers the first pitch among equally near ones and answers null without pitches", () => {
    expect(nearestWall(pitches, 300, 240)).toBe(pitches[1]);
    expect(nearestWall([pitches[3]!, pitches[1]!], 300, 240)).toBe(pitches[3]);
    expect(nearestWall(pitches, 200, 200)).toBe(pitches[0]);
    expect(nearestWall([], 10, 10)).toBeNull();
  });

  it("answers the wall that rises beside the end of a perch and the wall that drops from it", () => {
    const shelf = perch("shelf", 104, 296, 320);
    expect(nearestWall(pitches, shelf.x1, shelf.y)).toBe(pitches[3]);
    expect(nearestWall(pitches, shelf.x0, shelf.y)).toBe(pitches[0]);
  });
});

describe("segmentHits", () => {
  const card = keepout(100, 100, 100, 50);
  const hit = (x0: number, y0: number, x1: number, y1: number, margin = 0) => segmentHits({ x: x0, y: y0 }, { x: x1, y: y1 }, card, margin);

  it("is hit by a segment that passes through, ends in, begins in or lies in the box", () => {
    expect(hit(50, 125, 250, 125)).toBe(true);
    expect(hit(50, 50, 250, 200)).toBe(true);
    expect(hit(50, 125, 150, 125)).toBe(true);
    expect(hit(150, 125, 150, 300)).toBe(true);
    expect(hit(120, 110, 180, 140)).toBe(true);
    expect(hit(150, 50, 150, 200)).toBe(true);
  });

  it("is missed by a segment beside the box, short of it or beyond it", () => {
    expect(hit(50, 50, 250, 60)).toBe(false);
    expect(hit(50, 125, 99, 125)).toBe(false);
    expect(hit(201, 125, 300, 125)).toBe(false);
    expect(hit(50, 300, 99.5, 125)).toBe(false);
    expect(hit(0, 149, 99, 0)).toBe(false);
  });

  it("is not hit by a segment that only touches an edge or grazes a corner, and is hit a hair inside", () => {
    expect(hit(50, 100, 250, 100)).toBe(false);
    expect(hit(100, 50, 100, 200)).toBe(false);
    expect(hit(200, 50, 200, 200)).toBe(false);
    expect(hit(50, 150, 250, 150)).toBe(false);
    expect(hit(50, 125, 100, 125)).toBe(false);
    expect(hit(200, 125, 260, 125)).toBe(false);
    expect(hit(50, 150, 150, 50)).toBe(false);
    expect(hit(150, 50, 250, 150)).toBe(false);
    expect(hit(50, 100, 150, 200)).toBe(false);
    expect(hit(150, 200, 250, 100)).toBe(false);
    expect(hit(50, 100.5, 250, 100.5)).toBe(true);
    expect(hit(50, 150.5, 150.5, 50)).toBe(true);
    expect(hit(50, 125, 100.5, 125)).toBe(true);
  });

  it("treats a segment of no length as a point: hit inside the box, not on its edge or outside", () => {
    expect(hit(150, 125, 150, 125)).toBe(true);
    expect(hit(100, 125, 100, 125)).toBe(false);
    expect(hit(200, 150, 200, 150)).toBe(false);
    expect(hit(50, 50, 50, 50)).toBe(false);
  });

  it("grows the box by the margin on every side, shrinks it by a negative one, and never hits a box without area", () => {
    expect(hit(50, 98, 250, 98, 0)).toBe(false);
    expect(hit(50, 98, 250, 98, 2)).toBe(false);
    expect(hit(50, 98, 250, 98, 2.5)).toBe(true);
    expect(hit(97, 0, 97, 300, 3)).toBe(false);
    expect(hit(97, 0, 97, 300, 3.5)).toBe(true);
    expect(hit(50, 125, 250, 125, -24)).toBe(true);
    expect(hit(50, 125, 250, 125, -25)).toBe(false);
    expect(hit(50, 125, 250, 125, -40)).toBe(false);
    for (const flat of [keepout(100, 100, 0, 50), keepout(100, 100, 100, 0), keepout(100, 100, -20, 50), keepout(100, 100, 100, -5)]) expect(segmentHits({ x: 50, y: 100 }, { x: 250, y: 150 }, flat, 10)).toBe(false);
  });

  it("does not depend on the direction of the segment and agrees with the separating axes and with a sampled segment", () => {
    const next = stream(31);
    let hits = 0;
    let misses = 0;
    for (let sight = 0; sight < SIGHTS; sight++) {
      const from = { x: next(1200) / 4 - 20, y: next(1200) / 4 - 20 };
      const to = next(12) === 0 ? { x: from.x, y: next(1200) / 4 - 20 } : next(11) === 0 ? { x: next(1200) / 4 - 20, y: from.y } : { x: next(1200) / 4 - 20, y: next(1200) / 4 - 20 };
      const rect = keepout(next(800) / 4, next(800) / 4, next(9) === 0 ? 0 : next(480) / 4, next(9) === 0 ? 0 : next(480) / 4);
      const margin = [0, 0, 2, 6, -3][next(5)]!;
      const found = segmentHits(from, to, rect, margin);
      expect(found, `sight ${sight}`).toBe(crossed(from, to, rect, margin));
      expect(segmentHits(to, from, rect, margin), `sight ${sight} backwards`).toBe(found);
      if (threaded(from, to, rect, margin, 512)) expect(found, `sight ${sight} sampled`).toBe(true);
      if (found) hits++;
      else misses++;
    }
    expect(hits).toBeGreaterThan(SIGHTS / 8);
    expect(misses).toBeGreaterThan(SIGHTS / 8);
  });
});

describe("segmentClear", () => {
  const boxes = [keepout(100, 100, 100, 50), keepout(300, 40, 40, 200), keepout(260, 300, 0, 80)];

  it("is clear when the segment hits no box and blocked by any one of them", () => {
    expect(segmentClear({ x: 0, y: 20 }, { x: 400, y: 20 }, boxes, 0)).toBe(true);
    expect(segmentClear({ x: 0, y: 20 }, { x: 400, y: 20 }, [], 0)).toBe(true);
    expect(segmentClear({ x: 0, y: 125 }, { x: 250, y: 125 }, boxes, 0)).toBe(false);
    expect(segmentClear({ x: 250, y: 125 }, { x: 400, y: 125 }, boxes, 0)).toBe(false);
    expect(segmentClear({ x: 250, y: 340 }, { x: 270, y: 340 }, boxes, 0)).toBe(true);
    expect(segmentClear({ x: 210, y: 0 }, { x: 290, y: 400 }, boxes, 0)).toBe(true);
  });

  it("grows every box by the margin", () => {
    expect(segmentClear({ x: 250, y: 0 }, { x: 250, y: 400 }, boxes, 0)).toBe(true);
    expect(segmentClear({ x: 250, y: 0 }, { x: 250, y: 400 }, boxes, 50)).toBe(true);
    expect(segmentClear({ x: 250, y: 0 }, { x: 250, y: 400 }, boxes, 50.5)).toBe(false);
    expect(segmentClear({ x: 250, y: 300 }, { x: 250, y: 400 }, boxes, 49)).toBe(true);
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
