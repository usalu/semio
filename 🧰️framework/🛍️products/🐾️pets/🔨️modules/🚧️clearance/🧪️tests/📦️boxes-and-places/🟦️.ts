/** 📦️ Unit suite of the clearance module, boxes and places: the constants of the design, the box of a body around its feet, and the free places among obstacles.
 *
 * @see ../🧰️clearance-tools/🟦️.ts — the builders of the cases
 * @see ../../🟦️.ts — the implementation under test
 */
import { describe, expect, it } from "vitest";
import type { Perch } from "../../../../🧬️schema/🟦️.ts";
import { type Claim, DELAY, FOREVER, LANE_LIFT, LEAN, MARGIN, PATIENCE, PUSHES, SCOOT_HASTE, SEAM, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, SLIDE_OFF_SPEED, STEERINGS, TALLY, UPRIGHT, bodyOf, canopied, columnOver, freeAmong, freeAt, grown, leaning, meets, nearMisses, obstaclesOf, overlaps, shifted, slotIn, spotOn, united } from "../../🟦️.ts";
import { CASES, box, body, stream, sharing, scattered } from "../🧰️clearance-tools/🟦️.ts";

describe("constants", () => {
  it("are the starting values of the design", () => {
    expect([MARGIN, SEAM, FOREVER, PUSHES, PATIENCE, DELAY]).toEqual([4, 1 / 64, 2 ** 32 - 1, 4, 40, 32]);
    expect([SLIDE_OFF_SPEED, SLIDE_OFF_GAIN, SLIDE_OFF_LIMIT, LEAN, LANE_LIFT, SCOOT_HASTE]).toEqual([32, 2, 128, 0.25, 0.9, 1.5]);
    expect(STEERINGS).toEqual([0, 30, -30, 60, -60, 90, -90, 130, -130, 180, -180, 240, -240]);
    expect(TALLY).toEqual({ ticks: 0, actors: 0, overlaps: 0, nears: 0, poofs: 0, waits: 0 });
  });
});

