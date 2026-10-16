/** 🏛️ Unit suite of the clearance module, laws of the world: lanes and metrics, exact operations without side effects, and the reference world that keeps every pair of bodies apart.
 *
 * @see ../🌍️reference-world/🟦️.ts — the world under test
 * @see ../🧰️clearance-tools/🟦️.ts — the sweeps and the ablations
 * @see ../../🟦️.ts — the implementation under test
 */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import type { Perch, Point } from "../../../../🧬️schema/🟦️.ts";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { LANE_LIFT, MARGIN, PUSHES, TALLY, type Tally, bodyOf, claimClear, claimOf, columnOver, evicted, freeAt, guardedStride, headUnder, laneLift, leaning, mustPoof, nearMisses, obstaclesOf, orderKept, orderOf, overlaps, perMillion, pruned, pushedOut, released, scootFraction, seatOf, shifted, sliceAt, slideOf, slotIn, spotOn, tallied } from "../../🟦️.ts";
import { RULES, bodiesOf, openWorld, tickWorld, runWorld } from "../🌍️reference-world/🟦️.ts";
import { WORLDS, WORLD_TICKS, ABLATIONS, ABLATION_TICKS, BITES, box, body, frozen, sharing, marching } from "../🧰️clearance-tools/🟦️.ts";

describe("lanes and metrics", () => {
  it("lifts the high lane of a floater by nine tenths of its height and margin", () => {
    expect(laneLift(48, 4)).toBe(LANE_LIFT * 52);
    expect(laneLift(40, 0)).toBe(36);
  });

  it("tallies a run tick by tick", () => {
    const apart = [body("a", 0, 0, 10, 10), body("b", 20, 0, 30, 10)];
    const near = [body("a", 0, 0, 10, 10), body("b", 11, 0, 30, 10)];
    const over = [body("a", 0, 0, 10, 10), body("b", 9, 0, 30, 10), body("c", 9.5, 0, 12, 10)];
    let tally: Tally = TALLY;
    tally = tallied(tally, apart, 2, 0, 1);
    expect(tally).toEqual({ ticks: 1, actors: 2, overlaps: 0, nears: 0, poofs: 0, waits: 1 });
    tally = tallied(tally, near, 2, 1, 0);
    expect(tally).toEqual({ ticks: 2, actors: 4, overlaps: 0, nears: 1, poofs: 1, waits: 1 });
    tally = tallied(tally, over, 2, 2, 3);
    expect(tally).toEqual({ ticks: 3, actors: 7, overlaps: 1, nears: 0 + 1, poofs: 3, waits: 4 });
    expect(TALLY).toEqual({ ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 });
    expect(perMillion(3, 7)).toBe(3000000 / 7);
    expect(perMillion(3, 0)).toBe(0);
  });
});

