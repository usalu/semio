/** 🪑️ Unit suite of the clearance module, seating and heads: the nearest valid seating of pooled neighbours, landings on heads and slides off them, held bodies pushed out of obstacles and the last resort of a pet without a clear plan.
 *
 * @see ../🧰️clearance-tools/🟦️.ts — the builders of the cases
 * @see ../../🟦️.ts — the implementation under test
 */
import { describe, expect, it } from "vitest";
import type { Perch } from "../../../../🧬️schema/🟦️.ts";
import { type Body, MARGIN, PUSHES, SEAM, claimOf, evicted, freeAmong, headUnder, liftOnto, mustPoof, orderOf, overlaps, pushedOut, restsOn, scootFraction, scooted, seatOf, shifted, slideOf, slideSide, slideStride } from "../../🟦️.ts";
import { CASES, box, body, stream, scattered, marching } from "../🧰️clearance-tools/🟦️.ts";

describe("seating", () => {
  const perch: Perch = { surface: "s", x0: 100, x1: 400, y: 300 };

  /** 💺️ The boxes after a seating, in the order of its seats. */
  const seated = (bodies: readonly Body[], seating: { readonly seats: readonly { readonly owner: string; readonly shift: number }[] }): Body[] => seating.seats.map((seat) => ({ owner: seat.owner, extent: shifted(bodies.find((entry) => entry.owner === seat.owner)!.extent, seat.shift, 0) }));

  it("leaves a valid seating exactly as it is", () => {
    const bodies = [body("b", 220, 248, 268, 304), body("a", 150, 248, 198, 304), body("c", 268 + SEAM, 248, 300, 304)];
    expect(seatOf(bodies, perch, MARGIN)).toEqual({ seats: [{ owner: "a", shift: 0 }, { owner: "b", shift: 0 }, { owner: "c", shift: 0 }], leavers: [] });
    expect(seatOf([], perch, MARGIN)).toEqual({ seats: [], leavers: [] });
  });

  it("moves two bodies that overlap apart by the same amount, a seam between them", () => {
    const seating = seatOf([body("a", 150, 248, 198, 304), body("b", 190, 248, 238, 304)], perch, MARGIN);
    expect(seating).toEqual({ seats: [{ owner: "a", shift: -(4 + SEAM / 2) }, { owner: "b", shift: 4 + SEAM / 2 }], leavers: [] });
  });

  it("pools a whole run of violators into one block", () => {
    const bodies = [body("a", 200, 248, 240, 304), body("b", 210, 248, 250, 304), body("c", 220, 248, 260, 304), body("d", 300, 248, 340, 304)];
    const after = seated(bodies, seatOf(bodies, perch, MARGIN));
    expect(after.map((entry) => entry.extent.x0)).toEqual([170 - SEAM, 210, 250 + SEAM, 300]);
  });

  it("brings bodies back onto the perch, their margin beyond its ends", () => {
    expect(seatOf([body("a", 60, 248, 108, 304)], perch, MARGIN).seats).toEqual([{ owner: "a", shift: 36 }]);
    expect(seatOf([body("a", 380, 248, 428, 304)], perch, MARGIN).seats).toEqual([{ owner: "a", shift: -24 }]);
    const bodies = [body("a", 300, 248, 348, 304), body("b", 352, 248, 400, 304)];
    const short: Perch = { surface: "s", x0: 100, x1: 380, y: 300 };
    expect(seatOf(bodies, short, MARGIN).seats).toEqual([{ owner: "a", shift: -(12 + SEAM) }, { owner: "b", shift: -16 }]);
    expect(seated(bodies, seatOf(bodies, short, MARGIN))[1]!.extent.x1).toBe(short.x1 + MARGIN);
  });

  it("names whoever does not fit for leaving, the last in the list first", () => {
    const narrow: Perch = { surface: "s", x0: 100, x1: 200, y: 300 };
    const bodies = [body("c", 180, 248, 228, 304), body("a", 90, 248, 138, 304), body("b", 130, 248, 178, 304)];
    const seating = seatOf(bodies, narrow, MARGIN);
    expect(seating.leavers).toEqual(["b"]);
    expect(seating.seats.map((seat) => seat.owner)).toEqual(["a", "c"]);
    expect(overlaps(seated(bodies, seating))).toEqual([]);
    expect(seatOf([body("a", 90, 248, 210, 304)], narrow, MARGIN)).toEqual({ seats: [], leavers: ["a"] });
    expect(seatOf([body("a", 90, 248, 198, 304)], narrow, MARGIN).leavers).toEqual([]);
    expect(seatOf([body("a", 0, 248, 60, 304), body("b", 0, 248, 60, 304), body("c", 0, 248, 60, 304), body("d", 0, 248, 60, 304)], narrow, MARGIN).leavers).toEqual(["d", "c", "b"]);
  });

  it("is the nearest valid seating: it agrees with a search over every way to pool neighbours", () => {
    const next = stream(53);
    let moved = 0;
    for (let round = 0; round < CASES; round++) {
      const count = 1 + next(5);
      const bodies: Body[] = [];
      for (let index = 0; index < count; index++) {
        const x0 = 80 + next(1000) / 4;
        bodies.push(body(`s${index}`, x0, 248, x0 + 30 + next(30), 304));
      }
      const seating = seatOf(bodies, perch, MARGIN);
      if (seating.leavers.length > 0) continue;
      const order = orderOf(bodies).map((owner) => bodies.find((entry) => entry.owner === owner)!);
      expect(seating.seats.map((seat) => seat.owner)).toEqual(order.map((entry) => entry.owner));
      const offsets = [0];
      for (let at = 1; at < count; at++) offsets.push(offsets[at - 1]! + (order[at - 1]!.extent.x1 - order[at - 1]!.extent.x0) + SEAM);
      const wanted = order.map((entry, at) => entry.extent.x0 - offsets[at]!);
      const low = perch.x0 - MARGIN;
      const high = perch.x1 + MARGIN - (order[count - 1]!.extent.x1 - order[count - 1]!.extent.x0) - offsets[count - 1]!;
      let best: number[] = [];
      let least = Infinity;
      for (let cuts = 0; cuts < 1 << (count - 1); cuts++) {
        const values: number[] = [];
        let start = 0;
        for (let at = 0; at < count; at++) {
          if (at < count - 1 && (cuts & (1 << at)) === 0) continue;
          const block = wanted.slice(start, at + 1);
          const mean = Math.min(Math.max(block.reduce((sum, value) => sum + value, 0) / block.length, low), high);
          for (let member = start; member <= at; member++) values.push(mean);
          start = at + 1;
        }
        if (values.some((value, at) => at > 0 && value < values[at - 1]!)) continue;
        const cost = values.reduce((sum, value, at) => sum + (value - wanted[at]!) ** 2, 0);
        if (cost < least - 1e-9) {
          least = cost;
          best = values;
        }
      }
      seating.seats.forEach((seat, at) => expect(Math.abs(seat.shift - (best[at]! - wanted[at]!)), `round ${round} seat ${at}`).toBeLessThan(1e-9));
      if (seating.seats.some((seat) => seat.shift !== 0)) moved++;
      const after = seated(bodies, seating);
      expect(overlaps(after), `round ${round}`).toEqual([]);
      for (let at = 1; at < count; at++) expect(after[at]!.extent.x0 - after[at - 1]!.extent.x1, `round ${round}`).toBeGreaterThan(SEAM - 1e-9);
      expect(after[0]!.extent.x0).toBeGreaterThan(low - 1e-9);
      expect(after[count - 1]!.extent.x1).toBeLessThan(perch.x1 + MARGIN + 1e-9);
      for (const seat of seatOf(after, perch, MARGIN).seats) expect(Math.abs(seat.shift), `round ${round}`).toBeLessThan(1e-9);
    }
    expect(moved).toBeGreaterThan(CASES / 4);
  });

  it("scoots a group by one common share of the way, which never overlaps within the group", () => {
    expect(scootFraction([3, -6, 0], 1.5)).toBe(0.25);
    expect(scootFraction([3, -6, 0], 10)).toBe(1);
    expect(scootFraction([3, -6, 0], 6)).toBe(1);
    expect(scootFraction([0, 0], 1.5)).toBe(1);
    expect(scootFraction([], 1.5)).toBe(1);
    expect(scootFraction([3], 0)).toBe(0);
    expect(scootFraction([3], -2)).toBe(0);
    expect(scootFraction([0], -2)).toBe(1);
    expect(scooted(10, 20, 0.25)).toBe(12.5);
    expect(scooted(10, 20, 0)).toBe(10);
    expect(scooted(10, 20, 1)).toBe(20);
    expect(scooted(10, 20, 1.5)).toBe(20);
    const next = stream(67);
    for (let round = 0; round < CASES; round++) {
      const bodies: Body[] = [];
      let x = 100 + next(200) / 4;
      for (let index = 0; index < 2 + next(4); index++) {
        const width = 30 + next(30);
        bodies.push(body(`s${index}`, x, 248, x + width, 304));
        x = x + width + SEAM + next(160) / 4;
      }
      const shrunk: Perch = { surface: "s", x0: 100 + next(400) / 4, x1: 500 + next(800) / 4, y: 300 };
      const seating = seatOf(bodies, shrunk, MARGIN);
      const members = seating.seats.map((seat) => bodies.find((entry) => entry.owner === seat.owner)!);
      const goals = seating.seats.map((seat, at) => members[at]!.extent.x0 + seat.shift);
      let places = members.map((entry) => entry.extent.x0);
      let fraction = 0;
      for (let tick = 0; tick < 2000 && fraction < 1; tick++) {
        fraction = scootFraction(places.map((place, at) => goals[at]! - place), 1.125);
        places = places.map((place, at) => scooted(place, goals[at]!, fraction));
        const there = members.map((entry, at) => ({ owner: entry.owner, extent: shifted(entry.extent, places[at]! - entry.extent.x0, 0) }));
        expect(overlaps(there), `round ${round} tick ${tick}`).toEqual([]);
      }
      expect(places).toEqual(goals);
    }
  });
});

