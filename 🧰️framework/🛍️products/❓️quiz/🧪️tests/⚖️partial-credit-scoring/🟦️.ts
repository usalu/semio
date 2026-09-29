import { distance as mathDistance } from "mathjs";
import { createRequire } from "node:module";
import { describe, expect, it } from "vitest";
import {
  Mt19937,
  scoreRun,
  scoreTask,
  sheetOf,
  shuffle,
  type Answer,
  type ClassificationTask,
  type MatchingTask,
  type MatchingTaskResult,
  type Quiz,
  type Scale,
  type SheetTask,
  type SortingTask,
  type SortingTaskResult,
  type Text,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 📊️ jStat 1.9.6, the third-party statistics oracle (CommonJS without type declarations). */
const jStat = createRequire(import.meta.url)("jstat") as { spearmancoeff(left: number[], right: number[]): number };

const T = (en: string): Text => ({ en, de: en });
const TOLERANCE = 1e-12;

/** 📶️ A sorting task over the given values, item `i<index>` carrying `values[index]`. */
function sortingTask(values: readonly number[], scale: Scale = "linear"): SortingTask {
  return { kind: "sorting", id: "sorting", title: T("Sorting"), prompt: T("Sort"), quantity: { label: T("Q"), unit: "u", scale, prefixed: false }, items: values.map((value, index) => ({ id: `i${index}`, label: T(`i${index}`), value, ...(index === 0 ? { explanation: T("first") } : {}) })) };
}

/** 🪜️ The sheet task of a single-task quiz. */
function sheetTaskOf(task: SortingTask | MatchingTask | ClassificationTask, seed = 1): SheetTask {
  return sheetOf({ schema: "semio.quiz/v1", id: "quiz", emoji: "❓", title: T("Quiz"), description: T("Quiz"), tasks: [task] }, seed).tasks[0]!;
}

/** 🧮️ The sorting score of an order of item ids. */
function sortingScore(task: SortingTask, order: readonly string[]): number {
  return scoreTask(task, sheetTaskOf(task), { kind: "sorting", order: [...order] })!.score;
}

/** 🆔️ The item ids of a sorting task sorted by value (ties by definition index). */
function ascendingIds(task: SortingTask): string[] {
  return task.items.map((item, index) => ({ item, index })).sort((left, right) => left.item.value - right.item.value || left.index - right.index).map(({ item }) => item.id);
}

/** 🎲️ Random values: distinct integers or spread over orders of magnitude. */
function randomValues(random: Mt19937, count: number, scale: Scale): number[] {
  return Array.from({ length: count }, () => (scale === "logarithmic" ? 10 ** ((random.next() % 12000) / 1000) : (random.next() % 20001) - 10000));
}

describe("sorting — magnitude-weighted pair concordance", () => {
  it("scores 1 for the ascending order and 0 for the reversed order, exactly", () => {
    const random = new Mt19937(11);
    for (const scale of ["linear", "logarithmic"] as const) {
      for (let round = 0; round < 50; round++) {
        const task = sortingTask(randomValues(random, 2 + (round % 9), scale), scale);
        const ascending = ascendingIds(task);
        expect(sortingScore(task, ascending)).toBe(1);
        if (new Set(task.items.map((item) => item.value)).size === task.items.length) expect(sortingScore(task, [...ascending].reverse())).toBe(0);
      }
    }
  });

  it("equals (1 + Spearman ρ) / 2 when the values are equally spaced ranks — checked against jStat", () => {
    const random = new Mt19937(2026);
    for (let round = 0; round < 300; round++) {
      const count = 2 + (round % 14);
      const offset = (random.next() % 1000) - 500;
      const step = 1 + (random.next() % 9);
      const task = sortingTask(Array.from({ length: count }, (_, rank) => offset + step * rank));
      const order = shuffle(random, task.items.map((item) => item.id));
      const ranks = order.map((id) => Number(id.slice(1)));
      const rho = jStat.spearmancoeff(order.map((_, position) => position), ranks);
      expect(Math.abs(sortingScore(task, order) - (1 + rho) / 2)).toBeLessThan(TOLERANCE);
    }
  });

  it("is not (1 + Kendall τₐ) / 2 on ranks: the pair weights make it Spearman's footrule-of-squares instead", () => {
    const task = sortingTask([0, 1, 2]);
    const order = ["i1", "i0", "i2"];
    let concordant = 0;
    let discordant = 0;
    for (let i = 0; i < order.length; i++) for (let j = i + 1; j < order.length; j++) (Number(order[i]!.slice(1)) < Number(order[j]!.slice(1)) ? concordant++ : discordant++);
    const tau = (concordant - discordant) / 3;
    expect(sortingScore(task, order)).toBe(0.75);
    expect((1 + tau) / 2).toBeCloseTo(2 / 3, 15);
  });

  it("never decreases when an adjacent inversion is repaired, and strictly increases for distinct values", () => {
    const random = new Mt19937(77);
    for (const scale of ["linear", "logarithmic"] as const) {
      for (let round = 0; round < 200; round++) {
        const task = sortingTask(randomValues(random, 3 + (round % 8), scale), scale);
        const order = shuffle(random, task.items.map((item) => item.id));
        const value = (id: string) => task.items.find((item) => item.id === id)!.value;
        const position = order.findIndex((id, index) => index + 1 < order.length && value(id) > value(order[index + 1]!));
        if (position < 0) continue;
        const repaired = [...order];
        [repaired[position], repaired[position + 1]] = [repaired[position + 1]!, repaired[position]!];
        expect(sortingScore(task, repaired)).toBeGreaterThan(sortingScore(task, order));
      }
    }
  });

  it("punishes swapping distant magnitudes more than swapping neighbours", () => {
    const task = sortingTask([1, 60, 2000, 100000, 3000000, 1.4e9], "logarithmic");
    const ascending = ascendingIds(task);
    const neighbours = [ascending[1]!, ascending[0]!, ...ascending.slice(2)];
    const extremes = [ascending[5]!, ...ascending.slice(1, 5), ascending[0]!];
    expect(sortingScore(task, neighbours)).toBeGreaterThan(0.95);
    expect(sortingScore(task, extremes)).toBeLessThan(0.3);
  });

  it("measures distances on the quantity's scale", () => {
    const decades = [1, 10, 100, 1000];
    const low = ["i1", "i0", "i2", "i3"];
    const high = ["i0", "i1", "i3", "i2"];
    const logarithmic = sortingTask(decades, "logarithmic");
    expect(Math.abs(sortingScore(logarithmic, low) - sortingScore(logarithmic, high))).toBeLessThan(TOLERANCE);
    const linear = sortingTask(decades, "linear");
    expect(sortingScore(linear, low)).toBeGreaterThan(0.99);
    expect(sortingScore(linear, high)).toBeLessThan(0.75);
  });

  it("ignores the order among equal values and scores 1 when all values are equal", () => {
    const tied = sortingTask([5, 1, 5, 9]);
    expect(sortingScore(tied, ["i1", "i2", "i0", "i3"])).toBe(1);
    expect(sortingScore(tied, ["i1", "i0", "i2", "i3"])).toBe(1);
    expect(sortingScore(sortingTask([3, 3, 3]), ["i2", "i0", "i1"])).toBe(1);
  });

  it("reports items in the learner's order with value, position, true rank (ties by definition index) and explanation", () => {
    const task = sortingTask([5, 1, 5]);
    const result = scoreTask(task, sheetTaskOf(task), { kind: "sorting", order: ["i2", "i1", "i0"] }) as SortingTaskResult;
    expect(result.items).toEqual([
      { item: "i2", value: 5, position: 0, rank: 2 },
      { item: "i1", value: 1, position: 1, rank: 0 },
      { item: "i0", value: 5, position: 2, rank: 1, explanation: T("first") },
    ]);
    expect(result.kind).toBe("sorting");
    expect(result.task).toBe("sorting");
  });

  it("ranks among the drawn items only", () => {
    const task = { ...sortingTask([1, 2, 3, 4, 5, 6]), draw: 3 };
    const sheetTask = sheetTaskOf(task, 9);
    const order = sheetTask.items.map((item) => item.id).sort((left, right) => Number(left.slice(1)) - Number(right.slice(1)));
    const result = scoreTask(task, sheetTask, { kind: "sorting", order }) as SortingTaskResult;
    expect(result.items.map((item) => item.rank)).toEqual([0, 1, 2]);
    expect(result.score).toBe(1);
  });
});

/** 🔗️ A matching task over items `m<index>` with the given values per dimension. */
function matchingTask(dimensions: Readonly<Record<string, { readonly scale: Scale; readonly values: readonly number[] }>>): MatchingTask {
  const count = Object.values(dimensions)[0]!.values.length;
  return {
    kind: "matching",
    id: "matching",
    title: T("Matching"),
    prompt: T("Match"),
    dimensions: Object.entries(dimensions).map(([id, dimension]) => ({ id, quantity: { label: T(id), unit: "u", scale: dimension.scale, prefixed: false } })),
    items: Array.from({ length: count }, (_, index) => ({ id: `m${index}`, label: T(`m${index}`), values: Object.fromEntries(Object.entries(dimensions).map(([id, dimension]) => [id, dimension.values[index]!])) })),
  };
}

/** 🎴️ A matching answer assigning every item the card holding `pick(item's true value)`, each card once. */
function matchingAnswer(sheetTask: SheetTask, pick: (dimension: string, item: string) => number): Answer {
  if (sheetTask.kind !== "matching") throw new Error("kind");
  return {
    kind: "matching",
    assignments: Object.fromEntries(
      sheetTask.dimensions.map((dimension) => {
        const used = new Set<number>();
        return [
          dimension.id,
          Object.fromEntries(
            sheetTask.items.map((item) => {
              const card = dimension.cards.findIndex((value, index) => value === pick(dimension.id, item.id) && !used.has(index));
              used.add(card);
              return [item.id, card];
            }),
          ),
        ];
      }),
    ),
  };
}

describe("matching — per-dimension weighted concordance", () => {
  const task = matchingTask({ "u-value": { scale: "linear", values: [1.4, 0.5, 0.24, 0.12] }, demand: { scale: "logarithmic", values: [250, 120, 70, 15] } });
  const truth = (dimension: string, item: string) => task.items.find((candidate) => candidate.id === item)!.values[dimension]!;

  it("scores 1 for the true values and 0 for the mirrored values in every dimension", () => {
    for (let seed = 0; seed < 20; seed++) {
      const sheetTask = sheetTaskOf(task, seed);
      expect(scoreTask(task, sheetTask, matchingAnswer(sheetTask, truth))!.score).toBe(1);
      const mirrored = (dimension: string, item: string) => {
        const values = task.items.map((candidate) => candidate.values[dimension]!).sort((left, right) => left - right);
        return values[values.length - 1 - values.indexOf(truth(dimension, item))]!;
      };
      expect(scoreTask(task, sheetTask, matchingAnswer(sheetTask, mirrored))!.score).toBe(0);
    }
  });

  it("counts a tie of assigned values on distinct true values half", () => {
    const tied = matchingTask({ size: { scale: "linear", values: [1, 2, 2] } });
    const sheetTask = sheetTaskOf(tied, 3);
    if (sheetTask.kind !== "matching") throw new Error("kind");
    const cards = sheetTask.dimensions[0]!.cards;
    const one = cards.indexOf(1);
    const twos = cards.flatMap((value, index) => (value === 2 ? [index] : []));
    const result = scoreTask(tied, sheetTask, { kind: "matching", assignments: { size: { m0: twos[0]!, m1: twos[1]!, m2: one } } }) as MatchingTaskResult;
    expect(result.dimensions[0]!.score).toBe(0.25);
    expect(result.dimensions[0]!.items.map(({ item, assigned, correct }) => ({ item, assigned, correct }))).toEqual(sheetTask.items.map((item) => ({ item: item.id, assigned: item.id === "m2" ? 1 : 2, correct: tied.items.find((candidate) => candidate.id === item.id)!.values.size })));
  });

  it("scores a dimension like the sorting its assigned cards induce", () => {
    const random = new Mt19937(5);
    for (let round = 0; round < 100; round++) {
      const values = shuffle(
        random,
        Array.from({ length: 3 + (round % 6) }, (_, rank) => 10 ** (rank * 0.7 + (random.next() % 100) / 1000)),
      );
      const single = matchingTask({ power: { scale: "logarithmic", values } });
      const sheetTask = sheetTaskOf(single, round);
      if (sheetTask.kind !== "matching") throw new Error("kind");
      const permutation = shuffle(random, sheetTask.dimensions[0]!.cards.map((_, index) => index));
      const assignments = Object.fromEntries(sheetTask.items.map((item, index) => [item.id, permutation[index]!]));
      const matched = scoreTask(single, sheetTask, { kind: "matching", assignments: { power: assignments } })!.score;
      const induced = sheetTask.items.map((item) => ({ id: item.id, card: sheetTask.dimensions[0]!.cards[assignments[item.id]!]! })).sort((left, right) => left.card - right.card);
      const sorting = sortingTask(values, "logarithmic");
      const renamed = induced.map(({ id }) => `i${id.slice(1)}`);
      expect(Math.abs(matched - sortingScore(sorting, renamed))).toBeLessThan(TOLERANCE);
    }
  });

  it("averages the dimension scores in definition order", () => {
    const sheetTask = sheetTaskOf(task, 4);
    const answer = matchingAnswer(sheetTask, (dimension, item) => (dimension === "demand" ? truth(dimension, item) : truth(dimension, item === "m0" ? "m1" : item === "m1" ? "m0" : item)));
    const result = scoreTask(task, sheetTask, answer) as MatchingTaskResult;
    expect(result.dimensions.map((dimension) => dimension.dimension)).toEqual(["u-value", "demand"]);
    expect(result.dimensions[1]!.score).toBe(1);
    expect(result.dimensions[0]!.score).toBeLessThan(1);
    expect(result.score).toBe((result.dimensions[0]!.score + 1) / 2);
  });
});

describe("classification — profile similarity", () => {
  const task: ClassificationTask = {
    kind: "classification",
    id: "classification",
    title: T("Classification"),
    prompt: T("Classify"),
    axes: [
      { id: "heating", label: T("Heating"), unit: "kWh", min: 0, max: 200 },
      { id: "cooling", label: T("Cooling"), unit: "kWh", min: 10, max: 60 },
      { id: "venting", label: T("Venting"), unit: "1/h", min: 0, max: 4 },
      { id: "cost", label: T("Cost"), unit: "€", min: -5, max: 5 },
    ],
    categories: [
      { id: "passive", label: T("Passive"), profile: { heating: 15, cooling: 20, venting: 0.5, cost: 4 } },
      { id: "low-energy", label: T("Low energy"), profile: { heating: 50, cooling: 25, venting: 0.7, cost: 2 } },
      { id: "old", label: T("Old"), profile: { heating: 200, cooling: 50, venting: 3.5, cost: -5 } },
      { id: "unrated", label: T("Unrated") },
    ],
    items: [
      { id: "a", label: T("a"), category: "passive", explanation: T("insulated") },
      { id: "b", label: T("b"), category: "low-energy" },
      { id: "c", label: T("c"), category: "old" },
      { id: "d", label: T("d"), category: "unrated" },
    ],
  };
  const normalised = (category: string) => task.axes!.map((axis) => (task.categories.find((candidate) => candidate.id === category)!.profile![axis.id]! - axis.min) / (axis.max - axis.min));
  const profiled = ["passive", "low-energy", "old"];
  const farthest = Math.max(...profiled.flatMap((left, index) => profiled.slice(index + 1).map((right) => mathDistance(normalised(left), normalised(right)) as number)));

  /** 🗃️ The credits of a classification in sheet order, keyed by item. */
  function credits(assignments: Readonly<Record<string, string>>): Record<string, number> {
    const sheetTask = sheetTaskOf(task, 12);
    const result = scoreTask(task, sheetTask, { kind: "classification", assignments });
    if (result?.kind !== "classification") throw new Error("kind");
    expect(result.items.map((item) => item.item)).toEqual(sheetTask.items.map((item) => item.id));
    expect(Math.abs(result.score - result.items.reduce((sum, item) => sum + item.credit, 0) / result.items.length)).toBeLessThan(TOLERANCE);
    return Object.fromEntries(result.items.map((item) => [item.item, item.credit]));
  }

  it("gives full credit to the correct category", () => {
    expect(credits({ a: "passive", b: "low-energy", c: "old", d: "unrated" })).toEqual({ a: 1, b: 1, c: 1, d: 1 });
  });

  it("gives 1 − d / d_max for a wrong profiled category — checked against mathjs distances", () => {
    const got = credits({ a: "low-energy", b: "old", c: "passive", d: "unrated" });
    const expected = (assigned: string, correct: string) => Math.max(0, 1 - (mathDistance(normalised(assigned), normalised(correct)) as number) / farthest);
    expect(Math.abs(got.a! - expected("low-energy", "passive"))).toBeLessThan(TOLERANCE);
    expect(Math.abs(got.b! - expected("old", "low-energy"))).toBeLessThan(TOLERANCE);
    expect(got.a).toBeGreaterThan(got.b!);
    expect(got.c).toBe(0);
  });

  it("gives nothing when either category lacks a profile", () => {
    expect(credits({ a: "unrated", b: "low-energy", c: "old", d: "passive" })).toEqual({ a: 0, b: 1, c: 1, d: 0 });
  });

  it("gives nothing when the task has no axes or every profile coincides", () => {
    const flat: ClassificationTask = { ...task, categories: task.categories.map((category) => (category.profile ? { ...category, profile: { heating: 1, cooling: 20, venting: 1, cost: 0 } } : category)) };
    const sheetTask = sheetTaskOf(flat, 12);
    expect(scoreTask(flat, sheetTask, { kind: "classification", assignments: { a: "old", b: "old", c: "old", d: "old" } })!.score).toBe(0.25);
    const axisless: ClassificationTask = { ...task, axes: undefined };
    expect(scoreTask(axisless, sheetTaskOf(axisless, 12), { kind: "classification", assignments: { a: "low-energy", b: "low-energy", c: "low-energy", d: "low-energy" } })!.score).toBe(0.25);
  });

  it("carries the explanation of each item", () => {
    const result = scoreTask(task, sheetTaskOf(task, 12), { kind: "classification", assignments: { a: "passive", b: "old", c: "old", d: "old" } });
    if (result?.kind !== "classification") throw new Error("kind");
    expect(result.items.find((item) => item.item === "a")).toEqual({ item: "a", assigned: "passive", correct: "passive", credit: 1, explanation: T("insulated") });
    expect(result.items.find((item) => item.item === "b")).not.toHaveProperty("explanation");
  });
});

describe("scoreRun", () => {
  const sorting = sortingTask([1, 2, 3]);
  const classification: ClassificationTask = {
    kind: "classification",
    id: "classification",
    title: T("C"),
    prompt: T("C"),
    categories: [
      { id: "x", label: T("x") },
      { id: "y", label: T("y") },
    ],
    items: [
      { id: "p", label: T("p"), category: "x" },
      { id: "q", label: T("q"), category: "y" },
    ],
  };
  const quiz: Quiz = { schema: "semio.quiz/v1", id: "run", emoji: "🏁", title: T("Run"), description: T("Run"), tasks: [sorting, classification] };

  it("scores every task in sheet order and averages them", () => {
    const sheet = sheetOf(quiz, 99);
    const result = scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] }, classification: { kind: "classification", assignments: { p: "x", q: "x" } } });
    if (!result) throw new Error("unscored");
    expect(result.quiz).toBe("run");
    expect(result.tasks.map((task) => task.task)).toEqual(sheet.tasks.map((task) => task.id));
    expect(result.score).toBe(0.75);
  });

  it("scores nothing unless every sheet task has a valid, complete answer", () => {
    const sheet = sheetOf(quiz, 99);
    const units: Answer = { kind: "classification", assignments: { p: "x", q: "y" } };
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] } })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] }, classification: { kind: "classification", assignments: { p: "x" } } })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "classification", assignments: {} }, classification: units })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1"] }, classification: units })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] }, classification: { kind: "classification", assignments: { p: "x", q: "z" } } })).toBeUndefined();
    expect(scoreRun({ ...quiz, tasks: [sorting] }, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] }, classification: units })).toBeUndefined();
  });
});

