/** 🧗️ Unit suite of the climbing module: holds, climbs, slides and mantles on walls, the placement and the gait of ladders, shots, hook flights and zips of the grappling rope, and the routes between perches — each against its closed form, its per-tick sum or the law it has to keep.
 *
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧪️tests/🧗️wall-climbing/🥒️.feature — the same module against numpy and scipy
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { type Perch, type Pitch, type Point, type Rect, type Size, TICKS_PER_SECOND } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { segmentHits, wallsOf } from "../../../🏞️terrain/🟦️.ts";
import { smoothstep } from "../../../📐️trigonometry/🟦️.ts";
import { REEL_CAP, REEL_LEAST, REEL_RAMP, REEL_SPEED, reelStep } from "../../../🪢️swing/🟦️.ts";
import * as climbing from "../../🟦️.ts";
import { CLIMB_DESCENT, CLIMB_RAMP, CLIMB_RISE, GRIP_BITE, GRIP_BUDGET, GRIP_SPACING, HAND_HEIGHT, HOIST_HUMP, HOOK_INSET, HOOK_LIFT, HOOK_RETURN, HOOK_SPEED, LADDER_DESCENT, LADDER_EXIT, LADDER_FLAT, LADDER_FOLLOW, LADDER_FOOTING, LADDER_GIRTH, LADDER_LEAN, LADDER_RISE, LADDER_SHIFT, LADDER_SHORT, LADDER_STEEP, LADDER_TALL, LADDER_TUCK, MANTLE_INSET, MANTLE_TICKS, MUZZLE_FORWARD, MUZZLE_HEIGHT, ROPE_DETOUR, ROPE_ELEVATION, ROPE_FOLLOW, ROPE_LONG, ROPE_MARGIN, ROPE_MISS_OVERSHOOT, ROPE_RISE, ROPE_SHORT, RUNG_SPACING, SLIDE_GAIN, SLIDE_SPEED, SLIDE_START, SLIP_LIFT, SLIP_PUSH, TOPPLE_PUSH, TOPPLE_STEP, WALL_FOLLOW, WALL_GRAB_TICKS, WALL_HANG_TICKS, ZIP_RAMP, ZIP_SLANT, ZIP_SPEED } from "../../🟦️.ts";
import { clings, climbPhase, climbStep, climbTicks, clingOf, crowns, footOf, gripFor, gripStep, haulOf, haulStep, haulTicks, hoistPath, hookStep, hookTicks, ladderAt, ladderExit, ladderFor, ladderHolds, ladderLanding, ladderLean, ladderLength, ladderPhase, ladderRungs, ladderStep, ladderTo, landingFor, ledgeOf, mantlePath, missOf, rimFor, rimOf, routeOf, shotFor, shotHolds, slideStep, slipOf, spillOf, swayStep, wallHolds, zipStep } from "../../🟦️.ts";

type Ladder = NonNullable<ReturnType<typeof ladderFor>>;
type Shot = NonNullable<ReturnType<typeof shotFor>>;

/** 🎚️ How many generated stages each sweep of placements judges at the level of the run. */
const STAGES = sampled(60, 600, 6000);

/** 🐾️ The reference pet of the research brief and the smallest and the largest of the menagerie. */
const PET: Size = { width: 40, height: 48 };
const SMALL: Size = { width: 34, height: 40 };
const LARGE: Size = { width: 46, height: 56 };

/** 🎰️ A deterministic stream of integers in `[0, bound)` (a 32-bit linear congruential generator). */
function stream(seed: number): (bound: number) => number {
  let state = seed >>> 0;
  return (bound) => {
    state = (Math.imul(state, 1664525) + 1013904223) >>> 0;
    return Math.floor((state / 4294967296) * bound);
  };
}

/** 🪺️ A perch. */
function perch(id: string, x0: number, x1: number, y: number): Perch {
  return { surface: id, x0, x1, y };
}

/** 🧱️ A pitch. */
function pitch(id: string, surface: string, side: 1 | -1, x: number, y0: number, y1: number): Pitch {
  return { wall: id, surface, side, x, y0, y1 };
}

/** 🚧️ A keep-out box. */
function keepout(x: number, y: number, width: number, height: number): Rect {
  return { x, y, width, height };
}

/** 📦️ The box the survey reports for a card: its own, grown by 4 px on both sides. */
function solid(x0: number, x1: number, y0: number, y1: number): Rect {
  return keepout(x0 - 4, y0, x1 - x0 + 8, y1 - y0);
}

/** 🧮️ The heights of a climb after every tick until it has arrived. */
function climbed(y: number, goal: number): number[] {
  const heights: number[] = [];
  let height = y;
  for (let tick = 0; height !== goal && tick < 2000; tick++) {
    height = climbStep(height, goal, tick);
    heights.push(height);
  }
  return heights;
}

/** 🫙️ Whether a box has an area and holds a point, its edges included: what a ladder stands on or leans against, which is not in its way. */
function holding(rect: Rect, point: Point): boolean {
  return rect.width > 0 && rect.height > 0 && rect.x <= point.x && point.x <= rect.x + rect.width && rect.y <= point.y && point.y <= rect.y + rect.height;
}

/** 🛤️ The x of a segment at height `y`. */
function along(from: Point, to: Point, y: number): number {
  return from.x + ((to.x - from.x) * (y - from.y)) / (to.y - from.y);
}

const FLOOR = perch("floor", 0, 640, 480);
const CARD = perch("card", 306, 494, 300);
const CARD_BOX = solid(300, 500, 300, 480);
const LEFT = pitch("card-left", "card", -1, 300, 300, 480);
const RIGHT = pitch("card-right", "card", 1, 500, 300, 480);

describe("the stage of this suite", () => {
  it("is what the terrain cuts out of a card that stands on the floor", () => {
    const walls = [{ id: "card-left", surface: "card", side: -1 as const, x: 300, y0: 300, y1: 480 }, { id: "card-right", surface: "card", side: 1 as const, x: 500, y0: 300, y1: 480 }];
    expect(wallsOf(walls, [CARD_BOX], 640, 480, 48, 72)).toEqual([LEFT, RIGHT]);
  });
});

describe("hoistPath", () => {
  const from = { x: 280, y: 340.8 };
  const to = { x: 328, y: 300 };

  it("begins on its start and ends on its end, exactly, and stays there beyond", () => {
    expect(hoistPath(from, to, 48, 0)).toEqual(from);
    expect(hoistPath(from, to, 48, -0.5)).toEqual(from);
    expect(hoistPath(from, to, 48, 1)).toEqual(to);
    expect(hoistPath(from, to, 48, 1.5)).toEqual(to);
  });

  it("rises before it crosses over, tops its end by the hump and settles onto it", () => {
    const path = Array.from({ length: MANTLE_TICKS + 1 }, (_, tick) => hoistPath(from, to, 48, tick / MANTLE_TICKS));
    for (let tick = 1; tick <= MANTLE_TICKS; tick++) expect(path[tick]!.x, `tick ${tick}`).toBeGreaterThanOrEqual(path[tick - 1]!.x);
    for (const point of path.filter((_, tick) => tick / MANTLE_TICKS <= 0.35)) expect(point.x).toBe(from.x);
    const apex = Math.min(...path.map((point) => point.y));
    expect(apex).toBeGreaterThan(to.y - HOIST_HUMP * 48 - 1e-9);
    expect(apex).toBeLessThan(to.y - HOIST_HUMP * 48 + 0.2);
    const top = path.findIndex((point) => point.y === apex);
    for (let tick = 1; tick <= top; tick++) expect(path[tick]!.y, `tick ${tick}`).toBeLessThan(path[tick - 1]!.y);
    for (let tick = top + 1; tick <= MANTLE_TICKS; tick++) expect(path[tick]!.y, `tick ${tick}`).toBeGreaterThan(path[tick - 1]!.y);
  });

  it("is the three eased parts of its definition at every phase", () => {
    for (let step = 0; step <= 200; step++) {
      const phase = step / 200;
      const found = hoistPath(from, to, 48, phase);
      const rise = smoothstep(phase / 0.65);
      const across = smoothstep((phase - 0.35) / 0.65);
      const settle = smoothstep((phase - 0.65) / 0.35);
      expect(Math.abs(found.x - (from.x + (to.x - from.x) * across)), `phase ${phase}`).toBeLessThan(1e-9);
      expect(Math.abs(found.y - (from.y + (to.y - 4.8 - from.y) * rise + 4.8 * settle)), `phase ${phase}`).toBeLessThan(1e-9);
    }
  });

  it("mirrors with the side and leads down as well as up", () => {
    const mirrored = hoistPath({ x: 520, y: 340.8 }, { x: 472, y: 300 }, 48, 0.6);
    const found = hoistPath(from, to, 48, 0.6);
    expect(mirrored.x - 400).toBeCloseTo(400 - found.x, 9);
    expect(mirrored.y).toBe(found.y);
    const down = hoistPath({ x: 100, y: 200 }, { x: 130, y: 260 }, 40, 0.5);
    expect(down.y).toBeGreaterThan(200);
    expect(down.y).toBeLessThan(260);
  });
});