describe("heads", () => {
  const host = body("h", 100, 240, 150, 304);

  it("lands a descending body on the top it comes down onto, like a perch crossing", () => {
    const before = box(90, 180, 130, 236);
    const after = box(90, 190, 130, 246);
    expect(headUnder(before, after, "f", [host])).toBe(host);
    expect(liftOnto(after, host.extent)).toBe(-(6 + SEAM));
    expect(shifted(after, 0, liftOnto(after, host.extent)).y1).toBe(240 - SEAM);
    expect(headUnder(before, after, "h", [host])).toBeNull();
    expect(headUnder(shifted(before, -50, 0), shifted(after, -50, 0), "f", [host])).toBeNull();
    expect(headUnder(shifted(before, -30, 0), shifted(after, -29.75, 0), "f", [host])).toBe(host);
    expect(headUnder(shifted(before, -30, 0), shifted(after, -30, 0), "f", [host])).toBeNull();
    expect(headUnder(after, before, "f", [host])).toBeNull();
    expect(headUnder(shifted(before, 0, 14), shifted(after, 0, 14), "f", [host])).toBeNull();
    expect(headUnder(before, shifted(before, 0, 3), "f", [host])).toBeNull();
    expect(headUnder(before, shifted(before, 0, 4), "f", [host])).toBe(host);
    expect(headUnder(shifted(before, 0, 4), shifted(before, 0, 4.25), "f", [host])).toBe(host);
    expect(headUnder(before, after, "f", [])).toBeNull();
  });

  it("chooses the highest head, and the first one among equals", () => {
    const before = box(90, 100, 180, 200);
    const after = box(90, 160, 180, 260);
    const taller = body("t", 150, 220, 200, 304);
    const twin = body("w", 60, 220, 95, 304);
    expect(headUnder(before, after, "f", [host, taller])).toBe(taller);
    expect(headUnder(before, after, "f", [taller, host])).toBe(taller);
    expect(headUnder(before, after, "f", [host, twin, taller])).toBe(twin);
  });

  it("tells whether a rider still stands on its host", () => {
    const rider = box(90, 180, 130, 240 - SEAM);
    expect(restsOn(rider, host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 0, SEAM), host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 0, -SEAM), host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 0, -2 * SEAM), host.extent)).toBe(false);
    expect(restsOn(shifted(rider, 0, 2 * SEAM), host.extent)).toBe(false);
    expect(restsOn(shifted(rider, -30, 0), host.extent)).toBe(false);
    expect(restsOn(shifted(rider, -29.75, 0), host.extent)).toBe(true);
    expect(restsOn(shifted(rider, 60, 0), host.extent)).toBe(false);
  });

  it("slides off to the nearer side, faster and faster up to a limit", () => {
    expect(slideSide(box(110, 0, 150, 10), host.extent)).toBe(1);
    expect(slideSide(box(105, 0, 145, 10), host.extent)).toBe(1);
    expect(slideSide(box(104, 0, 145, 10), host.extent)).toBe(-1);
    expect(slideStride(0)).toBe(0.5);
    expect(slideStride(10)).toBe(0.8125);
    expect(slideStride(48)).toBe(2);
    expect(slideStride(1000)).toBe(2);
    for (let tick = 1; tick < 100; tick++) expect(slideStride(tick)).toBeGreaterThanOrEqual(slideStride(tick - 1));
  });

  it("guards a slide, keeps it on the stage and turns it round where it cannot move", () => {
    const rider = box(90, 180, 130, 240 - SEAM);
    expect(slideOf(rider, 1, 0, 0, 960, [host.extent])).toEqual({ stride: 0.5, side: 1 });
    expect(slideOf(rider, -1, 10, 0, 960, [host.extent])).toEqual({ stride: -0.8125, side: -1 });
    expect(slideOf(rider, 1, 0, 0, 960, [box(130.25, 100, 200, 235)])).toEqual({ stride: 0.25 - SEAM, side: 1 });
    expect(slideOf(rider, 1, 0, 0, 960, [box(130 + SEAM, 100, 200, 235)])).toEqual({ stride: 0, side: -1 });
    expect(slideOf(rider, -1, 0, 0, 960, [box(40, 100, 90, 235)])).toEqual({ stride: 0, side: 1 });
    expect(slideOf(rider, 1, 0, 0, 130, [])).toEqual({ stride: 0, side: -1 });
    expect(slideOf(rider, 1, 0, 0, 130.25, [])).toEqual({ stride: 0.25, side: 1 });
    expect(slideOf(rider, -1, 0, 90, 960, [])).toEqual({ stride: 0, side: 1 });
    expect(slideOf(rider, -1, 0, 89.75, 960, [])).toEqual({ stride: -0.25, side: -1 });
  });
});

