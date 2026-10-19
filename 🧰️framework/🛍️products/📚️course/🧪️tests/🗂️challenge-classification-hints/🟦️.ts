/** 🗂️ The hints of an easy run on a classification task (`hintsOf`): profile hints, pairs and categories, and how they agree with the same rules over mathjs. */

import { abs as mathAbs, add as mathAdd, divide as mathDivide, max as mathMax, min as mathMin, multiply as mathMultiply, sign as mathSign, subtract as mathSubtract } from "mathjs";
import { describe, expect, it } from "vitest";
import { CHALLENGES, Mt19937, hintsOf, type Answer, type ClassificationTask, type Hint, type SheetClassificationTask } from "../../📦️packages/🟦️typescript/🟦️.ts";
import { CLASSIFICATION, classificationSheet, oracleCapped, type OracleWeighted, sheetTaskOf, T } from "../🧱️challenge-ladder-kit/🟦️.ts";

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
