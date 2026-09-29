/** 📏️ Partial-credit scoring of complete answers: magnitude-weighted pair concordance for sortings and matchings, profile similarity for classifications.
 *
 * Every task scores in [0, 1]; a run scores the mean of its task scores in sheet order. Pair weights are the distance
 * of the true values on the quantity's scale, so swapping near neighbours costs little and swapping a tea light with a
 * power plant costs a lot. Sums run in the normative order so both cores agree to the last bit up to `log10`. Inputs
 * that bypass validation never throw and never yield NaN scores: an answer that is invalid or incomplete for its sheet
 * task, or a task that cannot be resolved, scores `undefined` — exactly where the Rust twin returns `None`.
 *
 * @see ../../README.md — the scoring formulas and why they punish large misorders more than small ones
 * @see ./🦀️.rs — the Rust twin
 */
import type {
  Answer,
  Category,
  ClassificationAnswer,
  ClassificationTask,
  ClassificationTaskResult,
  DimensionResult,
  MatchingAnswer,
  MatchingTask,
  MatchingTaskResult,
  Quiz,
  RunResult,
  Scale,
  Sheet,
  SheetClassificationTask,
  SheetMatchingTask,
  SheetTask,
  SortingAnswer,
  SortingTask,
  SortingTaskResult,
  Task,
  TaskResult,
  Text,
} from "../../🧬️schema/🟦️.ts";
import { ascendingItems } from "../🃏️sheet/🟦️.ts";
import { answerComplete, answerRejection } from "../✅️validation/🟦️.ts";

/** 📐️ A value on its scale: itself when linear, its decimal logarithm when logarithmic. */
export function scaled(value: number, scale: Scale): number {
  return scale === "logarithmic" ? Math.log10(value) : value;
}

/** ⚖️ `1 − discordant / total`, or 1 when no pair carries weight. */
function concordance(total: number, discordant: number): number {
  return total > 0 ? 1 - discordant / total : 1;
}

/** ➗️ The mean of the given scores in their order; 0 for none. */
function mean(scores: readonly number[]): number {
  return scores.length === 0 ? 0 : scores.reduce((sum, score) => sum + score, 0) / scores.length;
}

/** 💬️ The optional explanation, spread into a result item. */
function explained(explanation: Text | undefined): { readonly explanation?: Text } {
  return explanation ? { explanation } : {};
}

/** 🧺️ Every mapped entry, or `undefined` as soon as one maps to `undefined` (Rust's `collect::<Option<_>>`). */
function every<T, U>(entries: readonly T[], map: (entry: T, index: number) => U | undefined): U[] | undefined {
  const mapped: U[] = [];
  for (const [index, entry] of entries.entries()) {
    const value = map(entry, index);
    if (value === undefined) return undefined;
    mapped.push(value);
  }
  return mapped;
}

/** 📶️ Magnitude-weighted pair concordance of the learner's order: for i < j, weight |s(vᵢ) − s(vⱼ)| is discordant when vᵢ > vⱼ. */
function scoreSorting(task: SortingTask, answer: SortingAnswer): SortingTaskResult | undefined {
  const order = every(answer.order, (id) => task.items.find((item) => item.id === id));
  if (!order) return undefined;
  const values = order.map((item) => scaled(item.value, task.quantity.scale));
  let total = 0;
  let discordant = 0;
  for (let i = 0; i < order.length; i++) {
    for (let j = i + 1; j < order.length; j++) {
      const weight = Math.abs(values[i]! - values[j]!);
      total += weight;
      if (order[i]!.value > order[j]!.value) discordant += weight;
    }
  }
  const ranks = new Map(ascendingItems(task, order).map((item, rank) => [item.id, rank]));
  const items = order.map((item, position) => ({ item: item.id, value: item.value, position, rank: ranks.get(item.id)!, ...explained(item.explanation) }));
  return { kind: "sorting", task: task.id, score: concordance(total, discordant), items };
}