describe("holds on a wall", () => {
  it("clings half a width out on the air side, between the rim and the foot of the pitch", () => {
    expect(clingOf(LEFT, PET)).toBe(280);
    expect(clingOf(RIGHT, PET)).toBe(520);
    expect(ledgeOf(LEFT, PET)).toBe(300 + 20 + MANTLE_INSET);
    expect(ledgeOf(RIGHT, PET)).toBe(500 - 20 - MANTLE_INSET);
    expect(rimOf(LEFT, PET)).toBe(300 + HAND_HEIGHT * 48);
    expect(footOf(LEFT, PET)).toBe(480 - GRIP_BITE + HAND_HEIGHT * 48);
    expect(clings(LEFT, rimOf(LEFT, PET), PET)).toBe(true);
    expect(clings(LEFT, footOf(LEFT, PET), PET)).toBe(true);
    expect(clings(LEFT, 400, PET)).toBe(true);
    expect(clings(LEFT, rimOf(LEFT, PET) - 0.25, PET)).toBe(false);
    expect(clings(LEFT, footOf(LEFT, PET) + 0.25, PET)).toBe(false);
  });

  it("is crowned by the perch of its surface at its top that carries the ledge", () => {
    expect(crowns(CARD, LEFT, PET)).toBe(true);
    expect(crowns(CARD, RIGHT, PET)).toBe(true);
    expect(crowns(FLOOR, LEFT, PET)).toBe(false);
    expect(crowns(perch("card", 306, 494, 299.5), LEFT, PET)).toBe(false);
    expect(crowns(perch("other", 306, 494, 300), LEFT, PET)).toBe(false);
    expect(crowns(perch("card", 328, 494, 300), LEFT, PET)).toBe(true);
    expect(crowns(perch("card", 328.5, 494, 300), LEFT, PET)).toBe(false);
    expect(crowns(CARD, pitch("card-left", "card", -1, 300, 320, 480), PET)).toBe(false);
    expect(rimFor(LEFT, [FLOOR, perch("card", 400, 494, 300), CARD], PET)).toBe(CARD);
    expect(rimFor(LEFT, [FLOOR, perch("card", 400, 494, 300)], PET)).toBeNull();
    expect(rimFor(LEFT, [], PET)).toBeNull();
  });

  it("is taken from a perch at its foot where the actor can stand beside it with its hands on it", () => {
    expect(gripFor(FLOOR, LEFT, PET)).toEqual({ x: 280, y: 480, over: false });
    expect(gripFor(FLOOR, RIGHT, PET)).toEqual({ x: 520, y: 480, over: false });
    expect(gripFor(perch("shelf", 100, 280, 420), LEFT, PET)).toEqual({ x: 280, y: 420, over: false });
    expect(gripFor(perch("shelf", 100, 279.5, 420), LEFT, PET)).toBeNull();
    expect(gripFor(perch("shelf", 280.5, 296, 420), LEFT, PET)).toBeNull();
    expect(gripFor(perch("floor", 0, 640, 512.8), LEFT, PET)).toEqual({ x: 280, y: 512.8, over: false });
    expect(gripFor(perch("floor", 0, 640, 513), LEFT, PET)).toBeNull();
    expect(gripFor(perch("shelf", 100, 290, rimOf(LEFT, PET)), LEFT, PET)).toEqual({ x: 280, y: rimOf(LEFT, PET), over: false });
    expect(gripFor(perch("shelf", 100, 290, 340), LEFT, PET)).toBeNull();
    expect(gripFor(FLOOR, LEFT, LARGE)).toEqual({ x: 277, y: 480, over: false });
  });

  it("is taken over the rim from the perch that crowns it, when the pitch is long enough to hold", () => {
    expect(gripFor(CARD, LEFT, PET)).toEqual({ x: 280, y: 340.8, over: true });
    expect(gripFor(CARD, RIGHT, PET)).toEqual({ x: 520, y: 340.8, over: true });
    expect(gripFor(CARD, pitch("card-left", "card", -1, 300, 300, 308), PET)).toEqual({ x: 280, y: 340.8, over: true });
    expect(gripFor(CARD, pitch("card-left", "card", -1, 300, 300, 307.5), PET)).toBeNull();
    expect(gripFor(perch("card", 340, 494, 300), LEFT, PET)).toBeNull();
    expect(gripFor(CARD, pitch("card-left", "card", -1, 300, 330, 480), PET)).toBeNull();
  });

  it("carries its climber through a survey while the wall stays within the tolerance and free under its hands", () => {
    const moved = (dx: number, y0 = 300, y1 = 480) => [RIGHT, pitch("card-left", "card", -1, 300 + dx, y0, y1)];
    expect(wallHolds(LEFT, moved(0), 400, PET)).toEqual(LEFT);
    expect(wallHolds(LEFT, moved(WALL_FOLLOW), 400, PET)).toEqual(pitch("card-left", "card", -1, 306, 300, 480));
    expect(wallHolds(LEFT, moved(-WALL_FOLLOW), 400, PET)).not.toBeNull();
    expect(wallHolds(LEFT, moved(WALL_FOLLOW + 0.5), 400, PET)).toBeNull();
    expect(wallHolds(LEFT, moved(0, 300, 360), 400, PET)).toBeNull();
    expect(wallHolds(LEFT, moved(0, 300, 367.2), 400, PET)).not.toBeNull();
    expect(wallHolds(LEFT, moved(0, 360, 480), 400, PET)).toBeNull();
    expect(wallHolds(LEFT, [RIGHT], 400, PET)).toBeNull();
    expect(wallHolds(LEFT, [pitch("card-left", "card", 1, 300, 300, 480)], 400, PET)).toBeNull();
    expect(wallHolds(LEFT, [pitch("card-left", "card", -1, 300, 300, 340), pitch("card-left", "card", -1, 300, 350, 480)], 400, PET)).toEqual(pitch("card-left", "card", -1, 300, 350, 480));
    expect(wallHolds(LEFT, [], 400, PET)).toBeNull();
  });

  it("throws whoever lost it away from the wall and up", () => {
    expect(slipOf(LEFT)).toEqual({ vx: -SLIP_PUSH, vy: -SLIP_LIFT });
    expect(slipOf(RIGHT)).toEqual({ vx: SLIP_PUSH, vy: -SLIP_LIFT });
  });
});

describe("gripStep", () => {
  it("spends one per tick of climbing, a quarter per tick of hanging, and never falls below nothing", () => {
    expect(gripStep(GRIP_BUDGET, "climb")).toBe(383);
    expect(gripStep(GRIP_BUDGET, "hang")).toBe(383.75);
    expect(gripStep(0.5, "climb")).toBe(0);
    expect(gripStep(0, "hang")).toBe(0);
    let grip = GRIP_BUDGET;
    let ticks = 0;
    while (grip > 0) {
      grip = gripStep(grip, "climb");
      ticks++;
    }
    expect(ticks).toBe(GRIP_BUDGET);
    for (grip = GRIP_BUDGET, ticks = 0; grip > 0; ticks++) grip = gripStep(grip, "hang");
    expect(ticks).toBe(4 * GRIP_BUDGET);
  });

  it("gives two per tick of rest back, up to the budget", () => {
    expect(gripStep(100, "rest")).toBe(102);
    expect(gripStep(GRIP_BUDGET - 1, "rest")).toBe(GRIP_BUDGET);
    expect(gripStep(GRIP_BUDGET, "rest")).toBe(GRIP_BUDGET);
    let grip = 0;
    let ticks = 0;
    while (grip < GRIP_BUDGET) {
      grip = gripStep(grip, "rest");
      ticks++;
    }
    expect(ticks).toBe(GRIP_BUDGET / 2);
  });
});

describe("climbStep", () => {
  it("gathers its speed over the ramp and then climbs at the full speed, up slower than down", () => {
    const up = climbed(480, 340.8);
    const down = climbed(340.8, 480);
    for (let tick = 0; tick < 12; tick++) {
      const eased = smoothstep((tick + 1) / CLIMB_RAMP);
      expect(Math.abs((tick === 0 ? 480 : up[tick - 1]!) - up[tick]! - (CLIMB_RISE * eased) / 64), `up ${tick}`).toBeLessThan(1e-9);
      expect(Math.abs(down[tick]! - (tick === 0 ? 340.8 : down[tick - 1]!) - (CLIMB_DESCENT * eased) / 64), `down ${tick}`).toBeLessThan(1e-9);
    }
    expect(Math.abs(480 - up[5]! - (3.5 * CLIMB_RISE) / 64)).toBeLessThan(1e-9);
    expect(up.length).toBeGreaterThan(down.length);
  });

  it("never overshoots, ends on its goal exactly and stays there", () => {
    for (const [from, goal] of [[480, 340.8], [340.8, 480], [400, 399.9], [400, 400.3], [512.8, 340.8]] as const) {
      const heights = climbed(from, goal);
      expect(heights[heights.length - 1]).toBe(goal);
      for (const height of heights) expect((height - goal) * (from - goal)).toBeGreaterThanOrEqual(0);
      expect(climbStep(goal, goal, heights.length)).toBe(goal);
      expect(climbTicks(from, goal)).toBe(heights.length);
    }
    expect(climbTicks(400, 400)).toBe(0);
  });

  it("takes as long as its closed form says and gives up beyond the grip of a rested actor", () => {
    for (const distance of [0.1, 1.5, 1.640625, 2, 10, 92, 139.2, 150]) {
      const ramp = (3.5 * CLIMB_RISE) / 64;
      const expected = distance <= ramp ? climbed(480, 480 - distance).length : CLIMB_RAMP + Math.ceil((distance - ramp) / (CLIMB_RISE / 64) - 1e-9);
      expect(climbTicks(480, 480 - distance), `distance ${distance}`).toBe(expected);
    }
    expect(climbTicks(480, 480 - 178.8)).toBe(GRIP_BUDGET);
    expect(climbTicks(480, 480 - 181)).toBe(GRIP_BUDGET + 1);
    expect(climbTicks(480, -10000)).toBe(GRIP_BUDGET + 1);
    expect(climbTicks(480, Number.NEGATIVE_INFINITY)).toBe(GRIP_BUDGET + 1);
    expect(climbTicks(300, Number.NaN)).toBe(GRIP_BUDGET + 1);
  });
});