describe("robustness — inputs that bypass validation never throw and never yield NaN", () => {
  it("scores an empty run, an empty classification and a dimensionless matching 0", () => {
    const empty: Quiz = { schema: "semio.quiz/v1", id: "empty", emoji: "🫙", title: T("E"), description: T("E"), tasks: [] };
    expect(scoreRun(empty, sheetOf(empty, 1), {})).toEqual({ quiz: "empty", score: 0, tasks: [] });
    const nothing: ClassificationTask = { kind: "classification", id: "nothing", title: T("N"), prompt: T("N"), categories: [{ id: "x", label: T("x") }], items: [] };
    expect(scoreTask(nothing, sheetTaskOf(nothing), { kind: "classification", assignments: {} })).toEqual({ kind: "classification", task: "nothing", score: 0, items: [] });
    const flat: MatchingTask = { ...matchingTask({ size: { scale: "linear", values: [1, 2] } }), dimensions: [] };
    expect(scoreTask(flat, sheetTaskOf(flat), { kind: "matching", assignments: {} })).toEqual({ kind: "matching", task: "matching", score: 0, dimensions: [] });
  });

  it("treats a category with an incomplete profile as unprofiled, for d_max and credits alike", () => {
    const axes = [0, 1, 2].map((index) => ({ id: `a${index}`, label: T(`a${index}`), unit: "u", min: 0, max: 10 }));
    const task: ClassificationTask = {
      kind: "classification",
      id: "partial",
      title: T("P"),
      prompt: T("P"),
      axes,
      categories: [
        { id: "near", label: T("near"), profile: { a0: 1, a1: 1, a2: 1 } },
        { id: "mid", label: T("mid"), profile: { a0: 4, a1: 4, a2: 4 } },
        { id: "broken", label: T("broken"), profile: { a0: 10, a1: 10 } },
      ],
      items: [
        { id: "p", label: T("p"), category: "near" },
        { id: "q", label: T("q"), category: "broken" },
      ],
    };
    const result = scoreTask(task, sheetTaskOf(task), { kind: "classification", assignments: { p: "mid", q: "near" } });
    if (result?.kind !== "classification") throw new Error("kind");
    const credits = Object.fromEntries(result.items.map((item) => [item.item, item.credit]));
    expect(credits).toEqual({ p: 0, q: 0 });
    expect(Number.isNaN(result.score)).toBe(false);
    const withFar: ClassificationTask = { ...task, categories: [...task.categories, { id: "far", label: T("far"), profile: { a0: 10, a1: 10, a2: 10 } }] };
    const scored = scoreTask(withFar, sheetTaskOf(withFar), { kind: "classification", assignments: { p: "mid", q: "near" } });
    if (scored?.kind !== "classification") throw new Error("kind");
    const expected = 1 - (mathDistance([0.1, 0.1, 0.1], [0.4, 0.4, 0.4]) as number) / (mathDistance([0.1, 0.1, 0.1], [1, 1, 1]) as number);
    expect(Math.abs(scored.items.find((item) => item.item === "p")!.credit - expected)).toBeLessThan(TOLERANCE);
    expect(scored.items.find((item) => item.item === "q")!.credit).toBe(0);
  });

  it("presents a missing matching value as a NaN card and leaves the task unscored", () => {
    const task: MatchingTask = { ...matchingTask({ size: { scale: "linear", values: [1, 2, 3] } }) };
    const broken: MatchingTask = { ...task, items: task.items.map((item) => (item.id === "m1" ? { ...item, values: {} } : item)) };
    const sheetTask = sheetTaskOf(broken, 6);
    if (sheetTask.kind !== "matching") throw new Error("kind");
    const cards = sheetTask.dimensions[0]!.cards;
    expect(cards).toHaveLength(3);
    expect(cards.filter((card) => Number.isNaN(card))).toHaveLength(1);
    expect(cards.every((card) => typeof card === "number")).toBe(true);
    const answer: Answer = { kind: "matching", assignments: { size: Object.fromEntries(sheetTask.items.map((item, index) => [item.id, index])) } };
    expect(scoreTask(broken, sheetTask, answer)).toBeUndefined();
  });

  it("returns undefined where the Rust twin returns None", () => {
    const task = sortingTask([1, 2, 3]);
    const sheetTask = sheetTaskOf(task);
    const order = sheetTask.items.map((item) => item.id);
    expect(scoreTask(task, sheetTask, { kind: "sorting", order })).toBeDefined();
    expect(scoreTask({ ...task, id: "other" }, sheetTask, { kind: "sorting", order })).toBeUndefined();
    expect(scoreTask(task, sheetTask, { kind: "classification", assignments: {} })).toBeUndefined();
    expect(scoreTask(task, sheetTask, { kind: "sorting", order: [...order, order[0]!] })).toBeUndefined();
    expect(scoreTask({ ...task, items: task.items.slice(1) }, sheetTask, { kind: "sorting", order })).toBeUndefined();
    const matching = matchingTask({ size: { scale: "linear", values: [1, 2] } });
    const presented = sheetTaskOf(matching);
    expect(scoreTask(matching, presented, { kind: "matching", assignments: { size: { m0: 0 } } })).toBeUndefined();
    expect(scoreTask({ ...matching, dimensions: [] }, presented, { kind: "matching", assignments: { size: { m0: 0, m1: 1 } } })).toBeUndefined();
  });
});
