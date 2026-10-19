/** 💡️ The hints of an easy run on a sorting task (`hintsOf`): which relations are questioned, in which order, and how they agree with the same rule over mathjs. */

import { describe, expect, it } from "vitest";
import { HINTS_PER_TASK, Mt19937, hintsOf, shuffle, verdictOf, type Answer, type SortingTask } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { CLASSIFICATION, oracleCapped, oracleCompare, type OracleKeyed, oracleMisses, oracleReach, oracleVerdict, sheetTaskOf, sortingSheet, sortingTask } from "../🧱️challenge-ladder-kit/🟦️.ts";

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