describe("climbPhase", () => {
  it("runs one cycle per two holds, counted up the wall from the top of the pitch", () => {
    const stride = 2 * GRIP_SPACING * 48;
    const turned = (y: number, expected: number) => Math.abs(((climbPhase(LEFT, y, PET) - expected + 1.5) % 1) - 0.5);
    expect(climbPhase(LEFT, 300, PET)).toBe(0);
    expect(turned(300 + stride, 0)).toBeLessThan(1e-12);
    expect(turned(300 + 3 * stride, 0)).toBeLessThan(1e-12);
    expect(turned(300 + stride / 4, 0.75)).toBeLessThan(1e-12);
    expect(turned(300 + stride / 2, 0.5)).toBeLessThan(1e-12);
    expect(turned(300 - stride / 4, 0.25)).toBeLessThan(1e-12);
    expect(turned(300 + stride / 4, 0.25)).toBeGreaterThan(0.4);
    for (let y = 250; y < 520; y += 0.37) {
      const phase = climbPhase(LEFT, y, PET);
      expect(phase).toBeGreaterThanOrEqual(0);
      expect(phase).toBeLessThan(1);
    }
  });

  it("advances by the distance climbed, never by the time it took, so the hands do not slide", () => {
    const heights = [480, ...climbed(480, 340.8)];
    for (let tick = 1; tick < heights.length; tick++) {
      const advanced = climbPhase(LEFT, heights[tick]!, PET) - climbPhase(LEFT, heights[tick - 1]!, PET);
      const expected = (heights[tick - 1]! - heights[tick]!) / (2 * GRIP_SPACING * 48);
      expect(Math.abs(advanced - Math.floor(advanced) - expected), `tick ${tick}`).toBeLessThan(1e-9);
    }
    expect(climbPhase(LEFT, 400, PET)).toBe(climbPhase(LEFT, 400, PET));
    expect(climbPhase(pitch("moved", "card", -1, 304, 310, 490), 410, PET)).toBe(climbPhase(LEFT, 400, PET));
  });
});

describe("slideStep", () => {
  it("starts at the slip speed, gains speed up to the fastest slide and moves by the new speed", () => {
    expect(slideStep(340.8, 0, 480)).toEqual({ y: 340.8 + (SLIDE_START + SLIDE_GAIN / 64) / 64, vy: SLIDE_START + SLIDE_GAIN / 64 });
    expect(slideStep(340.8, 100, 480)).toEqual({ y: 340.8 + 109.375 / 64, vy: 109.375 });
    expect(slideStep(340.8, SLIDE_SPEED, 480)).toEqual({ y: 340.8 + 2.5, vy: SLIDE_SPEED });
    expect(slideStep(340.8, 5000, 480)).toEqual({ y: 340.8 + 2.5, vy: SLIDE_SPEED });
    let slide = { y: 340.8, vy: 0 };
    let ticks = 0;
    while (slide.vy < SLIDE_SPEED) {
      slide = slideStep(slide.y, slide.vy, 10000);
      ticks++;
      if (slide.vy < SLIDE_SPEED) expect(Math.abs(slide.vy - (SLIDE_START + (ticks * SLIDE_GAIN) / 64))).toBeLessThan(1e-9);
    }
    expect(ticks).toBe(Math.ceil(((SLIDE_SPEED - SLIDE_START) * 64) / SLIDE_GAIN));
  });

  it("ends on its floor exactly and stays there", () => {
    let slide = { y: 340.8, vy: 0 };
    let ticks = 0;
    while (slide.y !== 480 && ticks < 500) {
      const next = slideStep(slide.y, slide.vy, 480);
      expect(next.y).toBeGreaterThan(slide.y);
      expect(next.y).toBeLessThanOrEqual(480);
      slide = next;
      ticks++;
    }
    expect(slide.y).toBe(480);
    expect(ticks).toBeGreaterThan(56);
    expect(ticks).toBeLessThan(70);
    expect(slideStep(480, slide.vy, 480).y).toBe(480);
  });
});

describe("mantlePath", () => {
  it("leads from the highest hold over the rim onto the ledge, on either side", () => {
    expect(mantlePath(LEFT, PET, 0)).toEqual({ x: 280, y: 340.8 });
    expect(mantlePath(LEFT, PET, 1)).toEqual({ x: 328, y: 300 });
    expect(mantlePath(RIGHT, PET, 0)).toEqual({ x: 520, y: 340.8 });
    expect(mantlePath(RIGHT, PET, 1)).toEqual({ x: 472, y: 300 });
    for (let tick = 0; tick <= MANTLE_TICKS; tick++) {
      const left = mantlePath(LEFT, PET, tick / MANTLE_TICKS);
      const right = mantlePath(RIGHT, PET, tick / MANTLE_TICKS);
      expect(left).toEqual(hoistPath({ x: 280, y: 340.8 }, { x: 328, y: 300 }, 48, tick / MANTLE_TICKS));
      expect(Math.abs(right.x - 400 - (400 - left.x))).toBeLessThan(1e-9);
      expect(right.y).toBe(left.y);
      expect(left.x <= 300 || left.y <= 300.5, `tick ${tick}`).toBe(true);
    }
  });
});