describe("determinism", () => {
  it("uses nothing but exact operations and has no side effect", () => {
    const source = readFileSync(new URL("../../🟦️.ts", import.meta.url), "utf8");
    const used = [...source.matchAll(/Math\.(\w+)/g)].map((match) => match[1]!);
    expect([...new Set(used)].sort()).toEqual(["abs", "floor", "max", "min"]);
    expect(source).not.toMatch(/\b(Date|performance|console|globalThis|process|Map|Set)\b/);
    expect(source).not.toMatch(/\S \*\* \S/);
  });

  it("never writes into what it is given", () => {
    const bodies = frozen([body("a", 150, 248, 198, 304), body("b", 190, 248, 238, 304), body("c", 400, 100, 440, 150)]);
    const path = frozen(marching(0, 0, 10, 0, 10));
    const claims = frozen([claimOf("c", 0, path, 3, path[9]!)]);
    const perch = frozen<Perch>({ surface: "s", x0: 100, x1: 400, y: 300 });
    const extent = frozen(box(100, 250, 140, 300));
    const point: Point = frozen({ x: 100, y: 300 });
    expect(() => {
      bodyOf("a", point, frozen({ width: 40, height: 48 }), 0, frozen(leaning(48, 1)), MARGIN);
      overlaps(bodies);
      nearMisses(bodies, 2);
      freeAt(extent, "x", bodies, claims, 3);
      slotIn(120, extent, 0, 500, obstaclesOf("x", bodies, claims, 3));
      spotOn(perch, 120, extent, 16, obstaclesOf("x", bodies, claims, 3));
      columnOver(perch, 120, extent, 40, 16, obstaclesOf("x", bodies, claims, 3));
      claimClear(claims[0]!, bodies, claims);
      claimClear(claimOf("x", 0, path, 1, null), bodies, claims);
      released(claims, "c");
      pruned(claims, 5);
      sliceAt(claims[0]!, 5);
      guardedStride(extent, 5, obstaclesOf("x", bodies, claims, 3));
      orderKept(orderOf(bodies), orderOf(bodies));
      seatOf(bodies, perch, MARGIN);
      scootFraction(frozen([1, 2]), 1);
      headUnder(extent, shifted(extent, 0, 5), "x", bodies);
      slideOf(extent, 1, 3, 0, 960, obstaclesOf("x", bodies, claims, 3));
      pushedOut(extent, obstaclesOf("x", bodies, claims, 3), PUSHES);
      mustPoof("x", extent, claims, bodies, claims, 3);
      evicted(bodies, claims, 3);
      tallied(TALLY, bodies, 2, 0, 0);
    }).not.toThrow();
  });
});