describe("bodies", () => {
  const size = { width: 40, height: 48 };

  it("builds the box around the feet from size, hover, posture and margin", () => {
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, UPRIGHT, 4)).toEqual({ owner: "a", extent: { x0: 76, y0: 248, x1: 124, y1: 304 } });
    expect(bodyOf("a", { x: 100, y: 290 }, size, 10, UPRIGHT, 4).extent).toEqual({ x0: 76, y0: 238, x1: 124, y1: 304 });
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, leaning(48, 1), 4).extent).toEqual({ x0: 64, y0: 248, x1: 136, y1: 304 });
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, canopied(size, { width: 64, height: 28 }), 4).extent).toEqual({ x0: 64, y0: 220, x1: 136, y1: 304 });
    expect(bodyOf("a", { x: 100, y: 300 }, size, 0, { left: 3, right: 14, above: 0 }, 0).extent).toEqual({ x0: 77, y0: 252, x1: 134, y1: 300 });
  });

  it("widens a leaning body by a quarter of its height times the sine, and a canopied one by what the canopy overhangs", () => {
    expect(leaning(48, 1)).toEqual({ left: 12, right: 12, above: 0 });
    expect(leaning(48, -0.5)).toEqual({ left: 6, right: 6, above: 0 });
    expect(leaning(48, 0)).toEqual(UPRIGHT);
    expect(canopied(size, { width: 64, height: 28 })).toEqual({ left: 12, right: 12, above: 28 });
    expect(canopied(size, { width: 30, height: 20 })).toEqual({ left: 0, right: 0, above: 20 });
    expect(leaning(48, 1).left).toBe(LEAN * 48);
  });

  it("lets neighbours with the comfortable gap of 8 px between their size boxes touch without overlapping", () => {
    const one = bodyOf("a", { x: 100, y: 300 }, size, 0, UPRIGHT, MARGIN);
    const apart = bodyOf("b", { x: 100 + (40 + 30) / 2 + 2 * MARGIN, y: 300 }, { width: 30, height: 40 }, 0, UPRIGHT, MARGIN);
    const closer = bodyOf("b", { x: 100 + (40 + 30) / 2 + 2 * MARGIN - 1 / 1024, y: 300 }, { width: 30, height: 40 }, 0, UPRIGHT, MARGIN);
    expect(apart.extent.x0).toBe(one.extent.x1);
    expect(overlaps([one, apart])).toEqual([]);
    expect(overlaps([one, closer])).toEqual([{ first: "a", second: "b" }]);
  });

  it("meets only where open boxes intersect", () => {
    const one = box(0, 0, 10, 10);
    expect(meets(one, box(10, 0, 20, 10))).toBe(false);
    expect(meets(one, box(0, 10, 10, 20))).toBe(false);
    expect(meets(one, box(9.999, 9.999, 20, 20))).toBe(true);
    expect(meets(one, box(2, 2, 3, 3))).toBe(true);
    expect(meets(box(2, 2, 3, 3), one)).toBe(true);
    expect(meets(one, box(-5, 4, 15, 6))).toBe(true);
    expect(meets(one, box(11, 0, 20, 10))).toBe(false);
    expect(meets(one, one)).toBe(true);
  });

  it("moves, grows and unites boxes", () => {
    expect(shifted(box(1, 2, 3, 4), 10, -1)).toEqual(box(11, 1, 13, 3));
    expect(grown(box(1, 2, 3, 4), 0.5)).toEqual(box(0.5, 1.5, 3.5, 4.5));
    expect(united(box(1, 2, 3, 4), box(-1, 3, 2, 9))).toEqual(box(-1, 2, 3, 9));
  });

  it("names every overlapping pair, by the first and then the second in the list", () => {
    const bodies = [body("a", 0, 0, 10, 10), body("b", 5, 5, 15, 15), body("c", 8, 8, 20, 20), body("d", 100, 100, 110, 110)];
    expect(overlaps(bodies)).toEqual([
      { first: "a", second: "b" },
      { first: "a", second: "c" },
      { first: "b", second: "c" },
    ]);
    expect(overlaps([bodies[3]!, bodies[2]!, bodies[0]!])).toEqual([{ first: "c", second: "a" }]);
    expect(overlaps([])).toEqual([]);
    expect(overlaps([bodies[0]!])).toEqual([]);
  });

  it("names as near misses the pairs that are apart but closer than the margin", () => {
    const bodies = [body("a", 0, 0, 10, 10), body("b", 11, 0, 20, 10), body("c", 5, 5, 8, 8), body("e", 11, 11, 20, 20)];
    expect(nearMisses(bodies, 2)).toEqual([
      { first: "a", second: "b" },
      { first: "a", second: "e" },
      { first: "b", second: "e" },
    ]);
    expect(nearMisses(bodies, 1.5)).toEqual(nearMisses(bodies, 2));
    expect(nearMisses(bodies, 1)).toEqual([]);
    expect(nearMisses(bodies, 0)).toEqual([]);
  });

  it("agrees with the area of the intersection on generated bodies", () => {
    const next = stream(11);
    for (let round = 0; round < CASES; round++) {
      const bodies = scattered(next, 8).map((extent, index) => ({ owner: `b${index}`, extent }));
      const expected: { first: string; second: string }[] = [];
      for (let first = 0; first < bodies.length; first++) for (let second = first + 1; second < bodies.length; second++) if (sharing(bodies[first]!.extent, bodies[second]!.extent)) expected.push({ first: bodies[first]!.owner, second: bodies[second]!.owner });
      expect(overlaps(bodies)).toEqual(expected);
    }
  });
});