describe("ladderFor", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const wall = pitch("shelf-left", "shelf", -1, 300, 380, 480);
  const box = solid(300, 500, 380, 480);

  it("stands at the lean of real ladders with its top a little under the rim", () => {
    const ladder = ladderFor(FLOOR, shelf, wall, [box], PET)!;
    expect(ladder).toEqual({ wall: "shelf-left", surface: "floor", side: -1, foot: { x: 300 - LADDER_LEAN * 93, y: 480 }, top: { x: 300, y: 380 + LADDER_TUCK } });
    expect(ladderLean(ladder)).toBe(LADDER_LEAN);
    expect(Math.abs(ladderLength(ladder) - Math.hypot(23.25, 93))).toBeLessThan(1e-12);
    expect(ladderRungs(ladder)).toBe(Math.floor(Math.hypot(23.25, 93) / RUNG_SPACING));
    expect(ladderRungs(ladder)).toBe(9);
    const mirrored = ladderFor(FLOOR, shelf, pitch("shelf-right", "shelf", 1, 500, 380, 480), [box], PET)!;
    expect(mirrored.foot).toEqual({ x: 500 + LADDER_LEAN * 93, y: 480 });
    expect(mirrored.side).toBe(1);
  });

  it("moves its foot as near to that lean as its perch lets it, within the steepest and the flattest lean", () => {
    const short = (x0: number, x1: number) => ladderFor(perch("floor", x0, x1, 480), shelf, wall, [box], PET);
    expect(short(0, 640)!.foot.x).toBe(276.75);
    expect(short(0, 275)!.foot.x).toBe(275 - LADDER_FOOTING);
    expect(ladderLean(short(0, 275)!)).toBeCloseTo(35 / 93, 12);
    expect(short(0, 273)!.foot.x).toBe(263);
    expect(short(0, 272.5)).toBeNull();
    expect(short(270, 640)!.foot.x).toBe(270 + LADDER_FOOTING);
    expect(ladderLean(short(270, 640)!)).toBeCloseTo(20 / 93, 12);
    expect(short(276.5, 640)!.foot.x).toBe(286.5);
    expect(short(277.5, 640)).toBeNull();
    expect(short(260, 280)!.foot.x).toBe(270);
    expect(short(260, 279.5)).toBeNull();
    expect(short(310, 640)).toBeNull();
  });

  it("needs a rise between the shortest and the tallest ladder of its owner", () => {
    const from = (y: number, size: Size) => ladderFor(perch("floor", 0, 640, y), shelf, wall, [], size);
    expect(from(387 + LADDER_SHORT * 48 + 0.5, PET)).not.toBeNull();
    expect(from(387 + LADDER_SHORT * 48 - 0.5, PET)).toBeNull();
    expect(from(387 + LADDER_TALL * 48 - 0.5, PET)).not.toBeNull();
    expect(from(387 + LADDER_TALL * 48 + 0.5, PET)).toBeNull();
    expect(from(387 + LADDER_SHORT * 48 + 0.5, LARGE)).toBeNull();
    expect(from(387 + LADDER_SHORT * 56 + 0.5, LARGE)).not.toBeNull();
    expect(from(387 + LADDER_SHORT * 40 + 0.5, SMALL)).not.toBeNull();
    expect(from(387 + LADDER_TALL * 48 + 0.5, LARGE)).not.toBeNull();
    expect(from(380, PET)).toBeNull();
    expect(from(300, PET)).toBeNull();
  });

  it("leans only against a pitch that begins at a rim someone can stand on and reaches down to its contact", () => {
    expect(ladderFor(FLOOR, perch("shelf", 340, 494, 380), wall, [], PET)).toBeNull();
    expect(ladderFor(FLOOR, perch("other", 306, 494, 380), wall, [], PET)).toBeNull();
    expect(ladderFor(FLOOR, shelf, pitch("shelf-left", "shelf", -1, 300, 390, 480), [], PET)).toBeNull();
    expect(ladderFor(FLOOR, shelf, pitch("shelf-left", "shelf", -1, 300, 380, 387), [], PET)).not.toBeNull();
    expect(ladderFor(FLOOR, shelf, pitch("shelf-left", "shelf", -1, 300, 380, 386.5), [], PET)).toBeNull();
  });

  it("passes over what it stands on and what it leans against, and is blocked by anything else within its girth", () => {
    const stage = (blocker: Rect) => ladderFor(FLOOR, shelf, wall, [box, keepout(0, 480, 640, 40), blocker], PET);
    expect(stage(keepout(0, 0, 0, 0))).not.toBeNull();
    expect(stage(keepout(280, 400, 10, 30))).toBeNull();
    expect(stage(keepout(240, 400, 30, 30))).not.toBeNull();
    const ladder = stage(keepout(0, 0, 0, 0))!;
    const reach = along(ladder.foot, ladder.top, 430);
    expect(stage(keepout(reach - LADDER_GIRTH - 20, 420, 20, 20))).toBeNull();
    expect(stage(keepout(reach - LADDER_GIRTH - 40, 420, 20, 20))).not.toBeNull();
    expect(stage(keepout(200, 440, 60, 34))).not.toBeNull();
    expect(stage(keepout(200, 440, 71.5, 34.5))).toBeNull();
  });

  it("keeps every law on generated stages", () => {
    const next = stream(41);
    let granted = 0;
    let refused = 0;
    for (let stage = 0; stage < STAGES; stage++) {
      const size = [PET, SMALL, LARGE][next(3)]!;
      const side = next(2) === 0 ? -1 : 1;
      const x = 200 + next(800) / 4;
      const rim = 150 + next(600) / 4;
      const high = perch("high", side < 0 ? x + 6 : x - 180, side < 0 ? x + 180 : x - 6, rim);
      const face = pitch("high-side", "high", side, x, rim, rim + 20 + next(800) / 4);
      const low = perch("low", x - 150 + next(600) / 4, x + next(600) / 4, rim + 20 + next(900) / 4);
      const keepouts = [solid(Math.min(x, x - side * 186), Math.max(x, x - side * 186), rim, rim + 300), ...Array.from({ length: next(3) }, () => keepout(x - 120 + next(960) / 4, rim - 40 + next(1200) / 4, next(160) / 4, next(160) / 4))];
      const ladder = ladderFor(low, high, face, keepouts, size);
      if (ladder === null) {
        refused++;
        continue;
      }
      granted++;
      const label = `stage ${stage}`;
      const rise = ladder.foot.y - ladder.top.y;
      expect(ladder.top, label).toEqual({ x, y: rim + LADDER_TUCK });
      expect(ladder.foot.y, label).toBe(low.y);
      expect(ladder.foot.x >= low.x0 + LADDER_FOOTING && ladder.foot.x <= low.x1 - LADDER_FOOTING, label).toBe(true);
      expect((ladder.foot.x - x) * side, label).toBeGreaterThan(0);
      expect(rise >= LADDER_SHORT * size.height && rise <= LADDER_TALL * size.height, label).toBe(true);
      expect(ladderLean(ladder) >= LADDER_STEEP - 1e-12 && ladderLean(ladder) <= LADDER_FLAT + 1e-12, label).toBe(true);
      expect(Math.abs(ladderLength(ladder) - Math.hypot(ladder.foot.x - x, rise)), label).toBeLessThan(1e-9);
      expect(ladderRungs(ladder), label).toBe(Math.floor(Math.hypot(ladder.foot.x - x, rise) / RUNG_SPACING));
      for (const blocker of keepouts) if (!holding(blocker, ladder.foot) && !holding(blocker, ladder.top)) expect(segmentHits(ladder.foot, ladder.top, blocker, LADDER_GIRTH), label).toBe(false);
      expect(ladderHolds(ladder, [low, high], [face], keepouts, size), label).toEqual(ladder);
    }
    expect(granted).toBeGreaterThan(STAGES / 12);
    expect(refused).toBeGreaterThan(STAGES / 4);
  });
});

describe("climbing a ladder", () => {
  const ladder: Ladder = { wall: "shelf-left", surface: "floor", side: -1, foot: { x: 276.75, y: 480 }, top: { x: 300, y: 387 } };
  const length = Math.hypot(23.25, 93);

  it("gathers its speed over the ramp, climbs up slower than down and ends on its goal exactly", () => {
    const exit = ladderExit(ladder, PET);
    expect(Math.abs(exit - (length - LADDER_EXIT * 48))).toBeLessThan(1e-12);
    let travel = 0;
    let ticks = 0;
    while (travel !== exit && ticks < 1000) {
      const next = ladderStep(travel, exit, ticks);
      expect(Math.abs(next - travel - Math.min((LADDER_RISE * smoothstep((ticks + 1) / 6)) / 64, exit - travel)), `tick ${ticks}`).toBeLessThan(1e-9);
      travel = next;
      ticks++;
    }
    expect(travel).toBe(exit);
    expect(ticks).toBe(6 + Math.ceil((exit - (3.5 * LADDER_RISE) / 64) / (LADDER_RISE / 64)));
    expect(ladderStep(exit, exit, ticks)).toBe(exit);
    let down = 0;
    for (travel = exit; travel !== 0 && down < 1000; down++) travel = ladderStep(travel, 0, down);
    expect(travel).toBe(0);
    expect(down).toBe(6 + Math.ceil((exit - (3.5 * LADDER_DESCENT) / 64) / (LADDER_DESCENT / 64)));
    expect(down).toBeLessThan(ticks);
  });

  it("carries its climber along the rails, from the foot itself to the top itself", () => {
    expect(ladderAt(ladder, 0)).toEqual(ladder.foot);
    expect(ladderAt(ladder, -3)).toEqual(ladder.foot);
    expect(ladderAt(ladder, length)).toEqual(ladder.top);
    expect(ladderAt(ladder, length + 10)).toEqual(ladder.top);
    for (let travel = 0.5; travel < length; travel += 3.7) {
      const feet = ladderAt(ladder, travel);
      expect(Math.abs(Math.hypot(feet.x - ladder.foot.x, feet.y - ladder.foot.y) - travel)).toBeLessThan(1e-9);
      expect(Math.abs(Math.hypot(feet.x - ladder.top.x, feet.y - ladder.top.y) - (length - travel))).toBeLessThan(1e-9);
    }
    expect(ladderExit({ ...ladder, foot: { x: 298, y: 395 } }, PET)).toBe(0);
  });

  it("puts hands and feet on the rungs: one cycle of the clip per two rungs", () => {
    expect(ladderPhase(0)).toBe(0);
    expect(ladderPhase(RUNG_SPACING)).toBe(0.5);
    expect(ladderPhase(2 * RUNG_SPACING)).toBe(0);
    expect(ladderPhase(5.25)).toBe(0.25);
    expect(ladderPhase(47.25)).toBe(0.25);
    for (let travel = 0; travel < 120; travel += 0.41) expect(ladderPhase(travel) >= 0 && ladderPhase(travel) < 1).toBe(true);
  });

  it("lets its climber off on the rim, inside it, whoever climbs", () => {
    expect(ladderLanding(ladder, PET)).toEqual({ x: 300 + 20 + MANTLE_INSET, y: 380 });
    expect(ladderLanding(ladder, LARGE)).toEqual({ x: 300 + 23 + MANTLE_INSET, y: 380 });
    expect(ladderLanding({ ...ladder, side: 1, top: { x: 500, y: 387 } }, PET)).toEqual({ x: 500 - 20 - MANTLE_INSET, y: 380 });
  });

  it("drops whoever is high on it when it topples and lets whoever is low step off", () => {
    expect(spillOf(ladder, 480, PET)).toBeNull();
    expect(spillOf(ladder, 480 - TOPPLE_STEP * 48 + 0.5, PET)).toBeNull();
    expect(spillOf(ladder, 480 - TOPPLE_STEP * 48, PET)).toEqual({ vx: -TOPPLE_PUSH, vy: 0 });
    expect(spillOf({ ...ladder, side: 1 }, 400, PET)).toEqual({ vx: TOPPLE_PUSH, vy: 0 });
    expect(spillOf(ladder, 480 - TOPPLE_STEP * 48, LARGE)).toBeNull();
  });
});

