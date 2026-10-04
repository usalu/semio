import { abs as mathAbs, add as mathAdd, divide as mathDivide, max as mathMax, min as mathMin, multiply as mathMultiply, sign as mathSign, sqrt as mathSqrt, subtract as mathSubtract } from "mathjs";
import { describe, expect, it } from "vitest";
import {
  CHALLENGES,
  CHALLENGE_RULES,
  CLOCK_LEAD,
  HINTS_PER_TASK,
  Mt19937,
  REACH_FACTOR,
  REACH_SLACK,
  TASK_KINDS,
  TASK_SECONDS,
  acted,
  challengeMeets,
  challengeRank,
  challengeRules,
  hintsOf,
  misses,
  points,
  reach,
  sheetOf,
  shuffle,
  taskSeconds,
  verdictOf,
  type Answer,
  type Challenge,
  type ClassificationTask,
  type Hint,
  type MatchingTask,
  type Quiz,
  type Scale,
  type SheetClassificationTask,
  type SheetMatchingTask,
  type SheetSortingTask,
  type SheetTask,
  type SortingTask,
  type Task,
  type Text,
  type Verdict,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

const T = (en: string): Text => ({ en, de: en });

/** 🃏️ The sheet task of a single-task quiz at a challenge. */
function sheetTaskOf(task: Task, challenge: Challenge, seed = 1): SheetTask {
  const quiz: Quiz = { schema: "semio.quiz/v1", id: "quiz", emoji: "❓", title: T("Quiz"), description: T("Quiz"), tasks: [task] };
  return sheetOf(quiz, seed, challenge).tasks[0]!;
}

/** 📶️ A sorting task over the given values, item `i<index>` carrying `values[index]`, familiar where its index is in `familiar`. */
function sortingTask(values: readonly number[], scale: Scale, familiar: readonly number[] = []): SortingTask {
  return { kind: "sorting", id: "sorting", title: T("Sorting"), prompt: T("Sort"), quantity: { label: T("Q"), unit: "u", scale, prefixed: false, additive: false }, items: values.map((value, index) => ({ id: `i${index}`, label: T(`i${index}`), value, ...(familiar.includes(index) ? { familiar: true } : {}) })) };
}

/** 🔗️ A matching task over items `m<index>` with the given values per dimension. */
function matchingTask(dimensions: Readonly<Record<string, { readonly scale: Scale; readonly values: readonly number[] }>>): MatchingTask {
  const count = Object.values(dimensions)[0]!.values.length;
  return {
    kind: "matching",
    id: "matching",
    title: T("Matching"),
    prompt: T("Match"),
    dimensions: Object.entries(dimensions).map(([id, dimension]) => ({ id, quantity: { label: T(id), unit: "u", scale: dimension.scale, prefixed: false, additive: false } })),
    items: Array.from({ length: count }, (_, index) => ({ id: `m${index}`, label: T(`m${index}`), values: Object.fromEntries(Object.entries(dimensions).map(([id, dimension]) => [id, dimension.values[index]!])) })),
  };
}

const CLASSIFICATION: ClassificationTask = {
  kind: "classification",
  id: "classification",
  title: T("Classification"),
  prompt: T("Classify"),
  categories: [
    { id: "x", label: T("x") },
    { id: "y", label: T("y") },
  ],
  items: [
    { id: "p", label: T("p"), category: "x" },
    { id: "q", label: T("q"), category: "y" },
    { id: "r", label: T("r"), category: "y" },
    { id: "s", label: T("s"), category: "x" },
  ],
};

/** 📡️ The reach of presented true values, by mathjs: a factor `min(1000, sqrt(hi / lo))` on a logarithmic scale, a distance `(hi − lo) / 2` on a linear one. */
function oracleReach(values: readonly number[], scale: Scale): number {
  const [lo, hi] = values.length === 0 ? [Infinity, -Infinity] : [mathMin([...values]) as number, mathMax([...values]) as number];
  if (scale === "linear") return hi > lo ? (mathDivide(mathSubtract(hi, lo), 2) as number) : Infinity;
  return hi > lo ? (mathMin(1000, mathSqrt(mathDivide(hi, lo) as number) as number) as number) : 1000;
}

/** 🎯️ Whether a value misses its truth, by mathjs: the ratio of the larger to the smaller beyond a logarithmic reach, the distance beyond a linear one, either reach widened by the slack 1e-9. */
function oracleMisses(value: number, truth: number, scale: Scale, within: number): boolean {
  return (scale === "linear" ? (mathAbs(mathSubtract(value, truth)) as number) : (mathDivide(mathMax(value, truth), mathMin(value, truth)) as number)) > (mathMultiply(within, mathAdd(1, 1e-9)) as number);
}

/** 🔑️ An item of a numeric task as the oracle sees it: its assigned key, its true value, whether the key misses and whether it is familiar. */
type OracleKeyed = { readonly item: string; readonly key: number; readonly value: number; readonly miss: boolean; readonly familiar: boolean };

/** 🧷️ A reference the oracle may name for a compare hint, with its claimed and true relation, its oriented claim and its place in sheet order. */
type OracleRelated = { readonly other: string; readonly familiar: boolean; readonly claim: number; readonly truth: number; readonly oriented: number; readonly index: number };

/** 🏋️ A hint the oracle made, or a compare hint with the references tied for its largest error, with the weight it competes for the cap with (none for group and category hints). */
type OracleWeighted = { readonly hint: Hint; readonly weight?: number } | { readonly item: string; readonly dimension?: string; readonly linear: boolean; readonly tied: readonly OracleRelated[]; readonly weight: number };

/** 🪞️ The verdict by signs, by mathjs: reversed when the claim sits off the pivot and the truth does not sit on the claim's side of it, else under when the truth lies beyond the claim away from the pivot (above it for a claim at the pivot), else over. */
function oracleVerdict(claim: number, truth: number, pivot: number): Verdict {
  const side = mathSign(mathSubtract(claim, pivot)) as number;
  if (side !== 0 && (mathSign(mathSubtract(truth, pivot)) as number) !== side) return "reversed";
  return (mathSign(mathSubtract(truth, claim)) as number) === (side === 0 ? 1 : side) ? "under" : "over";
}

/** 🤨️ The compare hint of a missed item by mathjs, before its reference is chosen, weighted by the largest error: the pool is the other keyed items that do not miss, or all of them when every one misses; each is rated by the error between the claimed and the true relation (ratio of ratios, distance of differences); errors within the slack 1e-9 of the largest tie. */
function oracleCompare(hinted: OracleKeyed, keyed: readonly OracleKeyed[], scale: Scale, dimension?: string): OracleWeighted[] {
  const others = keyed.filter((candidate) => candidate.item !== hinted.item);
  const pool = others.some((candidate) => !candidate.miss) ? others.filter((candidate) => !candidate.miss) : others;
  const linear = scale === "linear";
  const rated = pool.map((other, index) => {
    const claim = (linear ? mathSubtract(hinted.key, other.key) : mathDivide(hinted.key, other.key)) as number;
    const truth = (linear ? mathSubtract(hinted.value, other.value) : mathDivide(hinted.value, other.value)) as number;
    const error = (linear ? mathAbs(mathSubtract(claim, truth)) : mathDivide(mathMax(claim, truth), mathMin(claim, truth))) as number;
    const oriented = (linear ? mathAbs(claim) : mathMax(claim, mathDivide(1, claim))) as number;
    return { other: other.item, familiar: other.familiar, claim, truth, error, oriented, index };
  });
  const largest = Math.max(...rated.map((candidate) => candidate.error));
  const tied = rated.filter((candidate) => (mathMultiply(candidate.error, mathAdd(1, 1e-9)) as number) >= largest);
  return tied.length === 0 ? [] : [{ item: hinted.item, ...(dimension === undefined ? {} : { dimension }), linear, tied, weight: largest }];
}

/** ✂️ The oracle's cap: rank the weighted hints by weight (largest first, earlier on ties), then the unweighted in order, keep three and restore their order; then choose each kept compare hint's reference in that order, by sorting its tied references: one no earlier kept hint of the dimension names first, then a familiar one, then the smallest oriented claim, then the earlier in sheet order. */
function oracleCapped(weighted: readonly OracleWeighted[]): Hint[] {
  const indexed = weighted.map((entry, order) => ({ entry, order }));
  const ranked = [...indexed.filter(({ entry }) => entry.weight !== undefined).sort((left, right) => right.entry.weight! - left.entry.weight! || left.order - right.order), ...indexed.filter(({ entry }) => entry.weight === undefined)];
  const named: string[] = [];
  return ranked
    .slice(0, 3)
    .sort((left, right) => left.order - right.order)
    .map(({ entry }) => {
      if ("hint" in entry) return entry.hint;
      const used = (candidate: OracleRelated) => Number(named.includes(`${entry.dimension}/${candidate.other}`));
      const best = [...entry.tied].sort((left, right) => used(left) - used(right) || Number(right.familiar) - Number(left.familiar) || left.oriented - right.oriented || left.index - right.index)[0]!;
      named.push(`${entry.dimension}/${best.other}`);
      return { kind: "compare", item: entry.item, other: best.other, ...(entry.dimension === undefined ? {} : { dimension: entry.dimension }), ...(entry.linear ? { difference: best.claim } : { factor: best.claim }), verdict: oracleVerdict(best.claim, best.truth, entry.linear ? 0 : 1) };
    });
}

/** 🪜️ A sorting sheet task presenting the items of `task` named by `ids` in that order, with the ascending ladder of their values. */
function sortingSheet(task: SortingTask, ids: readonly string[]): SheetSortingTask {
  const items = ids.map((id) => task.items.find((item) => item.id === id)!);
  return { kind: "sorting", id: task.id, title: task.title, prompt: task.prompt, quantity: task.quantity, keys: items.map((item) => item.value).sort((left, right) => left - right), items: items.map(({ id, label }) => ({ id, label })) };
}

/** 🀄️ A matching sheet task presenting the items of `task` named by `ids` in that order, each dimension with the given cards. */
function matchingSheet(task: MatchingTask, ids: readonly string[], cards: Readonly<Record<string, readonly number[]>>): SheetMatchingTask {
  return { kind: "matching", id: task.id, title: task.title, prompt: task.prompt, dimensions: task.dimensions.map((dimension) => ({ ...dimension, cards: cards[dimension.id]! })), items: ids.map((id) => ({ id, label: T(id) })) };
}

/** 🗄️ A classification sheet task presenting the categories named by `categories` and the items named by `items` of `task`, in those orders. */
function classificationSheet(task: ClassificationTask, items: readonly string[], categories: readonly string[] = task.categories.map((category) => category.id)): SheetClassificationTask {
  return { kind: "classification", id: task.id, title: task.title, prompt: task.prompt, ...(task.axes ? { axes: task.axes } : {}), categories: categories.map((id) => task.categories.find((category) => category.id === id)!), items: items.map((id) => ({ id, label: T(id) })) };
}

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

describe("hintsOf — sorting", () => {
  const order = (ids: readonly string[]): Answer => ({ kind: "sorting", order: [...ids] });

  it("questions, in the learner's order, every item whose ladder key misses its value against another item, never naming a direction or the truth", () => {
    const task = sortingTask([1, 10, 100, 1e8], "logarithmic");
    const sheetTask = sortingSheet(task, ["i2", "i0", "i3", "i1"]);
    expect(sheetTask.keys).toEqual([1, 10, 100, 1e8]);
    expect(hintsOf(task, sheetTask, order(["i0", "i1", "i2", "i3"]))).toEqual([]);
    expect(hintsOf(task, sheetTask, order(["i1", "i0", "i2", "i3"]))).toEqual([]);
    expect(hintsOf(task, sheetTask, order(["i3", "i2", "i1", "i0"]))).toEqual([
      { kind: "compare", item: "i3", other: "i1", factor: 1 / 100, verdict: "reversed" },
      { kind: "compare", item: "i0", other: "i2", factor: 1e8 / 10, verdict: "reversed" },
    ]);
    const [hint] = hintsOf(task, sheetTask, order(["i3", "i2", "i1", "i0"]));
    expect(Object.keys(hint!)).toEqual(["kind", "item", "other", "factor", "verdict"]);
  });

  it("prefers an anchor, an item whose own key does not miss, and takes the one whose claimed relation is the most wrong", () => {
    const task = sortingTask([1, 3, 1e7, 1e8], "logarithmic");
    const sheetTask = sortingSheet(task, ["i0", "i1", "i2", "i3"]);
    expect(hintsOf(task, sheetTask, order(["i1", "i2", "i0", "i3"]))).toEqual([
      { kind: "compare", item: "i2", other: "i3", factor: 3 / 1e8, verdict: "over" },
      { kind: "compare", item: "i0", other: "i1", factor: 1e7 / 1, verdict: "reversed" },
    ]);
  });

  it("says `under` when the truth lies further from 1 than the claim on the same side and `over` when it lies nearer, both when the claim is above and below 1", () => {
    const above = sortingTask([1, 10, 1e8], "logarithmic");
    expect(hintsOf(above, sortingSheet(above, ["i2", "i1", "i0"]), order(["i0", "i2", "i1"]))).toEqual([
      { kind: "compare", item: "i2", other: "i0", factor: 10, verdict: "under" },
      { kind: "compare", item: "i1", other: "i0", factor: 1e8, verdict: "over" },
    ]);
    const below = sortingTask([1, 1e7, 1e8], "logarithmic");
    expect(hintsOf(below, sortingSheet(below, ["i0", "i1", "i2"]), order(["i1", "i0", "i2"]))).toEqual([
      { kind: "compare", item: "i1", other: "i2", factor: 1 / 1e8, verdict: "over" },
      { kind: "compare", item: "i0", other: "i2", factor: 1e7 / 1e8, verdict: "under" },
    ]);
  });

  it("says `reversed` when the learner's keys put the pair the wrong way round, in both directions and when the truth sits at the pivot", () => {
    const pair = sortingTask([1, 1e8], "logarithmic");
    expect(hintsOf(pair, sortingSheet(pair, ["i0", "i1"]), order(["i1", "i0"])).map((hint) => hint.kind === "compare" && [hint.factor, hint.verdict])).toEqual([
      [1 / 1e8, "reversed"],
      [1e8, "reversed"],
    ]);
    expect(verdictOf(1e-8, 1e8, 1)).toBe("reversed");
    expect(verdictOf(1e8, 1e-8, 1)).toBe("reversed");
    expect(verdictOf(5, 1, 1)).toBe("reversed");
    expect(verdictOf(0.2, 1, 1)).toBe("reversed");
    expect(verdictOf(5, 0, 0)).toBe("reversed");
    expect(verdictOf(-5, 0, 0)).toBe("reversed");
    expect(verdictOf(-5, 3, 0)).toBe("reversed");
    expect(verdictOf(1, 1e8, 1)).toBe("under");
    expect(verdictOf(1, 1e-8, 1)).toBe("over");
    expect(verdictOf(1, 1, 1)).toBe("over");
    expect(verdictOf(0, 7, 0)).toBe("under");
    expect(verdictOf(0, -7, 0)).toBe("over");
    expect(verdictOf(4, 4, 1)).toBe("over");
    expect(verdictOf(-4, -4, 0)).toBe("over");
    expect(verdictOf(1 + Number.EPSILON, 1 - Number.EPSILON / 2, 1)).toBe("reversed");
    expect(verdictOf(1 + Number.EPSILON, 1 + 2 * Number.EPSILON, 1)).toBe("under");
  });

  it("agrees with the verdict by signs over mathjs on every sign and order of claim and truth", () => {
    const points = [-1e9, -3, -1, -0.5, 0, 1e-300, 0.5, 1 - Number.EPSILON / 2, 1, 1 + Number.EPSILON, 2, 3, 1e9];
    for (const pivot of [0, 1]) for (const claim of points) for (const truth of points) expect([claim, truth, pivot, verdictOf(claim, truth, pivot)]).toEqual([claim, truth, pivot, oracleVerdict(claim, truth, pivot)]);
  });

  it("takes the smallest claim among relations wrong within the slack, then the first in sheet order", () => {
    const task = sortingTask([1, 10, 1e4, 1e8], "logarithmic");
    const answer = order(["i3", "i1", "i2", "i0"]);
    for (const sheet of [["i0", "i1", "i2", "i3"], ["i3", "i2", "i1", "i0"]]) {
      expect(hintsOf(task, sortingSheet(task, sheet), answer)).toEqual([
        { kind: "compare", item: "i3", other: "i1", factor: 1 / 10, verdict: "reversed" },
        { kind: "compare", item: "i0", other: "i2", factor: 1e8 / 1e4, verdict: "reversed" },
      ]);
    }
    const twins = sortingTask([1, 8, 8, 8192], "logarithmic");
    const swapped = order(["i3", "i1", "i2", "i0"]);
    expect(hintsOf(twins, sortingSheet(twins, ["i2", "i0", "i1", "i3"]), swapped).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["i2", "i1"]);
    expect(hintsOf(twins, sortingSheet(twins, ["i1", "i0", "i2", "i3"]), swapped).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["i1", "i2"]);
    const tanks = (apart: number) => sortingTask([0, 1000, 3000, 3000 + apart, 4000], "linear");
    const levels = order(["i4", "i1", "i3", "i2", "i0"]);
    expect(hintsOf(tanks(1e-6), sortingSheet(tanks(1e-6), ["i2", "i4", "i1", "i0", "i3"]), levels).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["i1", "i2"]);
    expect(hintsOf(tanks(1e-5), sortingSheet(tanks(1e-5), ["i2", "i4", "i1", "i0", "i3"]), levels).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["i2", "i3"]);
  });

  it("prefers a familiar item inside the tie window, then the smallest claim, then the first in sheet order, and never beyond the window or the anchors", () => {
    const answer = order(["i3", "i1", "i2", "i0"]);
    const sheet = ["i0", "i1", "i2", "i3"];
    const others = (familiar: readonly number[]) => {
      const task = sortingTask([1, 10, 1e4, 1e8], "logarithmic", familiar);
      return hintsOf(task, sortingSheet(task, sheet), answer).map((hint) => hint.kind === "compare" && hint.other);
    };
    expect(others([])).toEqual(["i1", "i2"]);
    expect(others([1])).toEqual(["i1", "i2"]);
    expect(others([2])).toEqual(["i2", "i1"]);
    expect(others([1, 2])).toEqual(["i1", "i2"]);
    expect(others([0, 3])).toEqual(["i1", "i2"]);
    const familiar = sortingTask([1, 10, 1e4, 1e8], "logarithmic", [2]);
    expect(hintsOf(familiar, sortingSheet(familiar, sheet), answer)).toEqual([
      { kind: "compare", item: "i3", other: "i2", factor: 1 / 1e4, verdict: "reversed" },
      { kind: "compare", item: "i0", other: "i1", factor: 1e8 / 10, verdict: "reversed" },
    ]);
    const twins = sortingTask([1, 8, 8, 8192], "logarithmic", [1, 2]);
    expect(hintsOf(twins, sortingSheet(twins, ["i2", "i0", "i1", "i3"]), answer).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["i2", "i1"]);
    const outside = sortingTask([1, 3, 1e7, 1e8], "logarithmic", [1]);
    expect(hintsOf(outside, sortingSheet(outside, sheet), order(["i1", "i2", "i0", "i3"])).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["i3", "i1"]);
  });

  it("prefers inside the tie window a reference no earlier hint of the task names, before a familiar one and a smaller claim, names one again only when the window holds no other, and lets no hint the cap drops use one up", () => {
    const pairs = (task: SortingTask, sheet: readonly string[], ids: readonly string[]) => hintsOf(task, sortingSheet(task, sheet), order(ids)).map((hint) => hint.kind === "compare" && [hint.item, hint.other]);
    const appliances = [9, 40, 60, 2200, 11000, 1.6e9];
    expect(pairs(sortingTask(appliances, "logarithmic"), ["i5", "i0", "i3", "i2", "i4", "i1"], ["i0", "i1", "i2", "i3", "i5", "i4"])).toEqual([
      ["i5", "i3"],
      ["i4", "i2"],
    ]);
    const twins = [9, 40, 2200, 2200, 11000, 1.6e9];
    const sheet = ["i5", "i0", "i2", "i3", "i4", "i1"];
    expect(pairs(sortingTask(twins, "logarithmic"), sheet, ["i0", "i1", "i2", "i3", "i5", "i4"])).toEqual([
      ["i5", "i2"],
      ["i4", "i3"],
    ]);
    expect(pairs(sortingTask(twins, "logarithmic", [3, 4]), sheet, ["i0", "i1", "i2", "i3", "i5", "i4"])).toEqual([
      ["i5", "i3"],
      ["i4", "i2"],
    ]);
    expect(pairs(sortingTask(twins, "logarithmic"), sheet, ["i4", "i5", "i0", "i2", "i1", "i3"])).toEqual([
      ["i4", "i1"],
      ["i5", "i1"],
      ["i3", "i2"],
    ]);
    const eight = sortingTask([9, 40, 60, 2200, 2200, 11000, 3e6, 1.6e9], "logarithmic");
    expect(hintsOf(eight, sortingSheet(eight, ["i7", "i0", "i3", "i4", "i5", "i1", "i2", "i6"]), order(["i0", "i1", "i2", "i3", "i6", "i7", "i4", "i5"]))).toEqual([
      { kind: "compare", item: "i6", other: "i3", factor: 1, verdict: "under" },
      { kind: "compare", item: "i7", other: "i2", factor: 11000 / 60, verdict: "under" },
      { kind: "compare", item: "i5", other: "i1", factor: 1.6e9 / 40, verdict: "over" },
    ]);
  });

  it("falls back to every other keyed item when no key holds, and gives no hint when no other item holds a key", () => {
    const task = sortingTask([1, 1e8], "logarithmic");
    const sheetTask = sortingSheet(task, ["i0", "i1"]);
    expect(hintsOf(task, sheetTask, order(["i1", "i0"]))).toEqual([
      { kind: "compare", item: "i1", other: "i0", factor: 1 / 1e8, verdict: "reversed" },
      { kind: "compare", item: "i0", other: "i1", factor: 1e8, verdict: "reversed" },
    ]);
    expect(hintsOf(task, sheetTask, order(["i1"]))).toEqual([]);
  });

  it("questions a difference on a linear scale, `under` when the truth lies further from 0 on the same side, `over` nearer, `reversed` across", () => {
    const above = sortingTask([0, 1, 100], "linear");
    expect(hintsOf(above, sortingSheet(above, ["i0", "i1", "i2"]), order(["i1", "i2", "i0"]))).toEqual([
      { kind: "compare", item: "i2", other: "i1", difference: 1, verdict: "under" },
      { kind: "compare", item: "i0", other: "i1", difference: 100, verdict: "reversed" },
    ]);
    const below = sortingTask([0, 99, 100], "linear");
    const hints = hintsOf(below, sortingSheet(below, ["i0", "i1", "i2"]), order(["i1", "i0", "i2"]));
    expect(hints).toEqual([
      { kind: "compare", item: "i1", other: "i2", difference: -100, verdict: "over" },
      { kind: "compare", item: "i0", other: "i2", difference: -1, verdict: "under" },
    ]);
    expect(Object.keys(hints[0]!)).toEqual(["kind", "item", "other", "difference", "verdict"]);
    const tie = sortingTask([0, 10, 20, 30], "linear");
    expect(hintsOf(tie, sortingSheet(tie, ["i2", "i1", "i0", "i3"]), order(["i3", "i1", "i2", "i0"]))).toEqual([
      { kind: "compare", item: "i3", other: "i1", difference: -10, verdict: "reversed" },
      { kind: "compare", item: "i0", other: "i2", difference: 10, verdict: "reversed" },
    ]);
  });

  it("gives at most three hints: the largest errors, the earlier among equal ones, in the learner's order", () => {
    expect(HINTS_PER_TASK).toBe(3);
    const task = sortingTask([1, 1e2, 1e4, 1e6, 1e8, 1e10], "logarithmic");
    const sheetTask = sortingSheet(task, ["i0", "i1", "i2", "i3", "i4", "i5"]);
    const reversed = order(["i5", "i4", "i3", "i2", "i1", "i0"]);
    expect(hintsOf(task, sheetTask, reversed)).toEqual([
      { kind: "compare", item: "i5", other: "i2", factor: 1 / 1e6, verdict: "reversed" },
      { kind: "compare", item: "i4", other: "i2", factor: 1e2 / 1e6, verdict: "reversed" },
      { kind: "compare", item: "i0", other: "i3", factor: 1e10 / 1e4, verdict: "reversed" },
    ]);
    expect(hintsOf(task, sheetTask, order(["i0", "i4", "i3", "i2", "i1", "i5"])).map((hint) => hint.item)).toEqual(["i4", "i1"]);
  });

  it("measures the miss with the reach of the ladder: the square root of its max/min ratio where that is less than 1000, and half its spread on a linear scale", () => {
    const narrow = sortingTask([10, 100, 1000], "logarithmic");
    const sheetTask = sortingSheet(narrow, ["i0", "i1", "i2"]);
    expect(hintsOf(narrow, sheetTask, order(["i1", "i0", "i2"]))).toEqual([]);
    expect(hintsOf(narrow, sheetTask, order(["i2", "i1", "i0"])).map((hint) => hint.item)).toEqual(["i2", "i0"]);
    const linear = sortingTask([0, 10, 20, 31], "linear");
    const presented = sortingSheet(linear, ["i0", "i1", "i2", "i3"]);
    expect(hintsOf(linear, presented, order(["i1", "i0", "i3", "i2"]))).toEqual([]);
    expect(hintsOf(linear, presented, order(["i3", "i1", "i2", "i0"])).map((hint) => hint.item)).toEqual(["i3", "i0"]);
  });

  it("never hints values that do not spread on a linear scale", () => {
    const flat = sortingTask([5, 5, 5], "linear");
    expect(hintsOf(flat, sheetTaskOf(flat, "easy"), order(["i2", "i0", "i1"]))).toEqual([]);
  });

  it("is a function of the task, the sheet task and the answer: no hint where the keys are hidden, without an answer or for an answer of another kind", () => {
    const task = sortingTask([1, 1e3, 1e6, 1e9], "logarithmic");
    const wrong = order(["i3", "i2", "i1", "i0"]);
    expect(hintsOf(task, sheetTaskOf(task, "easy"), wrong).length).toBe(2);
    expect(hintsOf(task, sheetTaskOf(task, "medium"), wrong)).toEqual(hintsOf(task, sheetTaskOf(task, "easy"), wrong));
    for (const challenge of ["hard", "expert"] as const) expect(hintsOf(task, sheetTaskOf(task, challenge), wrong)).toEqual([]);
    expect(hintsOf(task, sheetTaskOf(task, "easy"))).toEqual([]);
    expect(hintsOf(task, sheetTaskOf(task, "easy"), { kind: "classification", assignments: {} })).toEqual([]);
    expect(hintsOf(CLASSIFICATION, sheetTaskOf(task, "easy"), wrong)).toEqual([]);
    const sheetTask = sortingSheet(task, ["i0", "i1", "i2", "i3"]);
    expect(hintsOf(task, sheetTask, order(["i3", "nobody", "i1", "i0", "i2"]))).toEqual([
      { kind: "compare", item: "i3", other: "i1", factor: 1 / 1e6, verdict: "reversed" },
      { kind: "compare", item: "i0", other: "i1", factor: 1e9 / 1e6, verdict: "reversed" },
    ]);
  });

  it("gives exactly the hints of the same rule over mathjs, for random ladders, sheets, familiar items and orders", () => {
    const random = new Mt19937(606);
    let hinted = 0;
    let capped = 0;
    const verdicts = new Set<string>();
    for (const scale of ["linear", "logarithmic"] as const) {
      for (let round = 0; round < 300; round++) {
        const values = Array.from({ length: 2 + (round % 7) }, () => 10 ** ((random.next() % 12000) / 1000));
        const familiar = values.flatMap((_, index) => (random.next() % 3 === 0 ? [index] : []));
        const drawn = sortingTask(values, scale, familiar);
        const sheetTask = sheetTaskOf(drawn, "easy", round);
        if (sheetTask.kind !== "sorting") throw new Error("kind");
        const ids = shuffle(random, sheetTask.items.map((item) => item.id));
        const keys = [...values].sort((left, right) => left - right);
        expect(sheetTask.keys).toEqual(keys);
        const within = oracleReach(values, scale);
        const keyed = sheetTask.items.map((item): OracleKeyed => {
          const key = keys[ids.indexOf(item.id)]!;
          const value = values[Number(item.id.slice(1))]!;
          return { item: item.id, key, value, miss: oracleMisses(key, value, scale, within), familiar: familiar.includes(Number(item.id.slice(1))) };
        });
        const weighted = ids.flatMap((id) => {
          const entry = keyed.find((candidate) => candidate.item === id)!;
          return entry.miss ? oracleCompare(entry, keyed, scale) : [];
        });
        const expected = oracleCapped(weighted);
        hinted += expected.length;
        if (weighted.length > HINTS_PER_TASK) capped++;
        for (const hint of expected) if (hint.kind === "compare") verdicts.add(hint.verdict);
        expect(hintsOf(drawn, sheetTask, order(ids))).toEqual(expected);
      }
    }
    expect(hinted).toBeGreaterThan(100);
    expect(capped).toBeGreaterThan(10);
    expect([...verdicts].sort()).toEqual(["over", "reversed", "under"]);
  });
});

describe("hintsOf — matching", () => {
  const task = matchingTask({ power: { scale: "logarithmic", values: [1, 1e3, 1e6, 1e9] }, length: { scale: "linear", values: [0, 10, 20, 31] } });

  /** 🎴️ A matching answer assigning every named item the card that holds `pick(dimension, item)`. */
  function assigning(sheetTask: SheetTask, pick: (dimension: string, item: string) => number | undefined): Answer {
    if (sheetTask.kind !== "matching") throw new Error("kind");
    return {
      kind: "matching",
      assignments: Object.fromEntries(
        sheetTask.dimensions.map((dimension) => [
          dimension.id,
          Object.fromEntries(
            sheetTask.items.flatMap((item) => {
              const value = pick(dimension.id, item.id);
              return value === undefined ? [] : [[item.id, dimension.cards!.indexOf(value)] as const];
            }),
          ),
        ]),
      ),
    };
  }
  const truth = (dimension: string, item: string) => task.items.find((candidate) => candidate.id === item)!.values[dimension]!;

  /** 🔮️ The oracle's hints of a matching answer: per dimension in sheet order, the items in sheet order that hold a valid card, each missed one questioned by {@link oracleCompare}, capped over the whole task by {@link oracleCapped}. */
  function oracleMatching(subject: MatchingTask, sheetTask: SheetMatchingTask, answer: Answer): Hint[] {
    if (answer.kind !== "matching" || !answer.assignments) return [];
    const assignments = answer.assignments;
    return oracleCapped(sheetTask.dimensions.flatMap((dimension) => {
      const valued = sheetTask.items.flatMap((presented) => subject.items.filter((item) => item.id === presented.id && dimension.id in item.values));
      const within = oracleReach(
        valued.map((item) => item.values[dimension.id]!),
        dimension.quantity.scale,
      );
      const keyed = valued.flatMap((item): OracleKeyed[] => {
        const card = assignments[dimension.id]?.[item.id];
        const key = card === undefined ? undefined : dimension.cards![card];
        return key === undefined ? [] : [{ item: item.id, key, value: item.values[dimension.id]!, miss: oracleMisses(key, item.values[dimension.id]!, dimension.quantity.scale, within), familiar: item.familiar === true }];
      });
      return keyed.flatMap((entry) => (entry.miss ? oracleCompare(entry, keyed, dimension.quantity.scale, dimension.id) : []));
    }));
  }

  it("questions, per dimension in sheet order and per item in sheet order, every assigned card that misses the item's value against another item of the dimension, at most three per task by the largest error", () => {
    for (let seed = 0; seed < 20; seed++) {
      const sheetTask = sheetTaskOf(task, "easy", seed);
      if (sheetTask.kind !== "matching") throw new Error("kind");
      expect(hintsOf(task, sheetTask, assigning(sheetTask, truth))).toEqual([]);
      const mirrored = assigning(sheetTask, (dimension, item) => truth(dimension, `m${3 - Number(item.slice(1))}`));
      const hints = hintsOf(task, sheetTask, mirrored);
      const missed = sheetTask.items.filter((item) => item.id === "m0" || item.id === "m3").map((item) => item.id);
      expect(hints.map((hint) => [hint.item, hint.kind === "compare" ? hint.dimension : undefined])).toEqual([...missed.map((item) => [item, "power"]), [missed[0], "length"]]);
      expect(hints).toEqual(oracleMatching(task, sheetTask, mirrored));
    }
    const sheetTask = matchingSheet(task, ["m0", "m1", "m2", "m3"], { power: [1, 1e3, 1e6, 1e9], length: [0, 10, 20, 31] });
    const hints = hintsOf(task, sheetTask, { kind: "matching", assignments: { power: { m0: 3, m1: 1, m2: 2, m3: 0 }, length: { m0: 3, m1: 1, m2: 2, m3: 0 } } });
    expect(hints).toEqual([
      { kind: "compare", item: "m0", other: "m2", dimension: "power", factor: 1e9 / 1e6, verdict: "reversed" },
      { kind: "compare", item: "m3", other: "m1", dimension: "power", factor: 1 / 1e3, verdict: "reversed" },
      { kind: "compare", item: "m0", other: "m2", dimension: "length", difference: 31 - 20, verdict: "reversed" },
    ]);
    expect(Object.keys(hints[0]!)).toEqual(["kind", "item", "other", "dimension", "factor", "verdict"]);
    expect(Object.keys(hints[2]!)).toEqual(["kind", "item", "other", "dimension", "difference", "verdict"]);
    expect(hintsOf(task, sheetTask, { kind: "matching", assignments: { length: { m0: 3, m1: 1, m2: 2, m3: 0 } } })).toEqual([
      { kind: "compare", item: "m0", other: "m2", dimension: "length", difference: 31 - 20, verdict: "reversed" },
      { kind: "compare", item: "m3", other: "m1", dimension: "length", difference: 0 - 10, verdict: "reversed" },
    ]);
  });

  it("prefers a familiar item of the dimension inside the tie window", () => {
    const sheetTask = matchingSheet(task, ["m0", "m1", "m2", "m3"], { power: [1, 1e3, 1e6, 1e9], length: [0, 10, 20, 31] });
    const answer: Answer = { kind: "matching", assignments: { length: { m0: 3, m1: 1, m2: 2, m3: 0 } } };
    expect(hintsOf(task, sheetTask, answer).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["m2", "m1"]);
    const familiar: MatchingTask = { ...task, items: task.items.map((item) => (item.id === "m1" ? { ...item, familiar: true } : item)) };
    expect(hintsOf(familiar, sheetTask, answer).map((hint) => hint.kind === "compare" && hint.other)).toEqual(["m1", "m2"]);
    expect(hintsOf(familiar, sheetTask, answer)).toEqual(oracleMatching(familiar, sheetTask, answer));
  });

  it("reads a claim of equal keys as a factor 1 or a difference 0, never reversed, and says `under` exactly when the truth lies above it", () => {
    const pair = matchingTask({ power: { scale: "logarithmic", values: [1, 1e8] }, length: { scale: "linear", values: [0, 100] } });
    const sheetTask = matchingSheet(pair, ["m0", "m1"], { power: [1, 1e8], length: [0, 100] });
    expect(hintsOf(pair, sheetTask, { kind: "matching", assignments: { power: { m0: 0, m1: 0 }, length: { m0: 0, m1: 0 } } })).toEqual([
      { kind: "compare", item: "m1", other: "m0", dimension: "power", factor: 1, verdict: "under" },
      { kind: "compare", item: "m1", other: "m0", dimension: "length", difference: 0, verdict: "under" },
    ]);
    expect(hintsOf(pair, sheetTask, { kind: "matching", assignments: { power: { m0: 1, m1: 1 }, length: { m0: 1, m1: 1 } } })).toEqual([
      { kind: "compare", item: "m0", other: "m1", dimension: "power", factor: 1, verdict: "over" },
      { kind: "compare", item: "m0", other: "m1", dimension: "length", difference: 0, verdict: "over" },
    ]);
  });

  it("questions only the items that hold a valid card, against items that hold one, each dimension on its own scale and reach", () => {
    const sheetTask = matchingSheet(task, ["m2", "m0", "m3", "m1"], { power: [1e6, 1, 1e9, 1e3], length: [20, 0, 31, 10] });
    expect(hintsOf(task, sheetTask, { kind: "matching", assignments: { power: { m0: 2, m1: 3 }, length: { m3: 3, m1: 0, m2: 7 } } })).toEqual([
      { kind: "compare", item: "m0", other: "m1", dimension: "power", factor: 1e9 / 1e3, verdict: "reversed" },
      { kind: "compare", item: "m3", other: "m1", dimension: "length", difference: 10 - 20, verdict: "reversed" },
    ]);
    expect(hintsOf(task, sheetTask, { kind: "matching", assignments: { power: { m0: 2 } } })).toEqual([]);
    expect(hintsOf(task, sheetTask, { kind: "matching", assignments: { power: { m0: 2, m1: -1, m2: 4 } } })).toEqual([]);
    expect(hintsOf(task, sheetTask, { kind: "matching", assignments: {} })).toEqual([]);
    expect(hintsOf(task, sheetTask, { kind: "matching", assignments: { power: {} } })).toEqual([]);
  });

  it("leaves out items without a value in the dimension, from the reach and from the candidates", () => {
    const gappy: MatchingTask = { ...task, items: task.items.map((item) => (item.id === "m1" ? { ...item, values: { length: item.values.length! } } : item)) };
    const sheetTask = matchingSheet(gappy, ["m0", "m1", "m2", "m3"], { power: [1, 1e3, 1e6, 1e9], length: [0, 10, 20, 31] });
    expect(hintsOf(gappy, sheetTask, { kind: "matching", assignments: { power: { m0: 3, m1: 1, m2: 2, m3: 0 } } })).toEqual([
      { kind: "compare", item: "m0", other: "m2", dimension: "power", factor: 1e9 / 1e6, verdict: "reversed" },
      { kind: "compare", item: "m3", other: "m2", dimension: "power", factor: 1 / 1e6, verdict: "reversed" },
    ]);
  });

  it("gives exactly the hints of the same rule over mathjs, for random tasks, familiar items, cards and assignments", () => {
    const random = new Mt19937(4711);
    let hinted = 0;
    for (let round = 0; round < 300; round++) {
      const count = 2 + (round % 6);
      const draw = (): number[] => Array.from({ length: count }, () => 10 ** ((random.next() % 9000) / 1000));
      const plain = matchingTask({ power: { scale: "logarithmic", values: draw() }, length: { scale: "linear", values: draw() } });
      const subject: MatchingTask = { ...plain, items: plain.items.map((item) => (random.next() % 3 === 0 ? { ...item, familiar: true } : item)) };
      const sheetTask = sheetTaskOf(subject, "easy", round);
      if (sheetTask.kind !== "matching") throw new Error("kind");
      const answer: Answer = {
        kind: "matching",
        assignments: Object.fromEntries(sheetTask.dimensions.map((dimension) => [dimension.id, Object.fromEntries(sheetTask.items.flatMap((item) => (random.next() % 5 === 0 ? [] : [[item.id, random.next() % count] as const])))])),
      };
      const expected = oracleMatching(subject, sheetTask, answer);
      hinted += expected.length;
      expect(hintsOf(subject, sheetTask, answer)).toEqual(expected);
    }
    expect(hinted).toBeGreaterThan(100);
  });

  it("gives no hint where the keys are hidden, for guesses, or without an answer", () => {
    const easy = sheetTaskOf(task, "easy", 5);
    const mirrored = assigning(easy, (dimension, item) => truth(dimension, `m${3 - Number(item.slice(1))}`));
    expect(hintsOf(task, sheetTaskOf(task, "medium", 5), mirrored)).toEqual(hintsOf(task, easy, mirrored));
    for (const challenge of ["hard", "expert"] as const) {
      expect(hintsOf(task, sheetTaskOf(task, challenge, 5), mirrored)).toEqual([]);
      expect(hintsOf(task, sheetTaskOf(task, challenge, 5), { kind: "matching", guesses: { power: { m0: 1e9 } } })).toEqual([]);
    }
    expect(hintsOf(task, easy, { kind: "matching", guesses: { power: { m0: 1e9 } } })).toEqual([]);
    expect(hintsOf(task, easy)).toEqual([]);
    expect(hintsOf(task, easy, { kind: "matching" })).toEqual([]);
  });
});

describe("hintsOf — classification", () => {
  const assigned = (assignments: Readonly<Record<string, string>>): Answer => ({ kind: "classification", assignments });

  const plain = classificationSheet(CLASSIFICATION, ["p", "q", "r", "s"]);
  const TRIAD: ClassificationTask = {
    ...CLASSIFICATION,
    categories: [...CLASSIFICATION.categories, { id: "z", label: T("z") }],
    items: [
      { id: "p", label: T("p"), category: "x" },
      { id: "s", label: T("s"), category: "x" },
      { id: "q", label: T("q"), category: "y" },
    ],
  };
  const axis = (id: string, max: number) => ({ id, label: T(id), unit: "u", min: 0, max });
  const PROFILED: ClassificationTask = {
    kind: "classification",
    id: "profiled",
    title: T("Profiled"),
    prompt: T("Classify"),
    axes: [axis("h", 400), axis("c", 40), axis("k", 4), axis("e", 10)],
    categories: [
      { id: "old", label: T("old"), profile: { h: 300, c: 10, k: 1, e: 5 } },
      { id: "low", label: T("low"), profile: { h: 60, c: 25, k: 1.5, e: 5 } },
      { id: "passive", label: T("passive"), profile: { h: 15, c: 30, k: 3, e: 5 } },
      { id: "mid", label: T("mid"), profile: { h: 100, c: 20, k: 2, e: 5 } },
      { id: "unrated", label: T("unrated") },
    ],
    items: [
      { id: "attic", label: T("attic"), category: "old" },
      { id: "barn", label: T("barn"), category: "low" },
      { id: "cabin", label: T("cabin"), category: "passive" },
      { id: "dome", label: T("dome"), category: "mid" },
      { id: "eave", label: T("eave"), category: "unrated" },
    ],
  };
  const profiled = classificationSheet(PROFILED, ["attic", "barn", "cabin", "dome", "eave"]);

  /** 🧿️ The oracle's hints of a classification answer: per presented, assigned item standing in a known category not its own, in sheet order — the profile axis by mathjs when both categories carry profiles, measured against the anchor between the two values farthest from the own one, else the first pairing found by filtering, else the category — capped by {@link oracleCapped}. */
  function oracleClassification(subject: ClassificationTask, sheetTask: SheetClassificationTask, answer: Answer): Hint[] {
    if (answer.kind !== "classification") return [];
    const placed = sheetTask.items.flatMap((shown) => subject.items.filter((item) => item.id === shown.id && shown.id in answer.assignments).map((item) => ({ item, category: answer.assignments[item.id]! })));
    const profileOf = (id: string) => subject.categories.find((category) => category.id === id)?.profile;
    return oracleCapped(placed.flatMap(({ item, category }): OracleWeighted[] => {
      if (category === item.category || !subject.categories.some((candidate) => candidate.id === category)) return [];
      const [chosen, own] = [profileOf(category), profileOf(item.category)];
      if (chosen && own) {
        const rated = (subject.axes ?? []).flatMap((spoke, index) => {
          if (!(spoke.id in chosen) || !(spoke.id in own)) return [];
          const values = sheetTask.categories.flatMap((shown) => {
            const profile = profileOf(shown.id);
            return profile && spoke.id in profile ? [profile[spoke.id]!] : [];
          });
          const within = values.length === 0 ? 0 : (mathDivide(mathSubtract(mathMax(values), mathMin(values)), 2) as number);
          if (!(within > 0)) return [];
          const gap = mathAbs(mathSubtract(chosen[spoke.id]!, own[spoke.id]!)) as number;
          return [{ axis: spoke.id, index, beyond: gap > (mathMultiply(within, mathAdd(1, 1e-9)) as number), ratio: mathDivide(gap, within) as number }];
        });
        const best = [...rated].sort((left, right) => right.ratio - left.ratio || left.index - right.index)[0];
        if (!best || !rated.some((entry) => entry.beyond)) return [];
        const [claimed, truth] = [chosen[best.axis]!, own[best.axis]!];
        const between = placed.flatMap((anchor, index) => {
          const value = anchor.item.id === item.id || anchor.category !== anchor.item.category ? undefined : profileOf(anchor.item.category)?.[best.axis];
          if (value === undefined || (mathSign(mathSubtract(claimed, value)) as number) * (mathSign(mathSubtract(truth, value)) as number) !== -1) return [];
          return [{ other: anchor.item.id, above: claimed > value, gap: mathAbs(mathSubtract(truth, value)) as number, index }];
        });
        const anchor = [...between].sort((left, right) => right.gap - left.gap || left.index - right.index)[0];
        return [{ hint: { kind: "profile", item: item.id, category, axis: best.axis, ...(anchor ? { other: anchor.other, above: anchor.above } : {}) }, weight: best.ratio }];
      }
      const others = placed.filter((other) => other.item.id !== item.id);
      const together = others.filter((other) => other.category === category && other.item.category !== item.category)[0];
      const apart = others.filter((other) => other.item.category === item.category && other.category !== category)[0];
      if (together) return [{ hint: { kind: "group", item: item.id, other: together.item.id, together: true } }];
      if (apart) return [{ hint: { kind: "group", item: item.id, other: apart.item.id, together: false } }];
      return [{ hint: { kind: "category", item: item.id, category } }];
    }));
  }

  it("pairs a misplaced item first with an item the learner put beside it although its own category differs", () => {
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "x", q: "y", r: "y", s: "x" }))).toEqual([]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "y", q: "y", r: "y", s: "x" }))).toEqual([{ kind: "group", item: "p", other: "q", together: true }]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "y", q: "y", r: "x", s: "x" }))).toEqual([
      { kind: "group", item: "p", other: "q", together: true },
      { kind: "group", item: "r", other: "s", together: true },
    ]);
    expect(hintsOf(CLASSIFICATION, classificationSheet(CLASSIFICATION, ["s", "r", "q", "p"]), assigned({ p: "y", q: "y", r: "x", s: "x" }))).toEqual([
      { kind: "group", item: "r", other: "s", together: true },
      { kind: "group", item: "p", other: "q", together: true },
    ]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "y", q: "x", r: "y", s: "x" }))).toEqual([
      { kind: "group", item: "p", other: "r", together: true },
      { kind: "group", item: "q", other: "s", together: true },
    ]);
    expect(Object.keys(hintsOf(CLASSIFICATION, plain, assigned({ p: "y", q: "y", r: "y", s: "x" }))[0]!)).toEqual(["kind", "item", "other", "together"]);
  });

  it("else pairs it with an item of its own category the learner put elsewhere, else questions the category it was put in", () => {
    const triad = classificationSheet(TRIAD, ["p", "s", "q"]);
    expect(hintsOf(TRIAD, triad, assigned({ p: "z", s: "x", q: "y" }))).toEqual([{ kind: "group", item: "p", other: "s", together: false }]);
    expect(hintsOf(TRIAD, triad, assigned({ p: "y", s: "z", q: "y" }))).toEqual([
      { kind: "group", item: "p", other: "q", together: true },
      { kind: "group", item: "s", other: "p", together: false },
    ]);
    expect(hintsOf(TRIAD, triad, assigned({ p: "z", s: "z", q: "y" }))).toEqual([
      { kind: "category", item: "p", category: "z" },
      { kind: "category", item: "s", category: "z" },
    ]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "y", q: "x", r: "x", s: "y" }))).toEqual([
      { kind: "category", item: "p", category: "y" },
      { kind: "category", item: "q", category: "x" },
      { kind: "category", item: "r", category: "x" },
    ]);
    expect(hintsOf(CLASSIFICATION, classificationSheet(CLASSIFICATION, ["s", "r", "q", "p"]), assigned({ p: "y", q: "x", r: "x", s: "y" })).map((hint) => hint.item)).toEqual(["s", "r", "q"]);
    expect(Object.keys(hintsOf(TRIAD, triad, assigned({ p: "z", s: "z", q: "y" }))[0]!)).toEqual(["kind", "item", "category"]);
  });

  it("pairs among the presented, assigned items only, and gives no hint for an unknown category", () => {
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "y" }))).toEqual([{ kind: "category", item: "p", category: "y" }]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "y", s: "x" }))).toEqual([{ kind: "group", item: "p", other: "s", together: false }]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({ p: "w", q: "x" }))).toEqual([{ kind: "category", item: "q", category: "x" }]);
    const drawn = classificationSheet(CLASSIFICATION, ["p", "r"]);
    expect(hintsOf(CLASSIFICATION, drawn, assigned({ p: "y", q: "y", r: "x", s: "x" }))).toEqual([
      { kind: "category", item: "p", category: "y" },
      { kind: "category", item: "r", category: "x" },
    ]);
    expect(hintsOf(CLASSIFICATION, plain, assigned({}))).toEqual([]);
    expect(hintsOf(CLASSIFICATION, plain)).toEqual([]);
    expect(hintsOf(CLASSIFICATION, plain, { kind: "sorting", order: ["p"] })).toEqual([]);
  });

  it("questions a profile on the axis with the largest gap relative to its reach, half the spread of the presented categories", () => {
    expect(hintsOf(PROFILED, profiled, assigned({ cabin: "low" }))).toEqual([{ kind: "profile", item: "cabin", category: "low", axis: "k" }]);
    expect(hintsOf(PROFILED, profiled, assigned({ barn: "passive" }))).toEqual([{ kind: "profile", item: "barn", category: "passive", axis: "k" }]);
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "low" }))).toEqual([{ kind: "profile", item: "attic", category: "low", axis: "h" }]);
    expect(hintsOf(PROFILED, profiled, assigned({ dome: "old" }))).toEqual([{ kind: "profile", item: "dome", category: "old", axis: "h" }]);
    expect(Object.keys(hintsOf(PROFILED, profiled, assigned({ cabin: "low" }))[0]!)).toEqual(["kind", "item", "category", "axis"]);
    expect(hintsOf(PROFILED, classificationSheet(PROFILED, ["cabin"], ["low", "passive"]), assigned({ cabin: "low" }))).toEqual([{ kind: "profile", item: "cabin", category: "low", axis: "h" }]);
  });

  it("measures a profile hint against an item placed in its own category whose value lies strictly between, the farthest from the own value, with the side the placement claims", () => {
    const hints = hintsOf(PROFILED, profiled, assigned({ attic: "old", barn: "low", cabin: "old", dome: "mid", eave: "unrated" }));
    expect(hints).toEqual([{ kind: "profile", item: "cabin", category: "old", axis: "h", other: "dome", above: true }]);
    expect(Object.keys(hints[0]!)).toEqual(["kind", "item", "category", "axis", "other", "above"]);
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "passive", barn: "low", cabin: "passive", dome: "mid" }))).toEqual([{ kind: "profile", item: "attic", category: "passive", axis: "h", other: "barn", above: false }]);
    expect(hintsOf(PROFILED, profiled, assigned({ cabin: "old", barn: "low" }))).toEqual([{ kind: "profile", item: "cabin", category: "old", axis: "h", other: "barn", above: true }]);
    expect(hintsOf(PROFILED, profiled, assigned({ cabin: "old", attic: "old", eave: "unrated" }))).toEqual([{ kind: "profile", item: "cabin", category: "old", axis: "h" }]);
    expect(hintsOf(PROFILED, profiled, assigned({ cabin: "old", barn: "passive", dome: "old" })).find((hint) => hint.item === "cabin")).toEqual({ kind: "profile", item: "cabin", category: "old", axis: "h" });
    const twins: ClassificationTask = { ...PROFILED, items: [...PROFILED.items, { id: "fort", label: T("fort"), category: "mid" }] };
    const both = assigned({ cabin: "old", dome: "mid", fort: "mid" });
    expect(hintsOf(twins, classificationSheet(twins, ["cabin", "fort", "dome"]), both)).toEqual([{ kind: "profile", item: "cabin", category: "old", axis: "h", other: "fort", above: true }]);
    expect(hintsOf(twins, classificationSheet(twins, ["dome", "cabin", "fort"]), both)).toEqual([{ kind: "profile", item: "cabin", category: "old", axis: "h", other: "dome", above: true }]);
    const partial: ClassificationTask = { ...twins, categories: twins.categories.map((category) => (category.id === "mid" ? { ...category, profile: { c: 20, k: 2, e: 5 } } : category)) };
    expect(hintsOf(partial, classificationSheet(partial, ["cabin", "barn", "dome"]), assigned({ cabin: "old", barn: "low", dome: "mid" }))).toEqual([{ kind: "profile", item: "cabin", category: "old", axis: "h", other: "barn", above: true }]);
  });

  it("gives at most three hints of a classification: profile hints by their gap relative to the reach first, then pairs and categories in sheet order", () => {
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "unrated", barn: "passive", cabin: "old", dome: "passive", eave: "old" }))).toEqual([
      { kind: "category", item: "attic", category: "unrated" },
      { kind: "profile", item: "barn", category: "passive", axis: "k" },
      { kind: "profile", item: "cabin", category: "old", axis: "h" },
    ]);
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "unrated", dome: "unrated", eave: "old", barn: "passive" }))).toEqual([
      { kind: "group", item: "attic", other: "dome", together: true },
      { kind: "profile", item: "barn", category: "passive", axis: "k" },
      { kind: "group", item: "dome", other: "attic", together: true },
    ]);
  });

  it("takes the first axis in axis order among equal ratios", () => {
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "passive" }))).toEqual([{ kind: "profile", item: "attic", category: "passive", axis: "h" }]);
    const reordered: ClassificationTask = { ...PROFILED, axes: [...PROFILED.axes!].reverse() };
    expect(hintsOf(reordered, classificationSheet(reordered, ["attic"]), assigned({ attic: "passive" }))).toEqual([{ kind: "profile", item: "attic", category: "passive", axis: "k" }]);
  });

  it("gives no hint for a near miss, a gap at most the reach on every axis, and none for a hint the profiles cannot give", () => {
    expect(hintsOf(PROFILED, profiled, assigned({ dome: "low" }))).toEqual([]);
    expect(hintsOf(PROFILED, profiled, assigned({ barn: "mid" }))).toEqual([]);
    expect(hintsOf(PROFILED, profiled, assigned({ dome: "passive" }))).toEqual([]);
    expect(hintsOf(PROFILED, profiled, assigned({ dome: "low", barn: "mid" }))).toEqual([]);
    const partial: ClassificationTask = { ...PROFILED, categories: PROFILED.categories.map((category) => (category.id === "passive" ? { ...category, profile: { h: 15, c: 30, e: 5 } } : category)) };
    expect(hintsOf(partial, classificationSheet(partial, ["cabin"]), assigned({ cabin: "low" }))).toEqual([]);
  });

  it("falls back to pairs and categories where one of the two categories carries no profile", () => {
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "unrated", dome: "unrated" }))).toEqual([
      { kind: "group", item: "attic", other: "dome", together: true },
      { kind: "group", item: "dome", other: "attic", together: true },
    ]);
    expect(hintsOf(PROFILED, profiled, assigned({ eave: "old", attic: "old" }))).toEqual([{ kind: "group", item: "eave", other: "attic", together: true }]);
    expect(hintsOf(PROFILED, profiled, assigned({ eave: "old" }))).toEqual([{ kind: "category", item: "eave", category: "old" }]);
    expect(hintsOf(PROFILED, profiled, assigned({ attic: "unrated", cabin: "low", dome: "low" }))).toEqual([
      { kind: "category", item: "attic", category: "unrated" },
      { kind: "profile", item: "cabin", category: "low", axis: "k" },
    ]);
  });

  it("gives the same hints at every challenge: they use the task's profiles, not the sheet's shares", () => {
    const answer = assigned({ attic: "low", barn: "passive", cabin: "old", dome: "low", eave: "mid" });
    const expected = hintsOf(PROFILED, sheetTaskOf(PROFILED, "easy", 3), answer);
    expect(expected.map((hint) => hint.item).sort()).toEqual(["attic", "barn", "cabin"]);
    for (const challenge of CHALLENGES) expect(hintsOf(PROFILED, sheetTaskOf(PROFILED, challenge, 3), answer)).toEqual(expected);
  });

  it("gives exactly the hints of the same rules over mathjs, for random profiles, sheets and answers", () => {
    const random = new Mt19937(1234);
    const kinds = new Set<string>();
    for (let round = 0; round < 400; round++) {
      const categories = Array.from({ length: 2 + (round % 4) }, (_, index) => ({
        id: `k${index}`,
        label: T(`k${index}`),
        ...(random.next() % 6 === 0 ? {} : { profile: { a: random.next() % 10, b: (random.next() % 1000) / 7, c: random.next() % 3 } }),
      }));
      const subject: ClassificationTask = {
        kind: "classification",
        id: "random",
        title: T("Random"),
        prompt: T("Classify"),
        axes: [axis("a", 10), axis("b", 150), axis("c", 3)],
        categories,
        items: Array.from({ length: 3 + (round % 6) }, (_, index) => ({ id: `t${index}`, label: T(`t${index}`), category: categories[random.next() % categories.length]!.id })),
        ...(round % 3 === 0 ? { draw: 3 } : {}),
      };
      const sheetTask = sheetTaskOf(subject, "easy", round);
      if (sheetTask.kind !== "classification") throw new Error("kind");
      const answer = assigned(Object.fromEntries(subject.items.flatMap((item) => (random.next() % 7 === 0 ? [] : [[item.id, categories[random.next() % categories.length]!.id] as const]))));
      const hints = hintsOf(subject, sheetTask, answer);
      for (const hint of hints) kinds.add(hint.kind === "group" ? `group-${hint.together}` : hint.kind === "profile" ? `profile-${hint.above ?? "alone"}` : hint.kind);
      expect(hints).toEqual(oracleClassification(subject, sheetTask, answer));
    }
    expect([...kinds].sort()).toEqual(["category", "group-false", "group-true", "profile-alone", "profile-false", "profile-true"]);
  });
});
