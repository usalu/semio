/** 🧱️ Shared builders of the challenge-ladder suites: the sheet tasks, the classification task and the independent oracle formulas (reach, misses, verdict, compare, cap) over mathjs. */

import { abs as mathAbs, add as mathAdd, divide as mathDivide, max as mathMax, min as mathMin, multiply as mathMultiply, sign as mathSign, sqrt as mathSqrt, subtract as mathSubtract } from "mathjs";
import { sheetOf, type Challenge, type ClassificationTask, type Hint, type MatchingTask, type Quiz, type Scale, type SheetClassificationTask, type SheetMatchingTask, type SheetSortingTask, type SheetTask, type SortingTask, type Task, type Text, type Verdict } from "../../📦️packages/🟦️typescript/🟦️.ts";

export const T = (en: string): Text => ({ en, de: en });

/** 🃏️ The sheet task of a single-task quiz at a challenge. */
export function sheetTaskOf(task: Task, challenge: Challenge, seed = 1): SheetTask {
  const quiz: Quiz = { schema: "semio.quiz/v1", id: "quiz", emoji: "❓", title: T("Quiz"), description: T("Quiz"), tasks: [task] };
  return sheetOf(quiz, seed, challenge).tasks[0]!;
}

/** 📶️ A sorting task over the given values, item `i<index>` carrying `values[index]`, familiar where its index is in `familiar`. */
export function sortingTask(values: readonly number[], scale: Scale, familiar: readonly number[] = []): SortingTask {
  return { kind: "sorting", id: "sorting", title: T("Sorting"), prompt: T("Sort"), quantity: { label: T("Q"), unit: "u", scale, prefixed: false, additive: false }, items: values.map((value, index) => ({ id: `i${index}`, label: T(`i${index}`), value, ...(familiar.includes(index) ? { familiar: true } : {}) })) };
}

/** 🔗️ A matching task over items `m<index>` with the given values per dimension. */
export function matchingTask(dimensions: Readonly<Record<string, { readonly scale: Scale; readonly values: readonly number[] }>>): MatchingTask {
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

export const CLASSIFICATION: ClassificationTask = {
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
export function oracleReach(values: readonly number[], scale: Scale): number {
  const [lo, hi] = values.length === 0 ? [Infinity, -Infinity] : [mathMin([...values]) as number, mathMax([...values]) as number];
  if (scale === "linear") return hi > lo ? (mathDivide(mathSubtract(hi, lo), 2) as number) : Infinity;
  return hi > lo ? (mathMin(1000, mathSqrt(mathDivide(hi, lo) as number) as number) as number) : 1000;
}

/** 🎯️ Whether a value misses its truth, by mathjs: the ratio of the larger to the smaller beyond a logarithmic reach, the distance beyond a linear one, either reach widened by the slack 1e-9. */
export function oracleMisses(value: number, truth: number, scale: Scale, within: number): boolean {
  return (scale === "linear" ? (mathAbs(mathSubtract(value, truth)) as number) : (mathDivide(mathMax(value, truth), mathMin(value, truth)) as number)) > (mathMultiply(within, mathAdd(1, 1e-9)) as number);
}

/** 🔑️ An item of a numeric task as the oracle sees it: its assigned key, its true value, whether the key misses and whether it is familiar. */
export type OracleKeyed = { readonly item: string; readonly key: number; readonly value: number; readonly miss: boolean; readonly familiar: boolean };

/** 🧷️ A reference the oracle may name for a compare hint, with its claimed and true relation, its oriented claim and its place in sheet order. */
type OracleRelated = { readonly other: string; readonly familiar: boolean; readonly claim: number; readonly truth: number; readonly oriented: number; readonly index: number };

/** 🏋️ A hint the oracle made, or a compare hint with the references tied for its largest error, with the weight it competes for the cap with (none for group and category hints). */
export type OracleWeighted = { readonly hint: Hint; readonly weight?: number } | { readonly item: string; readonly dimension?: string; readonly linear: boolean; readonly tied: readonly OracleRelated[]; readonly weight: number };

/** 🪞️ The verdict by signs, by mathjs: reversed when the claim sits off the pivot and the truth does not sit on the claim's side of it, else under when the truth lies beyond the claim away from the pivot (above it for a claim at the pivot), else over. */
export function oracleVerdict(claim: number, truth: number, pivot: number): Verdict {
  const side = mathSign(mathSubtract(claim, pivot)) as number;
  if (side !== 0 && (mathSign(mathSubtract(truth, pivot)) as number) !== side) return "reversed";
  return (mathSign(mathSubtract(truth, claim)) as number) === (side === 0 ? 1 : side) ? "under" : "over";
}

/** 🤨️ The compare hint of a missed item by mathjs, before its reference is chosen, weighted by the largest error: the pool is the other keyed items that do not miss, or all of them when every one misses; each is rated by the error between the claimed and the true relation (ratio of ratios, distance of differences); errors within the slack 1e-9 of the largest tie. */
export function oracleCompare(hinted: OracleKeyed, keyed: readonly OracleKeyed[], scale: Scale, dimension?: string): OracleWeighted[] {
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
export function oracleCapped(weighted: readonly OracleWeighted[]): Hint[] {
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
export function sortingSheet(task: SortingTask, ids: readonly string[]): SheetSortingTask {
  const items = ids.map((id) => task.items.find((item) => item.id === id)!);
  return { kind: "sorting", id: task.id, title: task.title, prompt: task.prompt, quantity: task.quantity, keys: items.map((item) => item.value).sort((left, right) => left - right), items: items.map(({ id, label }) => ({ id, label })) };
}

/** 🀄️ A matching sheet task presenting the items of `task` named by `ids` in that order, each dimension with the given cards. */
export function matchingSheet(task: MatchingTask, ids: readonly string[], cards: Readonly<Record<string, readonly number[]>>): SheetMatchingTask {
  return { kind: "matching", id: task.id, title: task.title, prompt: task.prompt, dimensions: task.dimensions.map((dimension) => ({ ...dimension, cards: cards[dimension.id]! })), items: ids.map((id) => ({ id, label: T(id) })) };
}

/** 🗄️ A classification sheet task presenting the categories named by `categories` and the items named by `items` of `task`, in those orders. */
export function classificationSheet(task: ClassificationTask, items: readonly string[], categories: readonly string[] = task.categories.map((category) => category.id)): SheetClassificationTask {
  return { kind: "classification", id: task.id, title: task.title, prompt: task.prompt, ...(task.axes ? { axes: task.axes } : {}), categories: categories.map((id) => task.categories.find((category) => category.id === id)!), items: items.map((id) => ({ id, label: T(id) })) };
}