describe("ladderHolds", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const wall = pitch("shelf-left", "shelf", -1, 300, 380, 480);
  const box = solid(300, 500, 380, 480);
  const ladder = ladderFor(FLOOR, shelf, wall, [box], PET)!;
  const moved = (dx: number, dy: number) => ({ perches: [FLOOR, perch("shelf", 306 + dx, 494 + dx, 380 + dy)], pitches: [pitch("shelf-left", "shelf", -1, 300 + dx, 380 + dy, 480 + dy)], keepouts: [solid(300 + dx, 500 + dx, 380 + dy, 480 + dy)] });

  it("stands as it is when nothing moved", () => {
    expect(ladderHolds(ladder, [FLOOR, shelf], [wall], [box], PET)).toEqual(ladder);
  });

  it("follows its wall with its top while the foot stays", () => {
    for (const [dx, dy] of [[3, 0], [0, -5], [-4.5, 6], [0, LADDER_FOLLOW], [LADDER_FOLLOW, 0]] as const) {
      const stage = moved(dx, dy);
      expect(ladderHolds(ladder, stage.perches, stage.pitches, stage.keepouts, PET), `${dx} ${dy}`).toEqual({ ...ladder, top: { x: 300 + dx, y: 387 + dy } });
    }
  });

  it("topples when its top moves too far, leaves or loses its lean — and stands on, leaning on its wall, when the perch on the rim is gone", () => {
    for (const [dx, dy] of [[0, LADDER_FOLLOW + 0.5], [LADDER_FOLLOW + 0.5, 0], [6, 6], [-40, 0]] as const) {
      const stage = moved(dx, dy);
      expect(ladderHolds(ladder, stage.perches, stage.pitches, stage.keepouts, PET), `${dx} ${dy}`).toBeNull();
    }
    expect(ladderHolds(ladder, [FLOOR, shelf], [], [box], PET)).toBeNull();
    expect(ladderHolds(ladder, [FLOOR, shelf], [pitch("shelf-left", "shelf", 1, 300, 380, 480)], [box], PET)).toBeNull();
    expect(ladderHolds(ladder, [FLOOR, shelf], [pitch("other", "shelf", -1, 300, 380, 480)], [box], PET)).toBeNull();
    expect(ladderHolds(ladder, [FLOOR], [wall], [box], PET)).toEqual(ladder);
    expect(ladderHolds(ladder, [FLOOR, perch("shelf", 400, 494, 380)], [wall], [box], PET)).toEqual(ladder);
    const steep = { ...ladder, foot: { x: 300 - LADDER_STEEP * 93 - 0.5, y: 480 } };
    const nearer = moved(-1, 0);
    expect(ladderHolds(steep, [FLOOR, shelf], [wall], [box], PET)).toEqual(steep);
    expect(ladderHolds(steep, nearer.perches, nearer.pitches, nearer.keepouts, PET)).toBeNull();
  });

  it("topples when the perch under it moves too far, shrinks from under its foot or vanishes", () => {
    const on = (low: Perch) => ladderHolds(ladder, [low, shelf], [wall], [box], PET);
    expect(on(perch("floor", 0, 640, 480 + LADDER_SHIFT))).toEqual({ ...ladder, foot: { x: ladder.foot.x, y: 492 } });
    expect(on(perch("floor", 0, 640, 480 - 6))).toEqual({ ...ladder, foot: { x: ladder.foot.x, y: 474 } });
    expect(on(perch("floor", 0, 640, 480 + LADDER_SHIFT + 0.5))).toBeNull();
    expect(on(perch("floor", 0, ladder.foot.x + LADDER_FOOTING, 480))).toEqual(ladder);
    expect(on(perch("floor", 0, ladder.foot.x + LADDER_FOOTING - 0.5, 480))).toBeNull();
    expect(on(perch("ground", 0, 640, 480))).toBeNull();
    expect(ladderHolds(ladder, [shelf], [wall], [box], PET)).toBeNull();
    expect(ladderHolds(ladder, [perch("floor", 0, 100, 480), FLOOR, shelf], [wall], [box], PET)).toEqual(ladder);
  });

  it("topples when something gets in its way", () => {
    expect(ladderHolds(ladder, [FLOOR, shelf], [wall], [box, keepout(270, 400, 30, 30)], PET)).toBeNull();
    expect(ladderHolds(ladder, [FLOOR, shelf], [wall], [box, keepout(100, 400, 30, 30)], PET)).toEqual(ladder);
  });
});

describe("ladderTo", () => {
  const card = pitch("card-left", "card", -1, 300, 200, 300);

  it("leans a ladder against a wall to take hold of it from its exit: its top at the lower end of the pitch, its foot at the lean of real ladders, the exit within the hands' reach — and follows that wall like any ladder", () => {
    const ladder = ladderTo(FLOOR, card, [], PET)!;
    expect(ladder).toEqual({ wall: "card-left", surface: "floor", side: -1, foot: { x: 300 - LADDER_LEAN * (480 - 300), y: 480 }, top: { x: 300, y: 300 } });
    const exit = ladderAt(ladder, ladderExit(ladder, PET));
    expect(clings(card, exit.y, PET)).toBe(true);
    expect(ladderHolds(ladder, [FLOOR], [card], [], PET)).toEqual(ladder);
    expect(ladderHolds(ladder, [FLOOR], [pitch("card-left", "card", -1, 300, 205, 305)], [], PET)).toEqual({ ...ladder, top: { x: 300, y: 305 } });
    expect(ladderHolds(ladder, [FLOOR], [pitch("card-left", "card", -1, 300, 205, 310)], [], PET)).toBeNull();
    expect(ladderTo(FLOOR, card, [solid(300, 500, 200, 480)], PET)).toEqual(ladder);
  });

  it("leans none that would be too tall or too short, or whose exit leaves the hands below the wall", () => {
    expect(ladderTo(FLOOR, pitch("high", "card", -1, 300, 0, 60), [], PET)).toBeNull();
    expect(ladderTo(FLOOR, pitch("high", "card", -1, 300, 0, 60), [], LARGE)).not.toBeNull();
    expect(ladderTo(FLOOR, pitch("low", "card", -1, 300, 440, 470), [], PET)).toBeNull();
    expect(ladderTo(FLOOR, pitch("stub", "card", -1, 300, 280, 300), [], PET)).toBeNull();
    expect(ladderTo(FLOOR, card, [keepout(270, 400, 30, 30)], PET)).toBeNull();
  });
});

