/** 📜️ Unit suite of the clearance module, claims and order: planned paths cut into slices and compared tick by tick, and strides that keep walkers apart and in their order on a perch.
 *
 * @see ../🧰️clearance-tools/🟦️.ts — the builders of the cases
 * @see ../../🟦️.ts — the implementation under test
 */
import { describe, expect, it } from "vitest";
import { sampled } from "../../../../🧪️tests/🎚️config/🟦️.ts";
import { hopOf, hopStep } from "../../../🏞️terrain/🟦️.ts";
import { type Body, type Claim, type Extent, MARGIN, SEAM, UPRIGHT, bodyOf, claimClear, claimOf, guardedStride, obstaclesOf, orderKept, orderOf, overlaps, pruned, released, shifted, sliceAt, vaults } from "../../🟦️.ts";
import { CASES, box, body, stream, sharing, scattered, marching } from "../🧰️clearance-tools/🟦️.ts";

describe("claims", () => {
  it("cuts a path into slices of a span, each the smallest box around its ticks", () => {
    const path = marching(0, 0, 10, 1, 10);
    const exact = claimOf("a", 100, path, 1, path[9]!);
    expect(exact.owner).toBe("a");
    expect(exact.slices.length).toBe(10);
    expect(exact.slices[3]).toEqual({ from: 103, until: 103, extent: path[3] });
    expect(exact.rest).toEqual(path[9]);
    const coarse = claimOf("a", 100, path, 4, null);
    expect(coarse.slices).toEqual([
      { from: 100, until: 103, extent: box(0, 0, 70, 53) },
      { from: 104, until: 107, extent: box(40, 4, 110, 57) },
      { from: 108, until: 109, extent: box(80, 8, 130, 59) },
    ]);
    expect(coarse.rest).toBeNull();
    expect(claimOf("a", 7, path, 0, null).slices.length).toBe(10);
    expect(claimOf("a", 7, path, 2.9, null).slices.length).toBe(5);
    expect(claimOf("a", 7, [], 3, null).slices).toEqual([]);
  });

  it("answers where a claim has its owner at a tick", () => {
    const path = marching(0, 0, 10, 0, 6);
    const claim = claimOf("a", 10, path, 2, box(500, 0, 540, 50));
    expect(sliceAt(claim, 9)).toBeNull();
    expect(sliceAt(claim, 10)).toEqual(box(0, 0, 50, 50));
    expect(sliceAt(claim, 13)).toEqual(box(20, 0, 70, 50));
    expect(sliceAt(claim, 15)).toEqual(box(40, 0, 90, 50));
    expect(sliceAt(claim, 16)).toEqual(box(500, 0, 540, 50));
    expect(sliceAt(claimOf("a", 10, path, 2, null), 16)).toBeNull();
    expect(sliceAt(claimOf("a", 10, [], 2, box(0, 0, 1, 1)), 16)).toBeNull();
  });

  it("refuses a plan that runs into a body that stays, and ignores the owner's own body", () => {
    const path = marching(0, 0, 10, 0, 20);
    const plan = claimOf("a", 0, path, 1, path[19]!);
    expect(claimClear(plan, [body("a", 0, 0, 40, 50)], [])).toBe(true);
    expect(claimClear(plan, [body("a", 0, 0, 40, 50), body("b", 100, 60, 140, 110)], [])).toBe(true);
    expect(claimClear(plan, [body("a", 0, 0, 40, 50), body("b", 100, 40, 140, 90)], [])).toBe(false);
    expect(claimClear(plan, [body("b", 230, 0, 270, 50)], [])).toBe(true);
    expect(claimClear(plan, [body("b", 229, 0, 269, 50)], [])).toBe(false);
  });

  it("lets two plans cross one place at different ticks and refuses them at the same tick", () => {
    const across = claimOf("a", 20, marching(0, 100, 10, 0, 21), 1, box(200, 100, 240, 150));
    const down = (from: number): Claim => claimOf("b", from, marching(100, -100, 0, 10, 31), 1, box(100, 200, 140, 250));
    expect(claimClear(down(0), [], [across])).toBe(true);
    expect(claimClear(down(10), [], [across])).toBe(false);
    expect(claimClear(down(30), [], [across])).toBe(true);
    expect(claimClear(across, [], [down(0)])).toBe(true);
    expect(claimClear(across, [], [down(10)])).toBe(false);
    expect(claimClear(across, [], [down(30)])).toBe(true);
  });

  it("refuses a rest where somebody arrives later, and a path through somebody's rest", () => {
    const late = claimOf("a", 0, marching(0, 0, 10, 0, 51), 1, box(500, 0, 540, 50));
    const early = claimOf("b", 0, marching(500, -300, 0, 10, 31), 1, box(500, 0, 540, 50));
    expect(claimClear(early, [], [late])).toBe(false);
    expect(claimClear(claimOf("b", 0, marching(500, -300, 0, 10, 31), 1, null), [], [late])).toBe(true);
    expect(claimClear(claimOf("b", 0, marching(560, -300, 0, 10, 31), 1, box(560, 0, 600, 50)), [], [late])).toBe(true);
    const through = claimOf("b", 100, marching(500, -300, 0, 10, 61), 1, box(500, 300, 540, 350));
    expect(claimClear(through, [], [late])).toBe(false);
    expect(claimClear(through, [], [claimOf("a", 0, marching(0, 0, 10, 0, 51), 1, null)])).toBe(true);
  });

  it("takes a planned body to be where its claim says, and where it stands until that claim begins", () => {
    const leaving = claimOf("a", 0, marching(100, 0, 10, 0, 31), 1, box(400, 0, 440, 50));
    const landing = claimOf("b", 0, marching(100, -300, 0, 10, 31), 1, box(100, 0, 140, 50));
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [leaving])).toBe(true);
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [])).toBe(false);
    const later = claimOf("a", 40, marching(300, 0, 10, 0, 11), 1, box(400, 0, 440, 50));
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [later])).toBe(false);
    expect(claimClear(landing, [], [later])).toBe(true);
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [claimOf("c", 0, marching(900, 0, 0, 0, 5), 1, null), leaving])).toBe(true);
    expect(claimClear(landing, [body("a", 100, 0, 140, 50)], [leaving, claimOf("a", 0, marching(100, 0, 0, 0, 40), 1, null)])).toBe(false);
  });

  it("agrees with a comparison of every tick of every time line", () => {
    const next = stream(37);
    const horizon = 60;
    /** 🧵️ The box of an owner at every tick of the horizon: its body until its claim begins, its slices, then its rest. */
    const line = (own: Extent | null, claim: Claim | null): (Extent | null)[] => {
      const boxes: (Extent | null)[] = [];
      for (let tick = 0; tick <= horizon; tick++) {
        if (claim === null || claim.slices.length === 0) boxes.push(own);
        else if (tick < claim.slices[0]!.from) boxes.push(own);
        else boxes.push(sliceAt(claim, tick));
      }
      return boxes;
    };
    let clear = 0;
    for (let round = 0; round < CASES; round++) {
      const others = 1 + next(4);
      const bodies: Body[] = [];
      const claims: Claim[] = [];
      for (let index = 0; index < others; index++) {
        const owner = `o${index}`;
        const x = next(60) * 4;
        const y = next(40) * 4;
        if (next(4) > 0) bodies.push(body(owner, x, y, x + 40, y + 50));
        if (next(2) > 0) {
          const path = marching(x, y, next(9) - 4, next(9) - 4, 1 + next(30));
          claims.push(claimOf(owner, next(20), path, 1 + next(4), next(3) > 0 ? path[path.length - 1]! : null));
        }
      }
      const from = next(20);
      const path = marching(next(60) * 4, next(40) * 4, next(9) - 4, next(9) - 4, 1 + next(30));
      const plan = claimOf("me", from, path, 1 + next(4), next(3) > 0 ? path[path.length - 1]! : null);
      let expected = true;
      const mine = line(null, plan);
      for (let index = 0; index < others; index++) {
        const owner = `o${index}`;
        const theirs = line(bodies.find((entry) => entry.owner === owner)?.extent ?? null, claims.find((entry) => entry.owner === owner) ?? null);
        for (let tick = from; tick <= horizon; tick++) if (mine[tick] !== null && theirs[tick] !== null && sharing(mine[tick]!, theirs[tick]!)) expected = false;
      }
      if (expected) clear++;
      expect(claimClear(plan, bodies, claims), `round ${round}`).toBe(expected);
    }
    expect(clear).toBeGreaterThan(CASES / 20);
    expect(clear).toBeLessThan(CASES - CASES / 20);
  });

  it("releases the claims of an owner and prunes what is over", () => {
    const one = claimOf("a", 0, marching(0, 0, 1, 0, 10), 5, box(9, 0, 49, 50));
    const two = claimOf("b", 3, marching(0, 0, 1, 0, 4), 2, null);
    expect(released([one, two], "a")).toEqual([two]);
    expect(released([one, two], "c")).toEqual([one, two]);
    expect(pruned([one, two], 0)).toEqual([one, two]);
    expect(pruned([one, two], 5)).toEqual([{ owner: "a", slices: [one.slices[1]], rest: one.rest }, { owner: "b", slices: [two.slices[1]], rest: null }]);
    expect(pruned([one, two], 7)).toEqual([{ owner: "a", slices: [one.slices[1]], rest: one.rest }]);
    expect(pruned([one, two], 10)).toEqual([]);
  });
});