describe("free places", () => {
  const bodies = [body("a", 0, 0, 10, 10), body("b", 20, 0, 30, 10)];
  const claims: Claim[] = [
    { owner: "c", slices: [{ from: 5, until: 5, extent: box(40, 0, 50, 10) }, { from: 6, until: 9, extent: box(60, 0, 70, 10) }], rest: box(80, 0, 90, 10) },
    { owner: "a", slices: [{ from: 5, until: 9, extent: box(200, 0, 210, 10) }], rest: box(220, 0, 230, 10) },
  ];

  it("gathers the bodies of the others, the corridors that are left and the rests", () => {
    expect(obstaclesOf("a", bodies, claims, 5)).toEqual([box(20, 0, 30, 10), box(40, 0, 50, 10), box(60, 0, 70, 10), box(80, 0, 90, 10)]);
    expect(obstaclesOf("a", bodies, claims, 6)).toEqual([box(20, 0, 30, 10), box(60, 0, 70, 10), box(80, 0, 90, 10)]);
    expect(obstaclesOf("a", bodies, claims, 10)).toEqual([box(20, 0, 30, 10), box(80, 0, 90, 10)]);
    expect(obstaclesOf("c", bodies, claims, 10)).toEqual([box(0, 0, 10, 10), box(20, 0, 30, 10), box(220, 0, 230, 10)]);
    expect(obstaclesOf("nobody", [], [], 0)).toEqual([]);
  });

  it("calls a place free when no body, no corridor that is left and no rest of another owner overlaps it", () => {
    expect(freeAt(box(10, 0, 20, 10), "x", bodies, claims, 5)).toBe(true);
    expect(freeAt(box(5, 0, 15, 10), "x", bodies, claims, 5)).toBe(false);
    expect(freeAt(box(5, 0, 15, 10), "a", bodies, claims, 5)).toBe(true);
    expect(freeAt(box(45, 0, 55, 10), "x", bodies, claims, 5)).toBe(false);
    expect(freeAt(box(45, 0, 55, 10), "x", bodies, claims, 6)).toBe(true);
    expect(freeAt(box(85, 0, 95, 10), "x", bodies, claims, 1000)).toBe(false);
    expect(freeAt(box(85, 0, 95, 10), "c", bodies, claims, 1000)).toBe(true);
    expect(freeAmong(box(10, 0, 20, 10), [box(0, 0, 10, 10), box(20, 0, 30, 10)])).toBe(true);
    expect(freeAmong(box(10, 0, 20.5, 10), [box(0, 0, 10, 10), box(20, 0, 30, 10)])).toBe(false);
  });

  const extent = box(80, 250, 120, 300);

  it("finds the nearest place that keeps a seam from every obstacle of its height", () => {
    expect(slotIn(100, extent, 20, 400, [])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 260, 150, 300)])).toBe(90 - SEAM);
    expect(slotIn(100, extent, 20, 400, [box(60, 260, 90, 300)])).toBe(110 + SEAM);
    expect(slotIn(100, extent, 20, 400, [box(90, 260, 110, 300)])).toBe(70 - SEAM);
    expect(slotIn(100, extent, 20, 400, [box(110, 100, 150, 200)])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 200, 150, 250)])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 300, 150, 350)])).toBe(100);
    expect(slotIn(100, extent, 20, 400, [box(110, 200, 150, 250.25)])).toBe(90 - SEAM);
    expect(slotIn(100, extent, 150, 400, [])).toBe(150);
    expect(slotIn(100, extent, 20, 60, [])).toBe(60);
  });

  it("answers a single point when that is all there is, and nothing when there is no room", () => {
    expect(slotIn(100, extent, 90 - SEAM, 160, [box(110, 260, 150, 300)])).toBe(90 - SEAM);
    expect(slotIn(100, extent, 90, 110, [box(110, 260, 150, 300)])).toBeNull();
    expect(slotIn(100, extent, 90, 170, [box(110, 260, 150, 300)])).toBeNull();
    expect(slotIn(100, extent, 90, 170 + SEAM, [box(110, 260, 150, 300)])).toBe(170 + SEAM);
    expect(slotIn(100, extent, 120, 110, [])).toBeNull();
    expect(slotIn(100, extent, Number.NaN, 110, [])).toBeNull();
  });

  it("keeps the footprint on the perch and looks down the whole column for a landing", () => {
    const perch: Perch = { surface: "s", x0: 50, x1: 300, y: 300 };
    expect(spotOn(perch, 100, extent, 16, [])).toBe(100);
    expect(spotOn(perch, 40, shifted(extent, -60, 0), 16, [])).toBe(66);
    expect(spotOn(perch, 400, shifted(extent, 300, 0), 16, [])).toBe(284);
    expect(spotOn({ surface: "s", x0: 50, x1: 70, y: 300 }, 60, shifted(extent, -40, 0), 16, [])).toBeNull();
    expect(spotOn(perch, 100, extent, 16, [box(110, 260, 150, 300)])).toBe(90 - SEAM);
    const high = box(80, 100, 120, 150);
    const below = [box(110, 200, 150, 250)];
    expect(spotOn(perch, 100, high, 16, below)).toBe(100);
    expect(columnOver(perch, 100, high, 150, 16, below)).toBe(90 - SEAM);
    expect(columnOver(perch, 100, high, 50, 16, below)).toBe(100);
    expect(columnOver(perch, 100, high, -30, 16, below)).toBe(100);
  });

  it("agrees with a search over every place of a 1/64 px lattice", () => {
    const next = stream(23);
    for (let round = 0; round < CASES; round++) {
      const x = 100 + next(400) / 4;
      const half = 10 + next(80) / 4;
      const own = box(x - half, 250, x + half, 300);
      const obstacles = scattered(next, 1 + next(5));
      const low = next(400) / 4;
      const high = low + next(800) / 4;
      let best: number | null = null;
      for (let step = 0; step <= (high - low) * 64; step++) {
        const place = low + step / 64;
        const there = shifted(own, place - x, 0);
        if (obstacles.some((obstacle) => sharing({ x0: there.x0 - SEAM, y0: there.y0, x1: there.x1 + SEAM, y1: there.y1 }, obstacle))) continue;
        if (best === null || Math.abs(place - x) < Math.abs(best - x)) best = place;
      }
      expect(slotIn(x, own, low, high, obstacles), `round ${round}`).toBe(best);
    }
  });
});