describe("shotFor", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const box = solid(300, 500, 380, 420);
  const from = (x: number, perches: readonly Perch[] = [FLOOR, shelf], keepouts: readonly Rect[] = [box], size: Size = PET) => shotFor({ x, y: 480 }, perches, keepouts, size);

  it("aims from the muzzle in front of the feet at a point just above the corner of the edge", () => {
    const shot = from(280)!;
    expect(shot.surface).toBe("shelf");
    expect(shot.facing).toBe(1);
    expect(shot.muzzle).toEqual({ x: 280 + MUZZLE_FORWARD * 40, y: 480 - MUZZLE_HEIGHT * 48 });
    expect(shot.hook).toEqual({ x: 306 + HOOK_INSET, y: 380 - HOOK_LIFT });
    expect(Math.abs(shot.length - Math.hypot(shot.hook.x - shot.muzzle.x, shot.hook.y - shot.muzzle.y))).toBeLessThan(1e-12);
    const mirrored = from(520)!;
    expect(mirrored.facing).toBe(-1);
    expect(mirrored.muzzle).toEqual({ x: 520 - MUZZLE_FORWARD * 40, y: 480 - MUZZLE_HEIGHT * 48 });
    expect(mirrored.hook).toEqual({ x: 494 - HOOK_INSET, y: 379 });
    expect(mirrored.length).toBeCloseTo(shot.length, 9);
  });

  it("takes the point of the edge nearest to the feet when that is nearer than both corners, and the middle of an edge too short for two", () => {
    expect(from(400)!.hook).toEqual({ x: 400, y: 379 });
    expect(from(400)!.facing).toBe(1);
    expect(from(400)!.reel).toBe("zip");
    expect(from(440)!.hook).toEqual({ x: 440, y: 379 });
    expect(from(311)!.hook).toEqual({ x: 310, y: 379 });
    expect(from(311)!.facing).toBe(-1);
    expect(from(309)!.hook).toEqual({ x: 310, y: 379 });
    expect(shotFor({ x: 400, y: 480 }, [perch("stub", 380, 420, 380)], [], PET)).toMatchObject({ hook: { x: 384, y: 379 }, facing: -1 });
    expect(shotFor({ x: 300, y: 480 }, [perch("stub", 380, 420, 380)], [], PET)).toMatchObject({ hook: { x: 384, y: 379 }, facing: 1 });
    expect(shotFor({ x: 400, y: 480 }, [perch("pin", 398, 404, 380)], [], PET)!.hook).toEqual({ x: 401, y: 379 });
  });

  it("reels a steep rope straight in and swings on a slanted one — unless the swing would carry it into a keep-out under the hook", () => {
    const steep = from(310)!;
    const slanted = from(240)!;
    expect(Math.abs(steep.hook.x - steep.muzzle.x)).toBeLessThanOrEqual(ZIP_SLANT * (steep.muzzle.y - steep.hook.y));
    expect(steep.reel).toBe("zip");
    expect(Math.abs(slanted.hook.x - slanted.muzzle.x)).toBeGreaterThan(ZIP_SLANT * (slanted.muzzle.y - slanted.hook.y));
    expect(slanted.reel).toBe("swing");
    const drop = slanted.hook.y + slanted.length;
    expect(from(240, [FLOOR, shelf], [box, keepout(296, drop - 10, 208, 20)])).toEqual({ ...slanted, reel: "zip" });
    expect(from(240, [FLOOR, shelf], [box, keepout(296, drop + ROPE_MARGIN + 0.5, 208, 20)])).toEqual(slanted);
    expect(from(240, [FLOOR, shelf], [box, keepout(slanted.hook.x + ROPE_MARGIN + 0.5, 430, 20, 20)])).toEqual(slanted);
  });

  it("needs a perch high enough above the feet, a rope of the right length and a rope steep enough", () => {
    const high = (y: number, size: Size = PET) => shotFor({ x: 400, y: 480 }, [perch("stub", 380, 420, y)], [], size);
    expect(high(480 - ROPE_RISE * 48 + 0.5)).toBeNull();
    expect(high(413)).not.toBeNull();
    expect(high(413)!.length).toBeGreaterThanOrEqual(ROPE_SHORT * 48);
    expect(high(414)).toBeNull();
    expect(high(66)).not.toBeNull();
    expect(high(66)!.length).toBeLessThanOrEqual(ROPE_LONG * 48);
    expect(high(65)).toBeNull();
    expect(high(480 - MUZZLE_HEIGHT * 56 - 420, LARGE)).not.toBeNull();
    expect(high(480 - MUZZLE_HEIGHT * 56 - 420, PET)).toBeNull();
    expect(high(480, PET)).toBeNull();
    expect(high(520, PET)).toBeNull();
    expect(from(240)).not.toBeNull();
    expect(from(140)).toBeNull();
    const flat = shotFor({ x: 100, y: 420 }, [perch("shelf", 180, 494, 380)], [], PET);
    expect(flat).toBeNull();
    for (const x of [200, 240, 280, 330, 400, 470, 520, 560]) {
      const shot = from(x);
      if (shot === null) continue;
      expect(shot.length >= ROPE_SHORT * 48 && shot.length <= ROPE_LONG * 48).toBe(true);
      expect(shot.muzzle.y - shot.hook.y).toBeGreaterThanOrEqual(ROPE_ELEVATION * shot.length);
    }
  });

  it("passes over what lies under its hook and is blocked by anything else on its line", () => {
    const clear = from(280)!;
    expect(segmentHits(clear.muzzle, clear.hook, box, ROPE_MARGIN)).toBe(true);
    expect(from(280, [FLOOR, shelf], [box, keepout(310, 380, 100, 30)])).toEqual(clear);
    expect(from(280, [FLOOR, shelf], [box, keepout(310, 384, 100, 30)])).toBeNull();
    expect(from(280, [FLOOR, shelf], [box, keepout(300, 420, 16, 30)])).toBeNull();
    expect(from(280, [FLOOR, shelf], [box, keepout(312, 340, 100, 80)])).toEqual(clear);
    expect(from(280, [FLOOR, shelf], [box, keepout(311.5, 340, 100, 80)])).toBeNull();
    const reach = along(clear.muzzle, clear.hook, 422);
    expect(from(280, [FLOOR, shelf], [box, keepout(reach - ROPE_MARGIN - 0.5 - 20, 400, 20, 20)])).toEqual(clear);
    expect(from(280, [FLOOR, shelf], [box, keepout(reach - ROPE_MARGIN + 0.5 - 20, 400, 20, 20)])).toBeNull();
  });

  it("chooses the shot with the least rope and detour among all perches, the first one among equals", () => {
    const upper = perch("upper", 306, 494, 350);
    expect(from(400, [FLOOR, upper, shelf], [])!.surface).toBe("shelf");
    expect(from(400, [FLOOR, shelf, upper], [])!.surface).toBe("shelf");
    expect(from(400, [upper], [])!.surface).toBe("upper");
    expect(from(400, [upper, shelf])!.surface).toBe("shelf");
    expect(from(400, [upper])).toBeNull();
    const twin = perch("twin", 306, 494, 380);
    expect(from(400, [twin, shelf], [])!.surface).toBe("twin");
    expect(from(400, [shelf, twin], [])!.surface).toBe("shelf");
    expect(from(480, [perch("left", 306, 400, 380), perch("right", 560, 654, 380)], [])!.surface).toBe("left");
    expect(from(480, [perch("right", 560, 654, 380), perch("left", 306, 400, 380)], [])!.surface).toBe("right");
    expect(from(481, [perch("left", 306, 400, 380), perch("right", 560, 654, 380)], [])!.surface).toBe("right");
    expect(from(400, [], [])).toBeNull();
    expect(from(400, [FLOOR], [])).toBeNull();
  });

  it("keeps every law on generated stages", () => {
    const next = stream(43);
    let granted = 0;
    let refused = 0;
    let swings = 0;
    for (let stage = 0; stage < STAGES; stage++) {
      const size = [PET, SMALL, LARGE][next(3)]!;
      const feet = { x: 100 + next(1600) / 4, y: 300 + next(800) / 4 };
      const perches = Array.from({ length: 1 + next(3) }, (_, index) => {
        const x0 = next(2000) / 4;
        return perch(`p${index}`, x0, x0 + 20 + next(800) / 4, 100 + next(1400) / 4);
      });
      const keepouts = [...perches.map((edge) => solid(edge.x0, edge.x1, edge.y, edge.y + 40)), ...Array.from({ length: next(3) }, () => keepout(next(2400) / 4, 100 + next(1600) / 4, next(240) / 4, next(240) / 4))];
      const shot = shotFor(feet, perches, keepouts, size);
      if (shot === null) {
        refused++;
        continue;
      }
      granted++;
      if (shot.reel === "swing") swings++;
      const label = `stage ${stage}`;
      const target = perches.find((edge) => edge.surface === shot.surface)!;
      expect(feet.y - target.y, label).toBeGreaterThanOrEqual(ROPE_RISE * size.height);
      expect(shot.hook.y, label).toBe(target.y - HOOK_LIFT);
      expect(target.x1 - target.x0 >= 2 * HOOK_INSET ? shot.hook.x >= target.x0 + HOOK_INSET && shot.hook.x <= target.x1 - HOOK_INSET : shot.hook.x === (target.x0 + target.x1) / 2, label).toBe(true);
      expect(shot.muzzle, label).toEqual({ x: feet.x + shot.facing * MUZZLE_FORWARD * size.width, y: feet.y - MUZZLE_HEIGHT * size.height });
      expect(shot.facing, label).toBe(shot.hook.x < feet.x ? -1 : 1);
      expect(Math.abs(shot.length - Math.hypot(shot.hook.x - shot.muzzle.x, shot.hook.y - shot.muzzle.y)), label).toBeLessThan(1e-9);
      expect(shot.length >= ROPE_SHORT * size.height && shot.length <= ROPE_LONG * size.height, label).toBe(true);
      expect(shot.muzzle.y - shot.hook.y, label).toBeGreaterThanOrEqual(ROPE_ELEVATION * shot.length);
      let dropped = false;
      for (const blocker of keepouts) {
        const under = blocker.width > 0 && blocker.height > 0 && blocker.x <= shot.hook.x && shot.hook.x <= blocker.x + blocker.width && blocker.y <= target.y && target.y <= blocker.y + blocker.height;
        if (!under) expect(segmentHits(shot.muzzle, shot.hook, blocker, ROPE_MARGIN), label).toBe(false);
        if (!under && segmentHits(shot.hook, { x: shot.hook.x, y: shot.hook.y + shot.length }, blocker, ROPE_MARGIN)) dropped = true;
      }
      expect(shot.reel, label).toBe(Math.abs(shot.hook.x - shot.muzzle.x) <= ZIP_SLANT * (shot.muzzle.y - shot.hook.y) || dropped ? "zip" : "swing");
      expect(shotHolds(shot, perches, keepouts), label).toEqual(shot);
    }
    expect(granted).toBeGreaterThan(STAGES / 12);
    expect(swings).toBeGreaterThan(STAGES / 60);
    expect(refused).toBeGreaterThan(STAGES / 4);
  });
});

describe("the hook", () => {
  const muzzle = { x: 292, y: 448.8 };
  const hook = { x: 330, y: 379 };
  const length = Math.hypot(38, 69.8);

  it("flies on a straight line at a constant speed and ends on its target exactly", () => {
    const ticks = hookTicks(muzzle, hook, HOOK_SPEED);
    expect(ticks).toBe(Math.ceil(length / 10));
    expect(hookStep(muzzle, hook, HOOK_SPEED, 0)).toEqual(muzzle);
    expect(hookStep(muzzle, hook, HOOK_SPEED, -2)).toEqual(muzzle);
    for (let tick = 1; tick < ticks; tick++) {
      const tip = hookStep(muzzle, hook, HOOK_SPEED, tick);
      expect(Math.abs(Math.hypot(tip.x - muzzle.x, tip.y - muzzle.y) - 10 * tick), `tick ${tick}`).toBeLessThan(1e-9);
      expect(Math.abs((tip.x - muzzle.x) * (hook.y - muzzle.y) - (tip.y - muzzle.y) * (hook.x - muzzle.x)), `tick ${tick}`).toBeLessThan(1e-9);
    }
    expect(hookStep(muzzle, hook, HOOK_SPEED, ticks)).toEqual(hook);
    expect(hookStep(muzzle, hook, HOOK_SPEED, ticks + 5)).toEqual(hook);
  });

  it("comes back twice as fast after a miss, from wherever it was", () => {
    const tip = hookStep(muzzle, hook, HOOK_SPEED, 5);
    const ticks = hookTicks(tip, muzzle, HOOK_RETURN);
    expect(ticks).toBe(3);
    expect(Math.hypot(hookStep(tip, muzzle, HOOK_RETURN, 1).x - tip.x, hookStep(tip, muzzle, HOOK_RETURN, 1).y - tip.y)).toBeCloseTo(20, 9);
    expect(hookStep(tip, muzzle, HOOK_RETURN, ticks)).toEqual(muzzle);
  });

  it("needs no tick for no distance and one tick for anything up to a step", () => {
    expect(hookTicks(muzzle, muzzle, HOOK_SPEED)).toBe(0);
    expect(hookStep(muzzle, muzzle, HOOK_SPEED, 0)).toEqual(muzzle);
    expect(hookStep(muzzle, muzzle, HOOK_SPEED, 1)).toEqual(muzzle);
    expect(hookTicks({ x: 0, y: 0 }, { x: 6, y: 8 }, HOOK_SPEED)).toBe(1);
    expect(hookTicks({ x: 0, y: 0 }, { x: 6, y: 8.5 }, HOOK_SPEED)).toBe(2);
    expect(hookTicks({ x: 0, y: 0 }, { x: 60, y: 80 }, HOOK_SPEED)).toBe(10);
    expect(hookStep({ x: 0, y: 0 }, { x: 60, y: 80 }, HOOK_SPEED, 9)).toEqual({ x: 54, y: 72 });
  });
});

