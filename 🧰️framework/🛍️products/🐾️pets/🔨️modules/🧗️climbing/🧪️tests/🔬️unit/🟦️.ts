/** 🧗️ Unit suite of the climbing module, walls: holds, climbs, slides and mantles on walls, each against its closed form.
 *
 * @see ../🧰️climbing-test-kit/🟦️.ts — the shared fixtures and builders
 * @see ../🪜️ladder-suites/🟦️.ts — ladders
 * @see ../🪝️rope-and-shot-suites/🟦️.ts — the grappling rope
 * @see ../🗺️route-and-determinism-suites/🟦️.ts — routes and determinism
 * @see ../../🟦️.ts — the implementation under test
 * @see ../../../../🧪️tests/🧗️wall-climbing/🥒️.feature — the same module against numpy and scipy
 */
import { describe, expect, it } from "vitest";
import { wallsOf } from "../../../🏞️terrain/🟦️.ts";
import { smoothstep } from "../../../📐️trigonometry/🟦️.ts";
import { CLIMB_DESCENT, CLIMB_RAMP, CLIMB_RISE, GRIP_BITE, GRIP_BUDGET, GRIP_SPACING, HAND_HEIGHT, HOIST_HUMP, MANTLE_INSET, MANTLE_TICKS, SLIDE_GAIN, SLIDE_SPEED, SLIDE_START, SLIP_LIFT, SLIP_PUSH, WALL_FOLLOW } from "../../🟦️.ts";
import { clings, climbPhase, climbStep, climbTicks, clingOf, crowns, footOf, gripFor, gripStep, hoistPath, ledgeOf, mantlePath, rimFor, rimOf, slideStep, slipOf, wallHolds } from "../../🟦️.ts";
import { CARD, CARD_BOX, climbed, FLOOR, LARGE, LEFT, perch, PET, pitch, RIGHT } from "../🧰️climbing-test-kit/🟦️.ts";

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
