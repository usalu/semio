import { add as mathAdd, max as mathMax, min as mathMin } from "mathjs";
import { describe, expect, it } from "vitest";
import { CHALLENGES, CHALLENGE_RULES, CLOCK_LEAD, Mt19937, REACH_FACTOR, REACH_SLACK, TASK_KINDS, TASK_SECONDS, acted, challengeMeets, challengeRank, challengeRules, misses, points, reach, taskSeconds } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { oracleMisses, oracleReach } from "../🧱️challenge-ladder-kit/🟦️.ts";

describe("the rule table", () => {
  it("adds one step per challenge: hints and keys, keys, neither, a clock", () => {
    expect(CHALLENGES).toEqual(["easy", "medium", "hard", "expert"]);
    expect(CHALLENGE_RULES).toEqual({
      easy: { keys: true, hints: true, timed: false, par: 100 },
      medium: { keys: true, hints: false, timed: false, par: 200 },
      hard: { keys: false, hints: false, timed: false, par: 300 },
      expert: { keys: false, hints: false, timed: true, par: 400 },
    });
    for (const challenge of CHALLENGES) expect(challengeRules(challenge)).toBe(CHALLENGE_RULES[challenge]);
    expect(Object.keys(CHALLENGE_RULES)).toEqual([...CHALLENGES]);
  });

  it("hints only where the keys show and keeps the time only where they are hidden", () => {
    for (const challenge of CHALLENGES) {
      const rules = challengeRules(challenge);
      if (rules.hints) expect(rules.keys).toBe(true);
      if (rules.timed) expect(rules.keys).toBe(false);
    }
  });

  it("ranks the challenges 0 to 3 in their order, with a par that grows with the rank", () => {
    expect(CHALLENGES.map(challengeRank)).toEqual([0, 1, 2, 3]);
    for (const challenge of CHALLENGES) expect(challengeRules(challenge).par).toBe(100 * (challengeRank(challenge) + 1));
  });

  it("meets a least challenge exactly when its rank is not lower", () => {
    for (const challenge of CHALLENGES) for (const least of CHALLENGES) expect(challengeMeets(challenge, least)).toBe(CHALLENGES.indexOf(challenge) >= CHALLENGES.indexOf(least));
    expect(CHALLENGES.filter((challenge) => challengeMeets(challenge, "hard"))).toEqual(["hard", "expert"]);
    expect(CHALLENGES.filter((challenge) => challengeMeets("medium", challenge))).toEqual(["easy", "medium"]);
  });
});

describe("points", () => {
  it("are score × par: more for the same accuracy on a harder challenge, at most the par", () => {
    expect(CHALLENGES.map((challenge) => points(1, challenge))).toEqual([100, 200, 300, 400]);
    expect(CHALLENGES.map((challenge) => points(0, challenge))).toEqual([0, 0, 0, 0]);
    expect(CHALLENGES.map((challenge) => points(0.87, challenge))).toEqual([0.87 * 100, 0.87 * 200, 0.87 * 300, 0.87 * 400]);
    expect(points(0.5, "expert")).toBe(points(1, "medium"));
    const random = new Mt19937(7);
    for (let round = 0; round < 500; round++) {
      const score = random.next() / 0xffffffff;
      const earned = CHALLENGES.map((challenge) => points(score, challenge));
      expect(earned).toEqual([...earned].sort((left, right) => left - right));
      for (const [index, challenge] of CHALLENGES.entries()) expect(earned[index]).toBeLessThanOrEqual(challengeRules(challenge).par);
    }
  });
});

describe("reach", () => {
  it("is the factor 1000 on a logarithmic scale, or the square root of the values' max/min ratio where that is less", () => {
    expect(REACH_FACTOR).toBe(1000);
    expect(reach([1, 1e9], "logarithmic")).toBe(1000);
    expect(reach([1, 1e6], "logarithmic")).toBe(1000);
    expect(reach([10, 1000], "logarithmic")).toBe(10);
    expect(reach([1, 10, 100, 1e4], "logarithmic")).toBe(100);
    expect(reach([18, 120], "logarithmic")).toBe(Math.sqrt(120 / 18));
    expect(reach([1e-30, 1e30], "logarithmic")).toBe(1000);
  });

  it("is always half the spread on a linear scale", () => {
    expect(reach([0, 10, 20], "linear")).toBe(10);
    expect(reach([-5, 5], "linear")).toBe(5);
    expect(reach([0, 1e12], "linear")).toBe(5e11);
    expect(reach([1.4, 0.5, 0.24, 0.12], "linear")).toBe((1.4 - 0.12) / 2);
  });

  it("is the cap itself when the values do not spread: the factor 1000, or without bound", () => {
    expect(reach([5, 5, 5], "logarithmic")).toBe(1000);
    expect(reach([5], "logarithmic")).toBe(1000);
    expect(reach([], "logarithmic")).toBe(1000);
    expect(reach([5, 5, 5], "linear")).toBe(Infinity);
    expect(reach([5], "linear")).toBe(Infinity);
    expect(reach([], "linear")).toBe(Infinity);
  });

  it("depends on the set of values, not on their order", () => {
    const random = new Mt19937(12);
    for (let round = 0; round < 200; round++) {
      const values = Array.from({ length: 2 + (round % 7) }, () => 10 ** ((random.next() % 12000) / 1000));
      for (const scale of ["linear", "logarithmic"] as const) expect(reach([...values].reverse(), scale)).toBe(reach(values, scale));
    }
  });

  it("agrees with the same formula over mathjs", () => {
    const random = new Mt19937(2027);
    for (let round = 0; round < 500; round++) {
      const exponent = round % 2 === 0 ? 2 : 12;
      const values = Array.from({ length: 1 + (round % 8) }, () => 10 ** ((random.next() % (exponent * 1000)) / 1000));
      expect(reach(values, "linear")).toBe(oracleReach(values, "linear"));
      const logarithmic = reach(values, "logarithmic");
      expect(logarithmic).toBe(oracleReach(values, "logarithmic"));
      expect(logarithmic).toBeLessThanOrEqual(1000);
      expect(logarithmic).toBeGreaterThanOrEqual(1);
    }
  });
});

