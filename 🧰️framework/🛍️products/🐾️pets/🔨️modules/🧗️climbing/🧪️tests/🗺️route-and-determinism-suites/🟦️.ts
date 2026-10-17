/** 🗺️ Unit suite of the climbing module, routes: the routes between perches, the chains, the constants and determinism. */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { TICKS_PER_SECOND } from "../../../../🧬️schema/🟦️.ts";
import { REEL_LEAST } from "../../../🪢️swing/🟦️.ts";
import * as climbing from "../../🟦️.ts";
import { CLIMB_RAMP, GRIP_BUDGET, LADDER_FOLLOW, LADDER_LEAN, LADDER_SHIFT, MANTLE_TICKS, ROPE_DETOUR, WALL_GRAB_TICKS, WALL_HANG_TICKS, climbTicks, ladderFor, ledgeOf, rimOf, routeOf, shotFor } from "../../🟦️.ts";
import { FLOOR, keepout, type Ladder, LARGE, perch, PET, pitch, solid } from "../🧰️climbing-test-kit/🟦️.ts";

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