describe("the haul", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const shot: Shot = shotFor({ x: 400, y: 520 }, [shelf], [], PET)!;

  it("begins at rest where the actor stood when the hook bit, its feet under its hands as they stood under the muzzle", () => {
    expect(shot.reel).toBe("zip");
    const start = haulOf(shot, shot.length, PET);
    expect(start.rope).toBe(shot.length);
    expect(start.before).toEqual(start.hand);
    expect(Math.hypot(start.hand.x - shot.muzzle.x, start.hand.y - shot.muzzle.y)).toBeLessThan(1e-9);
    expect(Math.abs(start.x - 400)).toBeLessThan(1e-9);
    expect(Math.abs(start.y - 520)).toBeLessThan(1e-9);
    const hung = haulOf(shot, 0, PET);
    expect(hung).toEqual({ rope: 0, hand: shot.hook, before: shot.hook, x: shot.hook.x - MUZZLE_FORWARD * 40, y: shot.hook.y + MUZZLE_HEIGHT * 48 });
    const half = haulOf(shot, shot.length / 2, PET);
    expect(Math.abs(half.x - (start.x + hung.x) / 2)).toBeLessThan(1e-9);
    expect(Math.abs(half.y - (start.y + hung.y) / 2)).toBeLessThan(1e-9);
    const mirrored = shotFor({ x: 420, y: 520 }, [perch("stub", 380, 420, 380)], [], PET)!;
    expect(mirrored.facing).toBe(-1);
    expect(haulOf(mirrored, 0, PET).x).toBe(mirrored.hook.x + MUZZLE_FORWARD * 40);
  });

  it("gathers its speed over the ramp, reels at the zip speed along the taut line and ends where the hoist begins", () => {
    const end = REEL_LEAST * 48;
    let haul = haulOf(shot, shot.length, PET);
    let ticks = 0;
    while (haul.rope > end && ticks < 500) {
      const next = zipStep(shot, haul, ticks, PET);
      expect(Math.abs(haul.rope - next.rope - Math.min((ZIP_SPEED * smoothstep((ticks + 1) / ZIP_RAMP)) / 64, haul.rope - end)), `tick ${ticks}`).toBeLessThan(1e-9);
      expect(next).toEqual({ ...haulOf(shot, next.rope, PET), before: haul.hand });
      expect(haulStep(shot, haul, ticks, PET)).toEqual(next);
      haul = next;
      ticks++;
    }
    expect(haul.rope).toBe(end);
    expect(ticks).toBe(haulTicks(shot, PET));
    expect(ticks).toBe(ZIP_RAMP + Math.ceil((shot.length - end - (5.5 * ZIP_SPEED) / 64) / (ZIP_SPEED / 64)));
    expect(zipStep(shot, haul, ticks, PET).rope).toBe(end);
  });

  it("leaves a rope alone that is no longer than its end", () => {
    const short = shotFor({ x: 400, y: 450.2 }, [shelf], [], PET)!;
    expect(short.length).toBeLessThan(REEL_LEAST * 48);
    expect(haulTicks(short, PET)).toBe(0);
    expect(zipStep(short, haulOf(short, short.length, PET), 0, PET).rope).toBe(short.length);
    expect(zipStep(shot, haulOf(shot, 20, PET), 30, PET).rope).toBe(20);
    expect(swayStep(shot, haulOf(shot, 20, PET), 30, PET).rope).toBe(20);
  });

  it("swings on a slanted rope as the pendulum of the swing module, the feet under the hands", () => {
    const slanted = shotFor({ x: 250, y: 480 }, [FLOOR, shelf], [], PET)!;
    const least = REEL_LEAST * 48;
    expect(slanted.reel).toBe("swing");
    let haul = haulOf(slanted, slanted.length, PET);
    let ticks = 0;
    let lowest = haul.hand.y;
    let crossed = false;
    while (haul.rope > least && ticks < 500) {
      const reel = reelStep(slanted.hook, haul.hand, haul.before, haul.rope, least, ticks);
      const next = swayStep(slanted, haul, ticks, PET);
      expect(next).toEqual({ rope: reel.length, hand: reel.bob, before: haul.hand, x: reel.bob.x - MUZZLE_FORWARD * 40, y: reel.bob.y + MUZZLE_HEIGHT * 48 });
      expect(haulStep(slanted, haul, ticks, PET)).toEqual(next);
      expect(Math.hypot(next.hand.x - slanted.hook.x, next.hand.y - slanted.hook.y), `tick ${ticks}`).toBeLessThan(next.rope + 1e-9);
      expect(Math.hypot(next.hand.x - haul.hand.x, next.hand.y - haul.hand.y), `tick ${ticks}`).toBeLessThan(REEL_CAP / 64 + 1e-9);
      expect(haul.rope - next.rope, `tick ${ticks}`).toBeCloseTo(Math.min((REEL_SPEED * Math.min(ticks + 1, REEL_RAMP)) / REEL_RAMP / 64, haul.rope - least), 9);
      lowest = Math.max(lowest, next.hand.y);
      crossed = crossed || next.hand.x > slanted.hook.x;
      haul = next;
      ticks++;
    }
    expect(haul.rope).toBe(least);
    expect(ticks).toBe(haulTicks(slanted, PET));
    expect(ticks).toBe(REEL_RAMP - 1 + Math.ceil((slanted.length - least - (4.5 * REEL_SPEED) / 64) / (REEL_SPEED / 64)));
    expect(crossed).toBe(true);
    expect(lowest).toBeGreaterThan(slanted.muzzle.y);
    expect(lowest).toBeLessThan(slanted.hook.y + slanted.length);
  });

  it("lands inside the hook, towards the middle of the perch and never beyond its ends", () => {
    expect(landingFor(shot, shelf, PET)).toEqual({ x: 400 + 20 + MANTLE_INSET, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 330, y: 379 } }, shelf, PET)).toEqual({ x: 358, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 470, y: 379 } }, shelf, PET)).toEqual({ x: 442, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 400, y: 379 } }, perch("stub", 380, 420, 380), PET)).toEqual({ x: 420, y: 380 });
    expect(landingFor({ ...shot, hook: { x: 401, y: 379 } }, perch("stub", 380, 420, 380), PET)).toEqual({ x: 380, y: 380 });
  });
});

describe("shotHolds", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const box = solid(300, 500, 380, 420);
  const shot = shotFor({ x: 280, y: 480 }, [FLOOR, shelf], [box], PET)!;

  it("holds as it is when nothing moved and follows its edge up and down", () => {
    expect(shotHolds(shot, [FLOOR, shelf], [box])).toEqual(shot);
    for (const dy of [-ROPE_FOLLOW, -3, 5, ROPE_FOLLOW]) {
      const held = shotHolds(shot, [FLOOR, perch("shelf", 306, 494, 380 + dy)], [solid(300, 500, 380 + dy, 420 + dy)])!;
      expect(held.hook).toEqual({ x: 310, y: 379 + dy });
      expect(Math.abs(held.length - Math.hypot(310 - shot.muzzle.x, 379 + dy - shot.muzzle.y))).toBeLessThan(1e-12);
      expect({ ...held, hook: shot.hook, length: shot.length }).toEqual(shot);
    }
  });

  it("loses an edge that moved too far, left the hook behind or vanished", () => {
    expect(shotHolds(shot, [FLOOR, perch("shelf", 306, 494, 380 + ROPE_FOLLOW + 0.5)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("shelf", 306, 494, 380 - ROPE_FOLLOW - 0.5)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("shelf", 310.5, 494, 380)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("shelf", 310, 494, 380)], [])).not.toBeNull();
    expect(shotHolds(shot, [FLOOR, perch("other", 306, 494, 380)], [])).toBeNull();
    expect(shotHolds(shot, [FLOOR], [])).toBeNull();
  });

  it("loses its line to a keep-out that appears on it", () => {
    expect(shotHolds(shot, [FLOOR, shelf], [box, keepout(300, 420, 16, 30)])).toBeNull();
    expect(shotHolds(shot, [FLOOR, shelf], [box, keepout(100, 420, 16, 30)])).toEqual(shot);
  });

  it("aims a shot that is meant to miss past the nearer end of the perch", () => {
    expect(missOf(shot, shelf)).toEqual({ x: 306 - ROPE_MISS_OVERSHOOT, y: 379 });
    expect(missOf({ ...shot, hook: { x: 470, y: 379 } }, shelf)).toEqual({ x: 494 + ROPE_MISS_OVERSHOOT, y: 379 });
    expect(missOf({ ...shot, hook: { x: 400, y: 379 } }, shelf)).toEqual({ x: 292, y: 379 });
  });
});