describe("misses", () => {
  it("says a value misses when it lies off the truth by more than the reach: a factor on a logarithmic scale, a distance on a linear one", () => {
    expect(misses(1, 1000, "logarithmic", 1000)).toBe(false);
    expect(misses(0.999, 1000, "logarithmic", 1000)).toBe(true);
    expect(misses(1e6, 1000, "logarithmic", 1000)).toBe(false);
    expect(misses(1.001e6, 1000, "logarithmic", 1000)).toBe(true);
    expect(misses(1000, 1000, "logarithmic", 1)).toBe(false);
    expect(misses(20, 10, "linear", 10)).toBe(false);
    expect(misses(20.5, 10, "linear", 10)).toBe(true);
    expect(misses(-0.5, 10, "linear", 10)).toBe(true);
    expect(misses(0, 10, "linear", 10)).toBe(false);
  });

  it("is symmetric on the scale and never fires with an unbounded reach", () => {
    const random = new Mt19937(99);
    for (let round = 0; round < 500; round++) {
      const [value, truth] = [10 ** ((random.next() % 12000) / 1000), 10 ** ((random.next() % 12000) / 1000)];
      const within = 1 + (random.next() % 5000) / 1000;
      for (const scale of ["linear", "logarithmic"] as const) {
        expect(misses(value, truth, scale, within)).toBe(misses(truth, value, scale, within));
        expect(misses(value, truth, scale, Infinity)).toBe(false);
        expect(misses(value, truth, scale, within)).toBe(oracleMisses(value, truth, scale, within));
      }
    }
  });

  it("is the factor 1000 of the owner's rule where the values span at least six decades", () => {
    const within = reach([1, 60, 2000, 1e5, 3e6, 1.4e9], "logarithmic");
    expect(within).toBe(1000);
    expect(misses(60 * 999, 60, "logarithmic", within)).toBe(false);
    expect(misses(60 / 999, 60, "logarithmic", within)).toBe(false);
    expect(misses(60 * 1001, 60, "logarithmic", within)).toBe(true);
    expect(misses(60 / 1001, 60, "logarithmic", within)).toBe(true);
  });

  it("never misses at exactly the factor 1000 or exactly at the square root of the spread, whatever the digits", () => {
    for (let truth = 1; truth <= 2000; truth++) {
      expect(misses(truth * 1000, truth, "logarithmic", REACH_FACTOR), `${truth} × 1000`).toBe(false);
      expect(misses(truth, truth * 1000, "logarithmic", REACH_FACTOR), `${truth * 1000} / 1000`).toBe(false);
      expect(misses(truth / 1000, truth, "logarithmic", REACH_FACTOR), `${truth} / 1000 typed in decimal`).toBe(false);
      expect(misses(truth * 1000 * (1 + 2e-9), truth, "logarithmic", REACH_FACTOR), `beyond ${truth} × 1000 and its slack`).toBe(true);
    }
    expect(misses(18000, 18, "logarithmic", 1000)).toBe(false);
    expect(misses(18000.000000000004, 18, "logarithmic", 1000)).toBe(false);
    expect(misses(18000 * (1 + 2e-9), 18, "logarithmic", 1000)).toBe(true);
    const within = reach([18, 72], "logarithmic");
    expect(within).toBe(2);
    expect(misses(36, 18, "logarithmic", within)).toBe(false);
    expect(misses(36.00000000000001, 18, "logarithmic", within)).toBe(false);
    expect(misses(36 * (1 + 2e-9), 18, "logarithmic", within)).toBe(true);
  });

  it("counts a decimal typed at exactly the reach as within: 0.018 lies a hair below 18 / 1000 in binary and the slack covers it", () => {
    expect(18 / 0.018).toBe(1000.0000000000001);
    expect(misses(0.018, 18, "logarithmic", 1000)).toBe(false);
    expect(misses(0.0179, 18, "logarithmic", 1000)).toBe(true);
  });

  it("widens the reach by exactly the slack 1e-9 and not one double more", () => {
    expect(REACH_SLACK).toBe(1e-9);
    const next = (value: number): number => {
      const view = new DataView(new ArrayBuffer(8));
      view.setFloat64(0, value);
      view.setBigUint64(0, view.getBigUint64(0) + 1n);
      return view.getFloat64(0);
    };
    for (const within of [REACH_FACTOR, 2, reach([3, 7], "logarithmic")]) {
      const bound = within * (1 + REACH_SLACK);
      expect(misses(bound, 1, "logarithmic", within) || misses(1, bound, "logarithmic", within), String(bound)).toBe(false);
      expect(misses(next(bound), 1, "logarithmic", within) && misses(1, next(bound), "logarithmic", within), String(next(bound))).toBe(true);
    }
    const bound = 55 * (1 + REACH_SLACK);
    expect(misses(bound, 0, "linear", 55) || misses(0, bound, "linear", 55)).toBe(false);
    expect(misses(next(bound), 0, "linear", 55)).toBe(true);
  });
});