describe("the reference world", () => {
  it("keeps every pair of bodies apart at the end of every tick, and the order on every perch", () => {
    const sum = { overlaps: 0, disorders: 0, actors: 0, falls: 0, heads: 0, steered: 0, chutes: 0, hops: 0, glides: 0, grabs: 0, surveys: 0, seated: 0 };
    for (let seed = 1; seed <= WORLDS; seed++) {
      const world = runWorld(seed, WORLD_TICKS, RULES, 1);
      expect(world.tally.ticks).toBe(WORLD_TICKS);
      expect(world.tally.overlaps, `seed ${seed}`).toBe(0);
      expect(world.counts.disorders, `seed ${seed}`).toBe(0);
      expect(world.pets.length).toBeGreaterThanOrEqual(6);
      expect(world.pets.length).toBeLessThanOrEqual(10);
      for (const pet of world.pets) {
        expect(pet.size.width).toBeGreaterThanOrEqual(28);
        expect(pet.size.width).toBeLessThanOrEqual(58);
        expect(pet.size.height).toBeGreaterThanOrEqual(40);
        expect(pet.size.height).toBeLessThanOrEqual(56);
      }
      sum.overlaps += world.tally.overlaps;
      sum.actors += world.tally.actors;
      for (const key of ["falls", "heads", "steered", "chutes", "hops", "glides", "grabs", "surveys", "seated"] as const) sum[key] += world.counts[key];
    }
    expect(sum.actors).toBeGreaterThan(WORLDS * WORLD_TICKS * 5);
    for (const key of ["falls", "heads", "hops", "glides", "grabs", "surveys", "chutes"] as const) expect(sum[key], key).toBeGreaterThan(0);
  });

  it("holds with claims cut into slices of four ticks as well", () => {
    for (let seed = 1; seed <= sampled(1, 3, 6); seed++) {
      const world = runWorld(seed, WORLD_TICKS, RULES, 4);
      expect([world.tally.overlaps, world.counts.disorders], `seed ${seed}`).toEqual([0, 0]);
    }
  });

  it("is judged alike by an overlap test that measures the shared area", () => {
    const world = openWorld(3, RULES, 1);
    const bare = openWorld(3, { ...RULES, guard: false }, 1);
    let shared = 0;
    for (let tick = 0; tick < sampled(600, 3000, 30000); tick++) {
      tickWorld(world);
      tickWorld(bare);
      for (const run of [world, bare]) {
        const bodies = bodiesOf(run);
        let pairs = 0;
        for (let first = 0; first < bodies.length; first++) for (let second = first + 1; second < bodies.length; second++) if (sharing(bodies[first]!.extent, bodies[second]!.extent)) pairs++;
        expect(overlaps(bodies).length).toBe(pairs);
        if (run === bare) shared += pairs;
      }
    }
    expect(world.tally.overlaps).toBe(0);
    expect(shared).toBeGreaterThan(0);
    expect(bare.tally.overlaps).toBeGreaterThan(0);
  });

  it("is a pure function of its seed and rules", () => {
    const ticks = sampled(800, 5000, 30000);
    const one = runWorld(5, ticks, RULES, 1);
    const other = runWorld(5, ticks, RULES, 1);
    expect(other.tally).toEqual(one.tally);
    expect(other.counts).toEqual(one.counts);
    expect(bodiesOf(other)).toEqual(bodiesOf(one));
    expect(bodiesOf(runWorld(6, ticks, RULES, 1))).not.toEqual(bodiesOf(one));
  });

  it.each(BITES)("lets bodies overlap as soon as the rule $rule is switched off", ({ rule, seed, ticks }) => {
    expect(runWorld(seed, ticks, RULES, 1).tally.overlaps).toBe(0);
    expect(runWorld(seed, ticks, { ...RULES, [rule]: false }, 1).tally.overlaps).toBeGreaterThan(0);
    let overlapping = 0;
    for (let other = 1; other <= ABLATIONS; other++) overlapping += runWorld(other, ABLATION_TICKS, { ...RULES, [rule]: false }, 1).tally.overlaps;
    if (ABLATIONS > 0) expect(overlapping).toBeGreaterThan(0);
  });

  it("needs the poof where planning fails: without heads and air control pets poof instead of overlapping, and without the poof as well they overlap", () => {
    const ticks = sampled(600, 6000, 15000);
    const seeds = sampled(1, 6, 12);
    const sum = { kept: 0, poofs: 0, plain: 0, bare: 0 };
    for (let seed = 1; seed <= seeds; seed++) {
      const kept = runWorld(seed, ticks, { ...RULES, heads: false, steering: false }, 1);
      sum.kept += kept.tally.overlaps;
      sum.poofs += kept.tally.poofs;
      sum.plain += runWorld(seed, ticks, RULES, 1).tally.poofs;
      sum.bare += runWorld(seed, ticks, { ...RULES, heads: false, steering: false, poof: false }, 1).tally.overlaps;
    }
    expect(sum.kept).toBe(0);
    expect(sum.poofs).toBeGreaterThan(sum.plain);
    expect(sum.bare).toBeGreaterThan(0);
  });

  it("keeps bodies apart without the quality rules too, at the price of more poofs and falls", () => {
    const ticks = sampled(1000, 6000, 15000);
    const seeds = sampled(1, 6, 12);
    const sum = { poofs: 0, headless: 0, unsteered: 0, spilled: 0, unseated: 0 };
    for (let seed = 1; seed <= seeds; seed++) {
      const plain = runWorld(seed, ticks, RULES, 1);
      const headless = runWorld(seed, ticks, { ...RULES, heads: false }, 1);
      const unsteered = runWorld(seed, ticks, { ...RULES, steering: false }, 1);
      const unseated = runWorld(seed, ticks, { ...RULES, seating: false }, 1);
      for (const world of [plain, headless, unsteered, unseated]) expect([world.tally.overlaps, world.counts.disorders], `seed ${seed}`).toEqual([0, 0]);
      expect(headless.counts.heads).toBe(0);
      expect(unsteered.counts.steered).toBe(0);
      expect(unseated.counts.seated).toBe(0);
      sum.poofs += plain.tally.poofs;
      sum.headless += headless.tally.poofs;
      sum.unsteered += unsteered.tally.poofs;
      sum.spilled += plain.counts.spilled;
      sum.unseated += unseated.counts.spilled;
    }
    if (seeds > 1) {
      expect(sum.headless).toBeGreaterThan(sum.poofs);
      expect(sum.unsteered).toBeGreaterThan(sum.poofs);
      expect(sum.unseated).toBeGreaterThan(sum.spilled);
    }
  });
});