describe("routeOf", () => {
  const shelf = perch("shelf", 306, 494, 380);
  const left = pitch("shelf-left", "shelf", -1, 300, 380, 480);
  const right = pitch("shelf-right", "shelf", 1, 500, 380, 480);
  const box = solid(300, 500, 380, 480);
  const standing = ladderFor(FLOOR, shelf, right, [box], LARGE)!;
  const up = (gear: readonly ("climb" | "ladder" | "grapple" | "parachute")[], ladders: readonly Ladder[] = [], grip = GRIP_BUDGET, x = 200) => routeOf(x, FLOOR, shelf, gear, PET, grip, [left, right], ladders, [box]);

  it("offers every way the gear allows, a standing ladder first, then the wall, then a ladder of its own, then the rope", () => {
    const legs = up(["climb", "ladder", "grapple"])!;
    expect(legs.map((leg) => leg.means)).toEqual(["wall", "wall", "raise", "raise", "grapple"]);
    expect(legs[0]).toEqual({ means: "wall", at: 280, pitch: left, hold: { x: 280, y: 480, over: false }, exit: left, goal: rimOf(left, PET) });
    expect(legs[1]).toEqual({ means: "wall", at: 520, pitch: right, hold: { x: 520, y: 480, over: false }, exit: right, goal: rimOf(right, PET) });
    expect(legs[2]).toEqual({ means: "raise", at: 300 - LADDER_LEAN * 93, ladder: ladderFor(FLOOR, shelf, left, [box], PET) });
    expect(legs[4]).toEqual({ means: "grapple", at: 200, shot: shotFor({ x: 200, y: 480 }, [shelf], [box], PET) });
    const joined = up(["climb", "ladder", "grapple"], [standing])!;
    expect(joined.map((leg) => leg.means)).toEqual(["ladder", "wall", "wall", "grapple"]);
    expect(joined[0]).toEqual({ means: "ladder", at: standing.foot.x, ladder: standing, up: true });
  });

  it("offers only what the gear allows and nothing to who has none", () => {
    expect(up(["climb"])!.map((leg) => leg.means)).toEqual(["wall", "wall"]);
    expect(up(["ladder"])!.map((leg) => leg.means)).toEqual(["raise", "raise"]);
    expect(up(["grapple"])!.map((leg) => leg.means)).toEqual(["grapple"]);
    expect(up(["grapple"], [standing])!.map((leg) => leg.means)).toEqual(["ladder", "grapple"]);
    expect(up(["parachute"], [standing])).toBeNull();
    expect(up([], [standing])).toBeNull();
    expect(up(["parachute"])).toBeNull();
  });

  it("shoots from where the actor stands when that works, else from the point of its perch nearest to the target, else from farther and farther back until the line clears what stands under the edge", () => {
    expect(up(["grapple"], [], GRIP_BUDGET, 400)![0]).toEqual({ means: "grapple", at: 400, shot: shotFor({ x: 400, y: 480 }, [shelf], [box], PET) });
    expect(up(["grapple"], [], GRIP_BUDGET, 100)![0]!.at).toBe(306);
    expect(routeOf(100, perch("floor", 0, 140, 480), shelf, ["grapple"], PET, GRIP_BUDGET, [], [], [box])).toBeNull();
    const card = [solid(300, 500, 380, 470), keepout(302, 400, 196, 60)];
    const blocked = (x: number) => shotFor({ x, y: 480 }, [shelf], card, PET);
    expect([blocked(296), blocked(306), blocked(266)]).toEqual([null, null, null]);
    expect(routeOf(296, FLOOR, shelf, ["grapple"], PET, GRIP_BUDGET, [], [], card)).toEqual([{ means: "grapple", at: 306 - 2 * 40, shot: blocked(306 - 2 * 40) }]);
    expect(blocked(306 - 2 * 40)!.reel).toBe("zip");
    expect(routeOf(100, perch("floor", 0, 250, 480), shelf, ["grapple"], PET, GRIP_BUDGET, [], [], [box])![0]!.at).toBe(250);
    expect(routeOf(600, perch("floor", 540, 640, 480), shelf, ["grapple"], PET, GRIP_BUDGET, [], [], [box])![0]!.at).toBe(600);
    expect(routeOf(700, perch("floor", 540, 720, 480), shelf, ["grapple"], PET, GRIP_BUDGET, [], [], [box])![0]!.at).toBe(540);
  });

  it("refuses a wall the grip does not last for", () => {
    const need = WALL_GRAB_TICKS + climbTicks(480, rimOf(left, PET)) + MANTLE_TICKS;
    expect(up(["climb"], [], need)!.length).toBe(2);
    expect(up(["climb"], [], need - 1)).toBeNull();
    expect(up(["climb", "grapple"], [], 10)!.map((leg) => leg.means)).toEqual(["grapple"]);
  });

  it("leads down over the rim, by the wall or by a standing ladder, and never by a rope or a new ladder", () => {
    const down = (gear: readonly ("climb" | "ladder" | "grapple" | "parachute")[], ladders: readonly Ladder[] = [], grip = GRIP_BUDGET) => routeOf(400, shelf, FLOOR, gear, PET, grip, [left, right], ladders, [box]);
    const legs = down(["climb", "ladder", "grapple"])!;
    expect(legs).toEqual([
      { means: "wall", at: 328, pitch: left, hold: { x: 280, y: rimOf(left, PET), over: true }, exit: left, goal: 480 },
      { means: "wall", at: 472, pitch: right, hold: { x: 520, y: rimOf(right, PET), over: true }, exit: right, goal: 480 },
    ]);
    expect(down(["ladder", "grapple"])).toBeNull();
    expect(down(["ladder"], [standing])).toEqual([{ means: "ladder", at: ledgeOf(right, PET), ladder: standing, up: false }]);
    const need = WALL_HANG_TICKS + climbTicks(rimOf(left, PET), 480);
    expect(down(["climb"], [], need)!.length).toBe(2);
    expect(down(["climb"], [], need - 1)).toBeNull();
  });

  it("finds no way between perches nothing joins, onto the perch the actor is on, or without terrain", () => {
    const far = perch("far", 20, 120, 40);
    expect(routeOf(200, FLOOR, far, ["climb", "ladder", "grapple"], PET, GRIP_BUDGET, [left, right], [standing], [box])).toBeNull();
    expect(routeOf(400, shelf, shelf, ["climb", "ladder", "grapple"], PET, GRIP_BUDGET, [left, right], [standing], [box])).toBeNull();
    expect(routeOf(200, FLOOR, FLOOR, ["climb", "ladder", "grapple"], PET, GRIP_BUDGET, [left, right], [standing], [box])).toBeNull();
    expect(routeOf(200, FLOOR, shelf, ["climb", "ladder"], PET, GRIP_BUDGET, [], [], [box])).toBeNull();
  });
});

describe("chainOf", () => {
  it("joins a column of cards into one line, from the top down, and never holds a pitch twice — even pitches without length far out, where an end and a hold cannot be told apart", () => {
    const upper = pitch("upper-left", "upper", -1, 300, 100, 260);
    const lower = pitch("lower-left", "lower", -1, 300, 290, 480);
    expect(climbing.chainOf(lower, [upper, lower], PET)).toEqual([upper, lower]);
    expect(climbing.chainOf(upper, [lower, upper, { ...upper }], PET)).toEqual([upper, lower]);
    for (const y of [1e300, -1e300, 1e308]) {
      const flat = pitch("flat", "far", 1, 0, y, y);
      expect(climbing.crossable(flat, flat, PET)).toBe(true);
      expect(climbing.chainOf(flat, [flat, { ...flat }, flat], PET)).toEqual([flat]);
    }
  });
});

describe("the constants", () => {
  it("are the starting values of the research brief, but for the reach of the telescopic ladder and of the grappling gun (eight heights: what the real pages need)", () => {
    const numbers = Object.fromEntries(Object.entries(climbing).filter(([, value]) => typeof value === "number"));
    for (const [name, value] of Object.entries(numbers)) expect(Number.isFinite(value) && (value as number) > 0, name).toBe(true);
    expect(numbers).toMatchObject({ CLIMB_RISE: 30, CLIMB_DESCENT: 40, GRIP_BUDGET: 384, SLIDE_START: 30, SLIDE_GAIN: 600, SLIDE_SPEED: 160, SLIP_PUSH: 140, SLIP_LIFT: 160, MANTLE_TICKS: 28, LADDER_LEAN: 0.25, LADDER_STEEP: 0.14, LADDER_FLAT: 0.4, LADDER_SHORT: 0.8, LADDER_TALL: 8, LADDER_RISE: 26, LADDER_DESCENT: 34, HOOK_SPEED: 640, HOOK_RETURN: 1280, ROPE_SHORT: 0.8, ROPE_LONG: 8, ROPE_ELEVATION: 0.42, ZIP_SLANT: 0.35, ZIP_SPEED: 150 });
    expect(REEL_LEAST).toBe(0.9);
    expect(LADDER_FOLLOW).toBe(8);
    expect(LADDER_SHIFT).toBe(12);
    expect(ROPE_DETOUR).toBe(0.3);
    expect(CLIMB_RAMP).toBe(6);
    expect(TICKS_PER_SECOND).toBe(64);
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