describe("held", () => {
  const extent = box(100, 100, 140, 150);

  it("takes the shortest way out of an obstacle, a seam to spare", () => {
    expect(pushedOut(extent, [], PUSHES)).toEqual({ x: 0, y: 0 });
    expect(pushedOut(extent, [box(140, 100, 200, 150), box(0, 150, 300, 200)], PUSHES)).toEqual({ x: 0, y: 0 });
    expect(pushedOut(extent, [box(130, 90, 200, 160)], PUSHES)).toEqual({ x: -(10 + SEAM), y: 0 });
    expect(pushedOut(extent, [box(40, 90, 108, 160)], PUSHES)).toEqual({ x: 8 + SEAM, y: 0 });
    expect(pushedOut(extent, [box(90, 140, 150, 200)], PUSHES)).toEqual({ x: 0, y: -(10 + SEAM) });
    expect(pushedOut(extent, [box(90, 60, 150, 105)], PUSHES)).toEqual({ x: 0, y: 5 + SEAM });
    expect(pushedOut(extent, [box(110, 0, 130, 300)], PUSHES)).toEqual({ x: -(30 + SEAM), y: 0 });
    expect(pushedOut(extent, [box(0, 115, 300, 135)], PUSHES)).toEqual({ x: 0, y: -(35 + SEAM) });
  });

  it("goes on from obstacle to obstacle and gives up when its rounds are over", () => {
    const pair = [box(130, 90, 200, 160), box(80, 60, 125, 105)];
    expect(pushedOut(extent, pair, PUSHES)).toEqual({ x: -(10 + SEAM), y: 5 + SEAM });
    expect(pushedOut(extent, pair, 1)).toEqual({ x: -(10 + SEAM), y: 5 + SEAM });
    expect(pushedOut(extent, [pair[1]!, pair[0]!], 1)).toEqual({ x: -(10 + SEAM), y: 5 + SEAM });
    expect(pushedOut(extent, [box(130, 90, 200, 160), box(60, 90, 95, 160)], PUSHES)).toBeNull();
    const walls = [box(0, 0, 105, 300), box(135, 0, 300, 300)];
    expect(pushedOut(extent, walls, PUSHES)).toBeNull();
    expect(pushedOut(extent, walls, 100)).toBeNull();
    expect(pushedOut(extent, [box(130, 90, 200, 160)], 0)).toBeNull();
    expect(pushedOut(extent, [box(140, 90, 200, 160)], 0)).toEqual({ x: 0, y: 0 });
  });

  it("answers a free place or nothing on generated crowds", () => {
    const next = stream(71);
    let freed = 0;
    let stuck = 0;
    for (let round = 0; round < CASES; round++) {
      const own = box(100, 250, 140, 300);
      const obstacles = scattered(next, 1 + next(4));
      const push = pushedOut(own, obstacles, PUSHES);
      if (push === null) {
        stuck++;
        continue;
      }
      expect(freeAmong(shifted(own, push.x, push.y), obstacles), `round ${round}`).toBe(true);
      if (!freeAmong(own, obstacles)) freed++;
      else expect(push).toEqual({ x: 0, y: 0 });
      if (obstacles.length === 1 && !freeAmong(own, obstacles)) {
        const only = obstacles[0]!;
        const ways = [own.x1 - only.x0, only.x1 - own.x0, own.y1 - only.y0, only.y1 - own.y0];
        expect(Math.abs(push.x) + Math.abs(push.y), `round ${round}`).toBe(Math.min(...ways) + SEAM);
      }
    }
    expect(freed).toBeGreaterThan(CASES / 10);
    expect(stuck).toBeLessThan(CASES / 2);
  });
});

