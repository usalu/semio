/** 🪜️ Unit suite of the climbing module, ladders: the placement of a ladder, climbing it, what it holds and where it leads. */
import { describe, expect, it } from "vitest";
import { type Perch, type Rect, type Size } from "../../../../🧬️schema/🟦️.ts";
import { segmentHits } from "../../../🏞️terrain/🟦️.ts";
import { smoothstep } from "../../../📐️trigonometry/🟦️.ts";
import { LADDER_DESCENT, LADDER_EXIT, LADDER_FLAT, LADDER_FOLLOW, LADDER_FOOTING, LADDER_GIRTH, LADDER_LEAN, LADDER_RISE, LADDER_SHIFT, LADDER_SHORT, LADDER_STEEP, LADDER_TALL, LADDER_TUCK, MANTLE_INSET, RUNG_SPACING, TOPPLE_PUSH, TOPPLE_STEP, clings, ladderAt, ladderExit, ladderFor, ladderHolds, ladderLanding, ladderLean, ladderLength, ladderPhase, ladderRungs, ladderStep, ladderTo, spillOf } from "../../🟦️.ts";
import { along, FLOOR, holding, keepout, type Ladder, LARGE, perch, PET, pitch, SMALL, solid, STAGES, stream } from "../🧰️climbing-test-kit/🟦️.ts";

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