/** 🔗️ Per presented dimension, items in sheet order: weight |s(tᵢ) − s(tⱼ)| is discordant when the assigned values order the pair the other way, half discordant when they tie on distinct true values. */
function scoreMatching(task: MatchingTask, sheetTask: SheetMatchingTask, answer: MatchingAnswer): MatchingTaskResult | undefined {
  const items = every(sheetTask.items, (presented) => task.items.find((item) => item.id === presented.id));
  if (!items) return undefined;
  const dimensions = every(sheetTask.dimensions, (presented): DimensionResult | undefined => {
    const scale = task.dimensions.find((dimension) => dimension.id === presented.id)?.quantity.scale;
    const assignments = Object.hasOwn(answer.assignments, presented.id) ? answer.assignments[presented.id] : undefined;
    if (scale === undefined || assignments === undefined) return undefined;
    const truth = every(items, (item) => (Object.hasOwn(item.values, presented.id) ? item.values[presented.id] : undefined));
    const assigned = every(items, (item) => (Object.hasOwn(assignments, item.id) ? presented.cards[assignments[item.id]!] : undefined));
    if (!truth || !assigned) return undefined;
    let total = 0;
    let discordant = 0;
    for (let i = 0; i < items.length; i++) {
      for (let j = i + 1; j < items.length; j++) {
        const weight = Math.abs(scaled(truth[i]!, scale) - scaled(truth[j]!, scale));
        total += weight;
        if ((truth[i]! > truth[j]! && assigned[i]! < assigned[j]!) || (truth[i]! < truth[j]! && assigned[i]! > assigned[j]!)) discordant += weight;
        else if (assigned[i] === assigned[j] && truth[i] !== truth[j]) discordant += weight / 2;
      }
    }
    return { dimension: presented.id, score: concordance(total, discordant), items: items.map((item, index) => ({ item: item.id, assigned: assigned[index]!, correct: truth[index]!, ...explained(item.explanation) })) };
  });
  if (!dimensions) return undefined;
  return { kind: "matching", task: task.id, score: mean(dimensions.map((dimension) => dimension.score)), dimensions };
}

/** 📏️ The Euclidean distance of two normalised profiles, summed in axis order. */
function distance(left: readonly number[], right: readonly number[]): number {
  let sum = 0;
  for (let k = 0; k < left.length; k++) {
    const delta = left[k]! - right[k]!;
    sum += delta * delta;
  }
  return Math.sqrt(sum);
}

/** 🗃️ Credit 1 for the correct category; a wrong one earns `max(0, 1 − d / d_max)` when both carry profiles complete on every axis, else 0. */
function scoreClassification(task: ClassificationTask, sheetTask: SheetClassificationTask, answer: ClassificationAnswer): ClassificationTaskResult | undefined {
  const axes = task.axes ?? [];
  const normalised = (category: Category): number[] | undefined => {
    const profile = category.profile;
    return profile && every(axes, (axis) => (Object.hasOwn(profile, axis.id) ? (profile[axis.id]! - axis.min) / (axis.max - axis.min) : undefined));
  };
  const profiles = task.categories.map(normalised);
  let farthest = 0;
  for (let i = 0; i < profiles.length; i++) {
    for (let j = i + 1; j < profiles.length; j++) {
      const left = profiles[i];
      const right = profiles[j];
      if (left && right) farthest = Math.max(farthest, distance(left, right));
    }
  }
  const profileOf = (id: string): number[] | undefined => profiles[task.categories.findIndex((category) => category.id === id)];
  const items = every(sheetTask.items, (presented) => {
    const item = task.items.find((candidate) => candidate.id === presented.id);
    const assigned = Object.hasOwn(answer.assignments, presented.id) ? answer.assignments[presented.id] : undefined;
    if (!item || assigned === undefined) return undefined;
    const left = profileOf(assigned);
    const right = profileOf(item.category);
    const credit = assigned === item.category ? 1 : left && right && farthest > 0 ? Math.max(0, 1 - distance(left, right) / farthest) : 0;
    return { item: item.id, assigned, correct: item.category, credit, ...explained(item.explanation) };
  });
  if (!items) return undefined;
  return { kind: "classification", task: task.id, score: mean(items.map((item) => item.credit)), items };
}

/** 🧮️ The result of one task, or `undefined` unless the sheet task presents it and the answer is valid and complete. */
export function scoreTask(task: Task, sheetTask: SheetTask, answer: Answer): TaskResult | undefined {
  if (task.id !== sheetTask.id || answerRejection(sheetTask, answer) !== undefined || !answerComplete(sheetTask, answer)) return undefined;
  if (task.kind === "classification" && sheetTask.kind === "classification" && answer.kind === "classification") return scoreClassification(task, sheetTask, answer);
  if (task.kind === "sorting" && sheetTask.kind === "sorting" && answer.kind === "sorting") return scoreSorting(task, answer);
  if (task.kind === "matching" && sheetTask.kind === "matching" && answer.kind === "matching") return scoreMatching(task, sheetTask, answer);
  return undefined;
}

/** 🏁️ The result of a run — every sheet task scored in sheet order and their mean — or `undefined` unless every sheet task has a valid, complete answer. */
export function scoreRun(quiz: Quiz, sheet: Sheet, answers: Readonly<Record<string, Answer>>): RunResult | undefined {
  const tasks = every(sheet.tasks, (sheetTask) => {
    const task = quiz.tasks.find((candidate) => candidate.id === sheetTask.id);
    return task && Object.hasOwn(answers, sheetTask.id) ? scoreTask(task, sheetTask, answers[sheetTask.id]!) : undefined;
  });
  return tasks && { quiz: sheet.quiz, score: mean(tasks.map((task) => task.score)), tasks };
}
