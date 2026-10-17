/** 🧲️ The hints of an easy run on a matching task (`hintsOf`): per dimension and item, the assigned cards that miss, and how they agree with the same rule over mathjs. */

import { describe, expect, it } from "vitest";
import { Mt19937, hintsOf, type Answer, type Hint, type MatchingTask, type SheetMatchingTask, type SheetTask } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { matchingSheet, matchingTask, oracleCapped, oracleCompare, type OracleKeyed, oracleMisses, oracleReach, sheetTaskOf } from "../🧱️challenge-ladder-kit/🟦️.ts";

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