describe("order on a perch", () => {
  const extent = box(80, 250, 120, 300);

  it("cuts a stride short a seam before the first obstacle ahead that shares its height", () => {
    expect(guardedStride(extent, 5, [])).toBe(5);
    expect(guardedStride(extent, -5, [])).toBe(-5);
    expect(guardedStride(extent, 0, [box(0, 0, 1000, 1000)])).toBe(0);
    expect(guardedStride(extent, 5, [box(123, 260, 160, 300)])).toBe(3 - SEAM);
    expect(guardedStride(extent, 2, [box(123, 260, 160, 300)])).toBe(2);
    expect(guardedStride(extent, 5, [box(120.01, 260, 160, 300)])).toBe(0);
    expect(guardedStride(extent, -5, [box(40, 260, 77, 300)])).toBe(SEAM - 3);
    expect(guardedStride(extent, -5, [box(123, 260, 160, 300)])).toBe(-5);
    expect(guardedStride(extent, 5, [box(40, 260, 77, 300)])).toBe(5);
    expect(guardedStride(extent, 5, [box(123, 100, 160, 250)])).toBe(5);
    expect(guardedStride(extent, 5, [box(123, 300, 160, 350)])).toBe(5);
    expect(guardedStride(extent, 5, [box(200, 260, 240, 300), box(123, 260, 160, 300), box(122, 100, 160, 200)])).toBe(3 - SEAM);
  });

  it("lets a body out of an obstacle it overlaps, never further in", () => {
    const inside = [box(110, 260, 150, 300)];
    expect(guardedStride(extent, 5, inside)).toBe(0);
    expect(guardedStride(extent, -5, inside)).toBe(-5);
    expect(guardedStride(extent, 5, [box(60, 260, 100, 300)])).toBe(5);
    expect(guardedStride(extent, -5, [box(60, 260, 100, 300)])).toBe(0);
    expect(guardedStride(extent, 5, [box(60, 260, 140, 300)])).toBe(5);
    expect(guardedStride(extent, -5, [box(60, 260, 140, 300)])).toBe(-5);
  });

  it("stops before the corridors and the rests of claims as before bodies", () => {
    const claims: Claim[] = [{ owner: "c", slices: [{ from: 5, until: 5, extent: box(122, 0, 160, 300) }, { from: 6, until: 9, extent: box(124, 0, 160, 300) }], rest: box(126, 250, 160, 300) }];
    expect(guardedStride(extent, 9, obstaclesOf("a", [], claims, 5))).toBe(2 - SEAM);
    expect(guardedStride(extent, 9, obstaclesOf("a", [], claims, 6))).toBe(4 - SEAM);
    expect(guardedStride(extent, 9, obstaclesOf("a", [], claims, 10))).toBe(6 - SEAM);
    expect(guardedStride(extent, 9, obstaclesOf("c", [], claims, 5))).toBe(9);
  });

  it("agrees with a walk along a 1/64 px lattice that stops at the first place too close to an obstacle", () => {
    const next = stream(41);
    let hindered = 0;
    for (let round = 0; round < CASES; round++) {
      const x = 150 + next(200) / 4;
      const own = box(x - 20, 250, x + 20, 300);
      const stride = (next(161) - 80) / 4;
      const side = stride < 0 ? -1 : 1;
      const near = x + 20 + next(80) / 4;
      const far = x - 20 - next(80) / 4;
      const flank = side > 0 ? box(near, 240 + next(40), near + 10 + next(80) / 4, 310) : box(far - 10 - next(80) / 4, 240 + next(40), far, 310);
      const obstacles = [...scattered(next, 1 + next(5)), flank].filter((obstacle) => !sharing({ x0: own.x0 - SEAM, y0: own.y0, x1: own.x1 + SEAM, y1: own.y1 }, obstacle));
      let reach = 0;
      for (let step = 1; step <= Math.abs(stride) * 64; step++) {
        const there = shifted(own, (side * step) / 64, 0);
        if (obstacles.some((obstacle) => sharing({ x0: there.x0 - SEAM, y0: there.y0, x1: there.x1 + SEAM, y1: there.y1 }, obstacle))) break;
        reach = step / 64;
      }
      if (reach < Math.abs(stride)) hindered++;
      expect(guardedStride(own, stride, obstacles), `round ${round}`).toBe(reach === 0 ? 0 : side * reach);
    }
    expect(hindered).toBeGreaterThan(CASES / 5);
  });

  it("keeps walkers on one line apart and in their order, which nothing but the guard does", () => {
    const ticks = sampled(400, 4000, 40000);
    for (const guarded of [true, false]) {
      const next = stream(5);
      const walkers: Body[] = [];
      let x = 0;
      for (let index = 0; index < 7; index++) {
        const width = 20 + next(21);
        walkers.push(body(`w${index}`, x, 0, x + width, 40));
        x = x + width + SEAM + next(40) / 8;
      }
      const order = orderOf(walkers);
      let collisions = 0;
      let swaps = 0;
      for (let tick = 0; tick < ticks; tick++) {
        for (let turn = 0; turn < walkers.length; turn++) {
          const index = (tick + turn) % walkers.length;
          const walker = walkers[index]!;
          const wanted = (next(49) - 24) / 8;
          const stride = guarded ? guardedStride(walker.extent, wanted, obstaclesOf(walker.owner, walkers, [], tick)) : wanted;
          walkers[index] = { owner: walker.owner, extent: shifted(walker.extent, stride, 0) };
        }
        if (overlaps(walkers).length > 0) collisions++;
        if (!orderKept(order, orderOf(walkers))) swaps++;
      }
      if (guarded) expect([collisions, swaps]).toEqual([0, 0]);
      else {
        expect(collisions).toBeGreaterThan(ticks / 10);
        expect(swaps).toBeGreaterThan(0);
      }
    }
  });

  it("ranks bodies from left to right and tells whether an order was kept", () => {
    const bodies = [body("c", 50, 0, 60, 10), body("a", 0, 0, 10, 10), body("b", 20, 0, 40, 10), body("d", 25, 0, 35, 10)];
    expect(orderOf(bodies)).toEqual(["a", "b", "d", "c"]);
    expect(orderOf([])).toEqual([]);
    expect(orderKept(["a", "b", "c"], ["a", "b", "c"])).toBe(true);
    expect(orderKept(["a", "b", "c"], ["a", "x", "c"])).toBe(true);
    expect(orderKept(["a", "b", "c"], ["c", "y", "a"])).toBe(false);
    expect(orderKept(["a", "b", "c"], ["b", "a"])).toBe(false);
    expect(orderKept(["a", "b", "c"], [])).toBe(true);
    expect(orderKept([], ["a"])).toBe(true);
  });

  it("allows a swap by a hop only when the rise lifts the hopper over its neighbour", () => {
    const hopper = box(80, 250, 120, 304);
    const hurdle = box(130, 244, 170, 304);
    expect(vaults(hopper, 60 + SEAM, hurdle)).toBe(true);
    expect(vaults(hopper, 60, hurdle)).toBe(false);
    expect(vaults(hopper, 84, box(130, 219, 170, 304))).toBe(false);
    expect(vaults(hopper, 84, box(130, 221, 170, 304))).toBe(true);
  });

  it("never finds a hop over a neighbour clear whose rise does not vault it", () => {
    const size = { width: 30, height: 40 };
    const from = { x: 100, y: 300 };
    let vaulted = 0;
    let refused = 0;
    for (let dx = 60; dx <= 160; dx += 5) {
      const to = { x: from.x + dx, y: 300 };
      const hop = hopOf(from, to);
      if (hop === null) continue;
      const path: Extent[] = [bodyOf("h", from, size, 0, UPRIGHT, MARGIN).extent];
      let flight = { x: from.x, y: from.y, vx: hop.vx, vy: hop.vy };
      for (let left = hop.ticks; left >= 1; left--) {
        flight = hopStep(flight.x, flight.y, flight.vx, flight.vy, to, left);
        path.push(bodyOf("h", flight, size, 0, UPRIGHT, MARGIN).extent);
      }
      const plan = claimOf("h", 0, path, 1, path[path.length - 1]!);
      const rise = path[0]!.y1 - Math.min(...path.map((extent) => extent.y1));
      for (let height = 4; height <= 60; height += 4) {
        for (let width = 4; width <= 40; width += 12) {
          const hurdle = bodyOf("n", { x: from.x + dx / 2, y: 300 }, { width, height }, 0, UPRIGHT, MARGIN);
          if (claimClear(plan, [hurdle], [])) {
            vaulted++;
            expect(vaults(path[0]!, rise, hurdle.extent), `dx ${dx} height ${height} width ${width}`).toBe(true);
          } else refused++;
        }
      }
    }
    expect(vaulted).toBeGreaterThan(20);
    expect(refused).toBeGreaterThan(20);
  });
});