describe("taskSeconds", () => {
  it("gives a base of 30 seconds plus a share per presented item, per item and dimension for a matching", () => {
    expect(TASK_SECONDS).toEqual({ base: 30, classification: 8, sorting: 12, matching: 12 });
    expect(Object.keys(TASK_SECONDS).slice(1)).toEqual([...TASK_KINDS]);
    expect(taskSeconds("classification", 6, 1)).toBe(78);
    expect(taskSeconds("sorting", 5, 1)).toBe(90);
    expect(taskSeconds("matching", 4, 1)).toBe(78);
    expect(taskSeconds("matching", 4, 2)).toBe(126);
    expect(taskSeconds("matching", 4, 3)).toBe(174);
  });

  it("ignores the dimensions of every other kind and never falls below the base", () => {
    for (const kind of ["classification", "sorting"] as const) for (const dimensions of [0, 1, 2, 7]) expect(taskSeconds(kind, 5, dimensions)).toBe(taskSeconds(kind, 5, 1));
    for (const kind of TASK_KINDS) {
      expect(taskSeconds(kind, 0, 1)).toBe(30);
      for (let items = 0; items < 30; items++) expect(taskSeconds(kind, items + 1, 2)).toBeGreaterThan(taskSeconds(kind, items, 2));
    }
  });
});

describe("acted", () => {
  it("raises the device's instant to the floor and keeps it otherwise", () => {
    expect(acted(1_500, 1_000, 1_000)).toBe(1_500);
    expect(acted(1_000, 1_000, 1_000)).toBe(1_000);
    expect(acted(999, 1_000, 1_000)).toBe(1_000);
    expect(acted(0, 1_000, 1_000)).toBe(1_000);
    expect(acted(Number.MAX_SAFE_INTEGER, 1_000, Number.MAX_SAFE_INTEGER)).toBe(Number.MAX_SAFE_INTEGER);
    expect(acted(7, 7, 0)).toBe(7);
  });

  it("lowers a claim beyond five minutes past the decider's clock to that lead, and the floor still wins", () => {
    const now = 1_790_000_000_000;
    expect(CLOCK_LEAD).toBe(5 * 60 * 1000);
    expect(acted(now + 240_000, 0, now)).toBe(now + 240_000);
    expect(acted(now + CLOCK_LEAD, 0, now)).toBe(now + CLOCK_LEAD);
    expect(acted(now + CLOCK_LEAD + 1, 0, now)).toBe(now + CLOCK_LEAD);
    expect(acted(now + 3_600_000, 0, now)).toBe(now + CLOCK_LEAD);
    expect(acted(Number.MAX_SAFE_INTEGER, now, now)).toBe(now + CLOCK_LEAD);
    expect(acted(now + 3_600_000, now + 2 * CLOCK_LEAD, now)).toBe(now + 2 * CLOCK_LEAD);
  });

  it("is max(min(at, now + lead), floor): never below the floor, never above the lead unless the floor is, the instant itself in between, and idempotent", () => {
    const random = new Mt19937(4);
    for (let round = 0; round < 1000; round++) {
      const floor = random.next() % 100_000;
      const now = random.next() % 200_000;
      const at = random.next() % 1_000_000;
      const counted = acted(at, floor, now);
      expect(counted).toBe(mathMax(mathMin(at, mathAdd(now, CLOCK_LEAD) as number), floor));
      expect(counted).toBeGreaterThanOrEqual(floor);
      if (floor <= now + CLOCK_LEAD) expect(counted).toBeLessThanOrEqual(now + CLOCK_LEAD);
      if (at >= floor && at <= now + CLOCK_LEAD) expect(counted).toBe(at);
      expect(acted(counted, floor, now)).toBe(counted);
    }
  });
});
