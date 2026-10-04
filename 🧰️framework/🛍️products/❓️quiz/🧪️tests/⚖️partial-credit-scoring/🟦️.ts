import { abs as mathAbs, distance as mathDistance, divide as mathDivide, log10 as mathLog10, max as mathMax, min as mathMin, sqrt as mathSqrt, subtract as mathSubtract } from "mathjs";
import { createRequire } from "node:module";
import { describe, expect, it } from "vitest";
import {
  CHALLENGES,
  CHALLENGE_RULES,
  Mt19937,
  scoreRun,
  scoreTask,
  sheetOf,
  shuffle,
  type Answer,
  type Challenge,
  type ClassificationTask,
  type ClassificationTaskResult,
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
  return { kind: "sorting", id: "sorting", title: T("Sorting"), prompt: T("Sort"), quantity: { label: T("Q"), unit: "u", scale, prefixed: false, additive: false }, items: values.map((value, index) => ({ id: `i${index}`, label: T(`i${index}`), value, ...(index === 0 ? { explanation: T("first") } : {}) })) };
}

/** 🪜️ The sheet task of a single-task quiz, at medium unless a challenge is named. */
function sheetTaskOf(task: SortingTask | MatchingTask | ClassificationTask, seed = 1, challenge: Challenge = "medium"): SheetTask {
  return sheetOf({ schema: "semio.quiz/v1", id: "quiz", emoji: "❓", title: T("Quiz"), description: T("Quiz"), tasks: [task] }, seed, challenge).tasks[0]!;
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

  it("scores the order alone where the keys show, alike on easy and medium, with neither a guess nor a miss in the item results", () => {
    const task = sortingTask([1, 60, 2000, 100000], "logarithmic");
    const order = ["i1", "i0", "i3", "i2"];
    const medium = scoreTask(task, sheetTaskOf(task), { kind: "sorting", order }) as SortingTaskResult;
    expect(scoreTask(task, sheetTaskOf(task, 1, "easy"), { kind: "sorting", order })).toEqual(medium);
    for (const item of medium.items) expect(Object.keys(item).filter((key) => key === "guess" || key === "miss")).toEqual([]);
  });

  it("scores no answer that carries guesses where the keys show", () => {
    const task = sortingTask([1, 60, 2000, 100000], "logarithmic");
    const order = ["i1", "i0", "i3", "i2"];
    for (const challenge of ["easy", "medium"] as const) {
      expect(scoreTask(task, sheetTaskOf(task, 1, challenge), { kind: "sorting", order, guesses: { i1: 0.5, i0: 3, i2: 1e12 } })).toBeUndefined();
      expect(scoreTask(task, sheetTaskOf(task, 1, challenge), { kind: "sorting", order, guesses: {} })).toBeUndefined();
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
    dimensions: Object.entries(dimensions).map(([id, dimension]) => ({ id, quantity: { label: T(id), unit: "u", scale: dimension.scale, prefixed: false, additive: false } })),
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
              const card = dimension.cards!.findIndex((value, index) => value === pick(dimension.id, item.id) && !used.has(index));
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
    const cards = sheetTask.dimensions[0]!.cards!;
    const one = cards.indexOf(1);
    const twos = cards.flatMap((value, index) => (value === 2 ? [index] : []));
    const result = scoreTask(tied, sheetTask, { kind: "matching", assignments: { size: { m0: twos[0]!, m1: twos[1]!, m2: one } } }) as MatchingTaskResult;
    expect(result.dimensions[0]!.score).toBe(0.25);
    expect(result.dimensions[0]!.items).toEqual(sheetTask.items.map((item) => ({ item: item.id, assigned: item.id === "m2" ? 1 : 2, correct: tied.items.find((candidate) => candidate.id === item.id)!.values.size })));
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
      const cards = sheetTask.dimensions[0]!.cards!;
      const permutation = shuffle(random, cards.map((_, index) => index));
      const assignments = Object.fromEntries(sheetTask.items.map((item, index) => [item.id, permutation[index]!]));
      const matched = scoreTask(single, sheetTask, { kind: "matching", assignments: { power: assignments } })!.score;
      const induced = sheetTask.items.map((item) => ({ id: item.id, card: cards[assignments[item.id]!]! })).sort((left, right) => left.card - right.card);
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

/** 📐️ A value on its scale, by mathjs. */
function oracleScaled(value: number, scale: Scale): number {
  return scale === "logarithmic" ? (mathLog10(value) as number) : value;
}

/** 📡️ The reach of presented true values, by mathjs: a factor `min(1000, sqrt(hi / lo))` on a logarithmic scale (1000 when they do not spread), a distance `(hi − lo) / 2` on a linear one (unbounded when they do not spread). */
function oracleReach(values: readonly number[], scale: Scale): number {
  const [lo, hi] = [mathMin([...values]) as number, mathMax([...values]) as number];
  if (scale === "linear") return hi > lo ? (mathDivide(mathSubtract(hi, lo), 2) as number) : Infinity;
  return hi > lo ? (mathMin(1000, mathSqrt(mathDivide(hi, lo) as number) as number) as number) : 1000;
}

/** 🎯️ Which items miss, by mathjs: no guess, or one off the truth by more than the reach — the ratio of the larger to the smaller on a logarithmic scale, the distance on a linear one. */
function oracleMisses(truths: readonly number[], guesses: readonly (number | undefined)[], scale: Scale): boolean[] {
  const within = oracleReach(truths, scale);
  return truths.map((truth, index) => {
    const guess = guesses[index];
    if (guess === undefined) return true;
    return (scale === "linear" ? (mathAbs(mathSubtract(guess, truth)) as number) : (mathDivide(mathMax(guess, truth), mathMin(guess, truth)) as number)) > within;
  });
}

/** ⚗️ The guessed concordance of items in a given order, by mathjs: a pair is discordant when it stands the wrong way round or either item misses, half discordant (matching only) when the guesses tie on distinct truths. */
function oracleGuessed(truths: readonly number[], guesses: readonly (number | undefined)[], scale: Scale, matching: boolean): number {
  const missed = oracleMisses(truths, guesses, scale);
  let total = 0;
  let discordant = 0;
  for (let i = 0; i < truths.length; i++) {
    for (let j = i + 1; j < truths.length; j++) {
      const weight = mathAbs(oracleScaled(truths[i]!, scale) - oracleScaled(truths[j]!, scale)) as number;
      total += weight;
      if (missed[i] || missed[j]) discordant += weight;
      else if (!matching) discordant += truths[i]! > truths[j]! ? weight : 0;
      else if ((truths[i]! - truths[j]!) * (guesses[i]! - guesses[j]!) < 0) discordant += weight;
      else if (guesses[i] === guesses[j] && truths[i] !== truths[j]) discordant += weight / 2;
    }
  }
  return total > 0 ? 1 - discordant / total : missed.includes(true) ? 0 : 1;
}

/** 🔮️ A sorting answer made of guesses: the guessed items in ascending guess order (ties by item index), the unguessed ones after them. */
function guessedSorting(task: SortingTask, guesses: Readonly<Record<string, number>>): Extract<Answer, { kind: "sorting" }> {
  const index = (id: string) => Number(id.slice(1));
  const guessed = Object.keys(guesses).sort((left, right) => guesses[left]! - guesses[right]! || index(left) - index(right));
  return { kind: "sorting", order: [...guessed, ...task.items.map((item) => item.id).filter((id) => !Object.hasOwn(guesses, id))], guesses };
}

/** 🎰️ A guess around a true value: within a few decades on a logarithmic scale, within the spread on a linear one. */
function randomGuess(random: Mt19937, truth: number, scale: Scale): number {
  const offset = (random.next() % 10001) / 1000 - 5;
  return scale === "logarithmic" ? truth * 10 ** offset : truth + offset * 3000;
}

describe("sorting — guesses where the keys are hidden", () => {
  const decades = sortingTask([1, 1e3, 1e6, 1e9], "logarithmic");

  it("stays perfect with the right order and every guess within reach: exactness is not required", () => {
    for (const challenge of ["hard", "expert"] as const) {
      const result = scoreTask(decades, sheetTaskOf(decades, 1, challenge), guessedSorting(decades, { i0: 5, i1: 200, i2: 3e7, i3: 1e8 })) as SortingTaskResult;
      expect(result.score).toBe(1);
      expect(result.items).toEqual([
        { item: "i0", value: 1, position: 0, rank: 0, guess: 5, miss: false, explanation: T("first") },
        { item: "i1", value: 1e3, position: 1, rank: 1, guess: 200, miss: false },
        { item: "i2", value: 1e6, position: 2, rank: 2, guess: 3e7, miss: false },
        { item: "i3", value: 1e9, position: 3, rank: 3, guess: 1e8, miss: false },
      ]);
      expect(Object.keys(result.items[0]!)).toEqual(["item", "value", "position", "rank", "guess", "miss", "explanation"]);
    }
  });

  it("earns no perfect score for typing 1, 2, 3, 4 in the right order: a miss costs every pair it touches", () => {
    const result = scoreTask(decades, sheetTaskOf(decades, 1, "hard"), guessedSorting(decades, { i0: 1, i1: 2, i2: 3, i3: 4 })) as SortingTaskResult;
    expect(result.items.map((item) => item.miss)).toEqual([false, false, true, true]);
    expect(result.items.map((item) => item.position)).toEqual([0, 1, 2, 3]);
    expect(Math.abs(result.score - (1 - (6 + 9 + 3 + 6 + 3) / 30))).toBeLessThan(TOLERANCE);
  });

  it("reaches the factor 1000 on a logarithmic scale, or the square root of the values' max/min ratio where that is less, and misses only beyond", () => {
    const wide = (guess: number) => (scoreTask(decades, sheetTaskOf(decades, 1, "hard"), guessedSorting(decades, { i0: 1, i1: guess, i2: 1e6, i3: 1e9 })) as SortingTaskResult).items.find((item) => item.item === "i1")!.miss;
    expect(wide(1e3)).toBe(false);
    expect(wide(1e3 * 999)).toBe(false);
    expect(wide(1)).toBe(false);
    expect(wide(1e3 * 1001)).toBe(true);
    expect(wide(0.999)).toBe(true);
    const narrow = sortingTask([10, 100, 1000], "logarithmic");
    const within = (guess: number) => (scoreTask(narrow, sheetTaskOf(narrow, 1, "hard"), guessedSorting(narrow, { i0: 10, i1: guess, i2: 1000 })) as SortingTaskResult).items.find((item) => item.item === "i1")!.miss;
    expect(within(100 * 9.9)).toBe(false);
    expect(within(100 / 9.9)).toBe(false);
    expect(within(100 * 10.1)).toBe(true);
    expect(within(100 / 10.1)).toBe(true);
  });

  it("reaches half the spread on a linear scale, whatever the magnitudes", () => {
    const task = sortingTask([0, 10, 20]);
    const sheetTask = sheetTaskOf(task, 1, "hard");
    const edge = scoreTask(task, sheetTask, guessedSorting(task, { i0: 10, i1: 20, i2: 30 })) as SortingTaskResult;
    expect(edge.items.map((item) => item.miss)).toEqual([false, false, false]);
    expect(edge.score).toBe(1);
    const beyond = scoreTask(task, sheetTask, guessedSorting(task, { i0: 10, i1: 20, i2: 30.5 })) as SortingTaskResult;
    expect(beyond.items.map((item) => item.miss)).toEqual([false, false, true]);
    expect(beyond.score).toBe(1 - 30 / 40);
    const huge = sortingTask([0, 1e9, 2e9]);
    expect((scoreTask(huge, sheetTaskOf(huge, 1, "hard"), guessedSorting(huge, { i0: 5e8, i1: 1.4e9, i2: 1.5e9 })) as SortingTaskResult).score).toBe(1);
  });

  it("scores the wrong order of guesses within reach like the order alone", () => {
    const sheetTask = sheetTaskOf(decades, 1, "hard");
    const swapped = scoreTask(decades, sheetTask, guessedSorting(decades, { i1: 50, i0: 60, i2: 1e6, i3: 1e9 })) as SortingTaskResult;
    expect(swapped.items.map((item) => item.item)).toEqual(["i1", "i0", "i2", "i3"]);
    expect(swapped.items.map((item) => item.miss)).toEqual([false, false, false, false]);
    expect(swapped.score).toBe(sortingScore(decades, ["i1", "i0", "i2", "i3"]));
  });

  it("equals (1 + Spearman ρ) / 2 on equally spaced values when no guess misses — checked against jStat", () => {
    const random = new Mt19937(404);
    for (let round = 0; round < 200; round++) {
      const count = 2 + (round % 12);
      const offset = (random.next() % 1000) - 500;
      const step = 1 + (random.next() % 9);
      const task = sortingTask(Array.from({ length: count }, (_, rank) => offset + step * rank));
      const order = shuffle(random, task.items.map((item) => item.id));
      const middle = offset + (step * (count - 1)) / 2;
      const result = scoreTask(task, sheetTaskOf(task, round, "hard"), { kind: "sorting", order, guesses: Object.fromEntries(order.map((id) => [id, middle])) }) as SortingTaskResult;
      expect(result.items.every((item) => item.miss === false && item.guess === middle)).toBe(true);
      const rho = jStat.spearmancoeff(order.map((_, position) => position), order.map((id) => Number(id.slice(1))));
      expect(Math.abs(result.score - (1 + rho) / 2)).toBeLessThan(TOLERANCE);
    }
  });

  it("agrees with the formula written over mathjs, misses and missing guesses included", () => {
    const random = new Mt19937(31337);
    for (const scale of ["linear", "logarithmic"] as const) {
      for (let round = 0; round < 300; round++) {
        const task = sortingTask(randomValues(random, 2 + (round % 8), scale), scale);
        const partial = round % 3 === 0;
        const guesses = Object.fromEntries(task.items.flatMap((item) => (partial && random.next() % 4 === 0 ? [] : [[item.id, randomGuess(random, item.value, scale)] as const])));
        const answer = guessedSorting(task, guesses);
        const result = scoreTask(task, sheetTaskOf(task, round, partial ? "expert" : "hard"), answer) as SortingTaskResult;
        const truths = answer.order.map((id) => task.items.find((item) => item.id === id)!.value);
        const given = answer.order.map((id) => (Object.hasOwn(guesses, id) ? guesses[id] : undefined));
        expect(result.items.map((item) => item.miss)).toEqual(oracleMisses(truths, given, scale));
        expect(result.items.map((item) => item.guess)).toEqual(given);
        expect(Math.abs(result.score - oracleGuessed(truths, given, scale, false))).toBeLessThan(TOLERANCE);
        expect(result.score).toBeGreaterThanOrEqual(0);
        expect(result.score).toBeLessThanOrEqual(1);
      }
    }
  });

  it("never misses on a linear scale and misses beyond the factor 1000 on a logarithmic one when the values do not spread", () => {
    const flat = sortingTask([5, 5, 5]);
    expect((scoreTask(flat, sheetTaskOf(flat, 1, "hard"), guessedSorting(flat, { i0: -1e9, i1: 5, i2: 1e9 })) as SortingTaskResult).score).toBe(1);
    const unguessed = scoreTask(flat, sheetTaskOf(flat, 1, "expert"), guessedSorting(flat, { i0: 5, i1: 5 })) as SortingTaskResult;
    expect(unguessed.items.map((item) => item.miss)).toEqual([false, false, true]);
    expect(unguessed.score).toBe(0);
    const level = sortingTask([5, 5, 5], "logarithmic");
    expect((scoreTask(level, sheetTaskOf(level, 1, "hard"), guessedSorting(level, { i0: 0.006, i1: 5, i2: 4000 })) as SortingTaskResult).score).toBe(1);
    const off = scoreTask(level, sheetTaskOf(level, 1, "hard"), guessedSorting(level, { i0: 5, i1: 5, i2: 5e4 })) as SortingTaskResult;
    expect(off.items.map((item) => item.miss)).toEqual([false, false, true]);
    expect(off.score).toBe(0);
  });

  it("counts an item without a guess as a miss on a timed sheet and leaves it without a guess in the result", () => {
    const sheetTask = sheetTaskOf(decades, 1, "expert");
    const result = scoreTask(decades, sheetTask, guessedSorting(decades, { i0: 1, i1: 1e3, i3: 1e9 })) as SortingTaskResult;
    expect(result.items).toEqual([
      { item: "i0", value: 1, position: 0, rank: 0, guess: 1, miss: false, explanation: T("first") },
      { item: "i1", value: 1e3, position: 1, rank: 1, guess: 1e3, miss: false },
      { item: "i3", value: 1e9, position: 2, rank: 3, guess: 1e9, miss: false },
      { item: "i2", value: 1e6, position: 3, rank: 2, miss: true },
    ]);
    expect(Math.abs(result.score - (1 - (6 + 3 + 3) / 30))).toBeLessThan(TOLERANCE);
    expect(scoreTask(decades, sheetTaskOf(decades, 1, "hard"), guessedSorting(decades, { i0: 1, i1: 1e3, i3: 1e9 }))).toBeUndefined();
  });

  it("scores a timed task without an answer 0, its items in sheet order and all a miss; an untimed one stays unscored", () => {
    const sheetTask = sheetTaskOf(decades, 5, "expert");
    const result = scoreTask(decades, sheetTask) as SortingTaskResult;
    expect(result.score).toBe(0);
    expect(result.items.map((item) => item.item)).toEqual(sheetTask.items.map((item) => item.id));
    expect(result.items.map((item) => item.position)).toEqual([0, 1, 2, 3]);
    expect(result.items.every((item) => item.miss === true && !Object.hasOwn(item, "guess"))).toBe(true);
    expect((scoreTask(decades, sheetTask, { kind: "sorting", order: sheetTask.items.map((item) => item.id) }) as SortingTaskResult).score).toBe(0);
    for (const challenge of ["easy", "medium", "hard"] as const) expect(scoreTask(decades, sheetTaskOf(decades, 5, challenge))).toBeUndefined();
    const flat = sortingTask([5, 5]);
    expect((scoreTask(flat, sheetTaskOf(flat, 1, "expert")) as SortingTaskResult).score).toBe(0);
  });

  it("scores a sheet task that shows the keys and is timed — never dealt today — 0 without an answer and by the order with one", () => {
    const sheetTask = { ...sheetTaskOf(decades, 5, "medium"), seconds: 60 };
    const unanswered = scoreTask(decades, sheetTask) as SortingTaskResult;
    expect(unanswered.score).toBe(0);
    expect(unanswered.items.map((item) => item.item)).toEqual(sheetTask.items.map((item) => item.id));
    expect(unanswered.items.some((item) => Object.hasOwn(item, "miss") || Object.hasOwn(item, "guess"))).toBe(false);
    expect((scoreTask(decades, sheetTask, { kind: "sorting", order: ["i0", "i1", "i2", "i3"] }) as SortingTaskResult).score).toBe(1);
  });
});

/** 🧾️ A matching answer made of guesses per dimension and item. */
function guessedMatching(guesses: Readonly<Record<string, Readonly<Record<string, number>>>>): Extract<Answer, { kind: "matching" }> {
  return { kind: "matching", guesses };
}

describe("matching — guesses where the keys are hidden", () => {
  const task = matchingTask({ "u-value": { scale: "linear", values: [1.4, 0.5, 0.24, 0.12] }, demand: { scale: "logarithmic", values: [250, 120, 70, 15] } });
  const truths = (dimension: string) => Object.fromEntries(task.items.map((item) => [item.id, item.values[dimension]!]));

  it("scores 1 for the true values and for guesses near them, with the guess as the assigned value and a miss mark on every item", () => {
    for (const challenge of ["hard", "expert"] as const) {
      for (let seed = 0; seed < 10; seed++) {
        const sheetTask = sheetTaskOf(task, seed, challenge);
        const exact = scoreTask(task, sheetTask, guessedMatching({ "u-value": truths("u-value"), demand: truths("demand") })) as MatchingTaskResult;
        expect(exact.score).toBe(1);
        expect(exact.dimensions.map((dimension) => dimension.dimension)).toEqual(["u-value", "demand"]);
        expect(exact.dimensions[1]!.items).toEqual(sheetTask.items.map((item) => ({ item: item.id, assigned: truths("demand")[item.id], correct: truths("demand")[item.id], miss: false })));
        const near = scoreTask(task, sheetTask, guessedMatching({ "u-value": { m0: 1.2, m1: 0.6, m2: 0.3, m3: 0.1 }, demand: { m0: 300, m1: 100, m2: 60, m3: 20 } })) as MatchingTaskResult;
        expect(near.score).toBe(1);
        expect(Object.keys(near.dimensions[0]!.items[0]!)).toEqual(["item", "assigned", "correct", "miss"]);
      }
    }
  });

  it("counts a guess beyond the reach as a miss that costs every pair it touches", () => {
    const single = matchingTask({ size: { scale: "linear", values: [0, 10, 20] } });
    const sheetTask = sheetTaskOf(single, 2, "hard");
    const result = scoreTask(single, sheetTask, guessedMatching({ size: { m0: 0, m1: 10, m2: 31 } })) as MatchingTaskResult;
    const items = Object.fromEntries(result.dimensions[0]!.items.map((item) => [item.item, item]));
    expect(items.m2).toEqual({ item: "m2", assigned: 31, correct: 20, miss: true });
    expect(items.m0!.miss).toBe(false);
    expect(result.dimensions[0]!.score).toBe(1 - 30 / 40);
    expect((scoreTask(single, sheetTask, guessedMatching({ size: { m0: 0, m1: 10, m2: 30 } })) as MatchingTaskResult).score).toBe(1);
  });

  it("counts equal guesses on distinct true values half, and the opposite order whole", () => {
    const single = matchingTask({ size: { scale: "linear", values: [1, 2, 3] } });
    const sheetTask = sheetTaskOf(single, 2, "hard");
    const tied = scoreTask(single, sheetTask, guessedMatching({ size: { m0: 2, m1: 2, m2: 2 } })) as MatchingTaskResult;
    expect(tied.dimensions[0]!.items.every((item) => item.miss === false)).toBe(true);
    expect(tied.dimensions[0]!.score).toBe(0.5);
    const swapped = scoreTask(single, sheetTask, guessedMatching({ size: { m0: 2, m1: 1.9, m2: 2.1 } })) as MatchingTaskResult;
    expect(swapped.dimensions[0]!.items.every((item) => item.miss === false)).toBe(true);
    expect(swapped.dimensions[0]!.score).toBe(1 - 1 / 4);
  });

  it("scores like the sorting the guesses induce when no guess misses and none ties", () => {
    const random = new Mt19937(8);
    let swaps = 0;
    for (let round = 0; round < 200; round++) {
      const values = shuffle(
        random,
        Array.from({ length: 3 + (round % 6) }, (_, rank) => 10 ** (rank * 0.7 + (random.next() % 100) / 1000)),
      );
      const single = matchingTask({ power: { scale: "logarithmic", values } });
      const sheetTask = sheetTaskOf(single, round, "hard");
      const guesses = Object.fromEntries(single.items.map((item) => [item.id, item.values.power! * 10 ** (((random.next() % 1001) - 500) / 1000)]));
      const matched = scoreTask(single, sheetTask, guessedMatching({ power: guesses })) as MatchingTaskResult;
      expect(matched.dimensions[0]!.items.every((item) => item.miss === false)).toBe(true);
      const induced = [...single.items].sort((left, right) => guesses[left.id]! - guesses[right.id]!).map((item) => `i${item.id.slice(1)}`);
      const sorting = sortingTask(values, "logarithmic");
      if (induced.join() !== ascendingIds(sorting).join()) swaps++;
      expect(Math.abs(matched.score - sortingScore(sorting, induced))).toBeLessThan(TOLERANCE);
    }
    expect(swaps).toBeGreaterThan(20);
  });

  it("agrees with the formula written over mathjs, misses, ties and missing guesses included", () => {
    const random = new Mt19937(90210);
    for (const scale of ["linear", "logarithmic"] as const) {
      for (let round = 0; round < 300; round++) {
        const values = randomValues(random, 2 + (round % 7), scale);
        const single = matchingTask({ size: { scale, values } });
        const partial = round % 3 === 0;
        const sheetTask = sheetTaskOf(single, round, partial ? "expert" : "hard");
        const guesses = Object.fromEntries(
          single.items.flatMap((item, index) => {
            if (partial && random.next() % 4 === 0) return [];
            return [[item.id, random.next() % 5 === 0 ? values[(index + 1) % values.length]! : randomGuess(random, item.values.size!, scale)] as const];
          }),
        );
        const result = scoreTask(single, sheetTask, guessedMatching({ size: guesses })) as MatchingTaskResult;
        const order = sheetTask.items.map((item) => item.id);
        const truth = order.map((id) => single.items.find((item) => item.id === id)!.values.size!);
        const given = order.map((id) => (Object.hasOwn(guesses, id) ? guesses[id] : undefined));
        expect(result.dimensions[0]!.items.map((item) => item.miss)).toEqual(oracleMisses(truth, given, scale));
        expect(result.dimensions[0]!.items.map((item) => item.assigned)).toEqual(given);
        expect(Math.abs(result.score - oracleGuessed(truth, given, scale, true))).toBeLessThan(TOLERANCE);
      }
    }
  });

  it("leaves an unguessed item without an assigned value on a timed sheet and counts it as a miss; an untimed sheet stays unscored", () => {
    const single = matchingTask({ size: { scale: "linear", values: [0, 10, 20] } });
    const sheetTask = sheetTaskOf(single, 2, "expert");
    const result = scoreTask(single, sheetTask, guessedMatching({ size: { m0: 0, m2: 20 } })) as MatchingTaskResult;
    const items = Object.fromEntries(result.dimensions[0]!.items.map((item) => [item.item, item]));
    expect(items.m1).toEqual({ item: "m1", correct: 10, miss: true });
    expect(items.m0).toEqual({ item: "m0", assigned: 0, correct: 0, miss: false });
    expect(result.dimensions[0]!.score).toBe(1 - 20 / 40);
    expect(scoreTask(single, sheetTaskOf(single, 2, "hard"), guessedMatching({ size: { m0: 0, m2: 20 } }))).toBeUndefined();
    for (const answer of [undefined, guessedMatching({}), { kind: "matching" } as Answer]) {
      const empty = scoreTask(single, sheetTask, answer) as MatchingTaskResult;
      expect(empty.score).toBe(0);
      expect(empty.dimensions[0]!.items).toEqual(sheetTask.items.map((item) => ({ item: item.id, correct: single.items.find((candidate) => candidate.id === item.id)!.values.size, miss: true })));
    }
    expect(scoreTask(single, sheetTaskOf(single, 2, "hard"))).toBeUndefined();
  });

  it("scores a dimension whose true values do not spread 1 when every item is guessed and 0 when one is not", () => {
    const flat = matchingTask({ size: { scale: "linear", values: [7, 7, 7] } });
    expect((scoreTask(flat, sheetTaskOf(flat, 1, "hard"), guessedMatching({ size: { m0: -100, m1: 7, m2: 1e6 } })) as MatchingTaskResult).score).toBe(1);
    expect((scoreTask(flat, sheetTaskOf(flat, 1, "expert"), guessedMatching({ size: { m0: 7, m1: 7 } })) as MatchingTaskResult).score).toBe(0);
  });

  it("scores no answer of the other shape: card assignments where the keys are hidden, guesses where they show", () => {
    const single = matchingTask({ size: { scale: "linear", values: [0, 10, 20] } });
    for (const challenge of ["hard", "expert"] as const) expect(scoreTask(single, sheetTaskOf(single, 2, challenge), { kind: "matching", assignments: { size: { m0: 0, m1: 1, m2: 2 } } })).toBeUndefined();
    for (const challenge of ["easy", "medium"] as const) expect(scoreTask(single, sheetTaskOf(single, 2, challenge), guessedMatching({ size: { m0: 0, m1: 10, m2: 20 } }))).toBeUndefined();
  });

  it("counts an unassigned item of a sheet task that shows the keys and is timed — never dealt today — as a miss without a miss mark", () => {
    const single = matchingTask({ size: { scale: "linear", values: [0, 10, 20] } });
    const medium = sheetTaskOf(single, 2, "medium");
    if (medium.kind !== "matching") throw new Error("kind");
    const sheetTask = { ...medium, seconds: 60 };
    const cards = medium.dimensions[0]!.cards!;
    const result = scoreTask(single, sheetTask, { kind: "matching", assignments: { size: { m0: cards.indexOf(0), m2: cards.indexOf(20) } } }) as MatchingTaskResult;
    const items = Object.fromEntries(result.dimensions[0]!.items.map((item) => [item.item, item]));
    expect(items.m1).toEqual({ item: "m1", correct: 10 });
    expect(items.m2).toEqual({ item: "m2", assigned: 20, correct: 20 });
    expect(result.dimensions[0]!.score).toBe(1 - 20 / 40);
    expect((scoreTask(single, sheetTask) as MatchingTaskResult).score).toBe(0);
  });
});

describe("classification — unanswered items on a timed sheet", () => {
  const task: ClassificationTask = {
    kind: "classification",
    id: "classification",
    title: T("C"),
    prompt: T("C"),
    categories: [
      { id: "x", label: T("x") },
      { id: "y", label: T("y") },
    ],
    items: [
      { id: "p", label: T("p"), category: "x", explanation: T("why") },
      { id: "q", label: T("q"), category: "y" },
      { id: "r", label: T("r"), category: "y" },
    ],
  };

  it("gives an unassigned item no credit and no assigned category, and scores a task without an answer 0", () => {
    const sheetTask = sheetTaskOf(task, 4, "expert");
    const partial = scoreTask(task, sheetTask, { kind: "classification", assignments: { q: "y" } }) as ClassificationTaskResult;
    expect(partial.items.map((item) => item.item)).toEqual(sheetTask.items.map((item) => item.id));
    expect(partial.items.find((item) => item.item === "p")).toEqual({ item: "p", correct: "x", credit: 0, explanation: T("why") });
    expect(partial.items.find((item) => item.item === "q")).toEqual({ item: "q", assigned: "y", correct: "y", credit: 1 });
    expect(partial.score).toBe(1 / 3);
    const nothing = scoreTask(task, sheetTask) as ClassificationTaskResult;
    expect(nothing.score).toBe(0);
    expect(nothing.items.every((item) => item.credit === 0 && !Object.hasOwn(item, "assigned"))).toBe(true);
  });

  it("stays unscored on every untimed sheet until every item is assigned, and scores alike at every challenge then", () => {
    const whole: Answer = { kind: "classification", assignments: { p: "x", q: "x", r: "y" } };
    const medium = scoreTask(task, sheetTaskOf(task, 4, "medium"), whole);
    for (const challenge of CHALLENGES) expect(scoreTask(task, sheetTaskOf(task, 4, challenge), whole)).toEqual(medium);
    for (const challenge of ["easy", "medium", "hard"] as const) {
      expect(scoreTask(task, sheetTaskOf(task, 4, challenge), { kind: "classification", assignments: { q: "y" } })).toBeUndefined();
      expect(scoreTask(task, sheetTaskOf(task, 4, challenge))).toBeUndefined();
    }
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
    const sheet = sheetOf(quiz, 99, "medium");
    const result = scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] }, classification: { kind: "classification", assignments: { p: "x", q: "x" } } });
    if (!result) throw new Error("unscored");
    expect(result.quiz).toBe("run");
    expect(result.tasks.map((task) => task.task)).toEqual(sheet.tasks.map((task) => task.id));
    expect(result.score).toBe(0.75);
    expect(Object.keys(result)).toEqual(["quiz", "challenge", "score", "points", "tasks"]);
  });

  it("names the challenge of the sheet and earns score × par points: more for the same accuracy on a harder challenge", () => {
    const answers = (challenge: Challenge): Record<string, Answer> => ({
      sorting: CHALLENGE_RULES[challenge].keys ? { kind: "sorting", order: ["i0", "i1", "i2"] } : { kind: "sorting", order: ["i0", "i1", "i2"], guesses: { i0: 1, i1: 2, i2: 3 } },
      classification: { kind: "classification", assignments: { p: "x", q: "x" } },
    });
    const earned = CHALLENGES.map((challenge) => {
      const result = scoreRun(quiz, sheetOf(quiz, 99, challenge), answers(challenge));
      if (!result) throw new Error("unscored");
      expect(result.challenge).toBe(challenge);
      expect(result.score).toBe(0.75);
      expect(result.points).toBe(0.75 * CHALLENGE_RULES[challenge].par);
      return result.points;
    });
    expect(earned).toEqual([75, 150, 225, 300]);
  });

  it("scores a timed sheet as it stands: a task without an answer or with an incomplete one counts, what is missing as a miss", () => {
    const sheet = sheetOf(quiz, 99, "expert");
    const nothing = scoreRun(quiz, sheet, {});
    if (!nothing) throw new Error("unscored");
    expect(nothing).toMatchObject({ quiz: "run", challenge: "expert", score: 0, points: 0 });
    expect(nothing.tasks.map((task) => task.task)).toEqual(sheet.tasks.map((task) => task.id));
    const sortingOnly = scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"], guesses: { i0: 1, i1: 2, i2: 3 } } });
    expect(sortingOnly).toMatchObject({ score: 0.5, points: 200 });
    const half = scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"], guesses: { i0: 1, i1: 2, i2: 3 } }, classification: { kind: "classification", assignments: { p: "x" } } });
    expect(half).toMatchObject({ score: 0.75, points: 300 });
    const classified = half!.tasks.find((task) => task.kind === "classification") as ClassificationTaskResult;
    expect(classified.items.find((item) => item.item === "p")).toEqual({ item: "p", assigned: "x", correct: "x", credit: 1 });
    expect(classified.items.find((item) => item.item === "q")).toEqual({ item: "q", correct: "y", credit: 0 });
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1"] } })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"], guesses: { i0: 3, i1: 2 } } })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { classification: { kind: "classification", assignments: { p: "z" } } })).toBeUndefined();
  });

  it("scores nothing on an untimed sheet that hides the keys unless every item has a guess", () => {
    const sheet = sheetOf(quiz, 99, "hard");
    const units: Answer = { kind: "classification", assignments: { p: "x", q: "y" } };
    expect(scoreRun(quiz, sheet, { classification: units })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"] }, classification: units })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"], guesses: { i0: 1, i1: 2 } }, classification: units })).toBeUndefined();
    expect(scoreRun(quiz, sheet, { sorting: { kind: "sorting", order: ["i0", "i1", "i2"], guesses: { i0: 1, i1: 2, i2: 3 } }, classification: units })).toMatchObject({ score: 1, points: 300 });
  });

  it("scores nothing unless every sheet task has a valid, complete answer", () => {
    const sheet = sheetOf(quiz, 99, "medium");
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
    for (const challenge of CHALLENGES) expect(scoreRun(empty, sheetOf(empty, 1, challenge), {})).toEqual({ quiz: "empty", challenge, score: 0, points: 0, tasks: [] });
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
    const cards = sheetTask.dimensions[0]!.cards!;
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