describe("the last resort", () => {
  const bodies = [body("a", 0, 0, 40, 50), body("b", 100, 0, 140, 50)];
  const clear = claimOf("a", 0, marching(0, 0, 0, 10, 10), 1, box(0, 90, 40, 140));
  const blocked = claimOf("a", 0, marching(0, 0, 10, 0, 10), 1, box(90, 0, 130, 50));

  it("lets a pet be as long as it may stay or one of its plans is clear", () => {
    expect(mustPoof("a", box(0, 0, 40, 50), [], bodies, [], 0)).toBe(false);
    expect(mustPoof("a", box(0, 0, 40, 50), [blocked], bodies, [], 0)).toBe(false);
    expect(mustPoof("a", null, [blocked, clear], bodies, [], 0)).toBe(false);
    expect(mustPoof("a", box(70, 0, 110, 50), [blocked, clear], bodies, [], 0)).toBe(false);
  });

  it("poofs a pet that may not stay and has no clear plan", () => {
    expect(mustPoof("a", null, [], bodies, [], 0)).toBe(true);
    expect(mustPoof("a", null, [blocked], bodies, [], 0)).toBe(true);
    expect(mustPoof("a", box(70, 0, 110, 50), [blocked], bodies, [], 0)).toBe(true);
    const corridor = [claimOf("c", 5, marching(0, -100, 0, 20, 6), 1, null)];
    expect(mustPoof("a", box(0, 0, 40, 50), [], bodies, corridor, 5)).toBe(true);
    expect(mustPoof("a", box(0, 0, 40, 50), [], bodies, corridor, 11)).toBe(false);
    expect(mustPoof("a", box(0, 0, 40, 50), [blocked], bodies, corridor, 5)).toBe(true);
    expect(mustPoof("a", box(0, 0, 40, 50), [blocked, clear], bodies, corridor, 5)).toBe(false);
  });

  it("names whoever a change from outside carried into somebody who stays", () => {
    expect(evicted(bodies, [], 0)).toEqual([]);
    expect(evicted([...bodies, body("c", 30, 0, 70, 50)], [], 0)).toEqual(["c"]);
    expect(evicted([body("c", 30, 0, 70, 50), ...bodies], [], 0)).toEqual(["a"]);
    expect(evicted([...bodies, body("c", 30, 0, 70, 50), body("d", 60, 0, 90, 50)], [], 0)).toEqual(["c"]);
    expect(evicted([...bodies, body("c", 30, 0, 110, 50), body("d", 60, 0, 90, 50)], [], 0)).toEqual(["c"]);
    expect(evicted([...bodies, body("d", 60, 0, 90, 50), body("c", 30, 0, 110, 50)], [], 0)).toEqual(["c"]);
  });

  it("evicts a body without a plan from the corridors and rests of those who stay", () => {
    const plan = claimOf("b", 0, marching(100, 0, 20, 0, 10), 1, box(300, 0, 340, 50));
    const rider = body("c", 200, 0, 240, 50);
    expect(evicted([...bodies, rider], [plan], 0)).toEqual(["c"]);
    expect(evicted([...bodies, rider], [plan], 8)).toEqual([]);
    expect(evicted([...bodies, body("c", 310, 0, 350, 50)], [plan], 100)).toEqual(["c"]);
    expect(evicted([...bodies, rider], [plan, claimOf("c", 0, marching(200, 0, 0, 0, 3), 1, null)], 0)).toEqual([]);
    expect(evicted([body("a", 0, 0, 40, 50), body("b", 30, 0, 70, 50), rider], [plan], 0)).toEqual(["b"]);
  });
});
