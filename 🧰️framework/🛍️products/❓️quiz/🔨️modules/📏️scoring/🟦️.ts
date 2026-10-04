/** 📏️ Partial-credit scoring of answers: magnitude-weighted pair concordance for sortings and matchings, profile similarity for classifications.
 *
 * Every task scores in [0, 1] and the score means accuracy at every challenge; a run scores the mean of its task scores
 * in sheet order and earns points = score × par of its challenge. Pair weights are the distance of the true values on
 * the quantity's scale, so swapping near neighbours costs little and swapping a tea light with a power plant costs a
 * lot. Where the sheet hides the keys the answer is the learner's guesses: a pair also counts as discordant when either
 * of its items misses (no guess, or one beyond the reach of the presented values), so the right order alone earns
 * nothing and exactness is not required either. On a timed sheet a task may lack an answer or hold an incomplete one;
 * what is missing scores as a miss. Sums run in the normative order so both cores agree to the last bit up to `log10`.
 * Inputs that bypass validation never throw and never yield NaN scores: an answer that is invalid for its sheet task
 * (or incomplete on an untimed sheet), or a task that cannot be resolved, scores `undefined` — exactly where the Rust
 * twin returns `None`.
 *
 * @see ../../README.md — the scoring formulas and why they punish large misorders more than small ones
 * @see ../⛰️challenge/🟦️.ts — `reach`, `misses` and `points`
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
  SheetSortingTask,
  SheetTask,
  SortingAnswer,
  SortingTask,
  SortingTaskResult,
  Task,
  TaskResult,
  Text,
} from "../../🧬️schema/🟦️.ts";
import { misses, points, reach } from "../⛰️challenge/🟦️.ts";
import { ascendingItems } from "../🃏️sheet/🟦️.ts";
import { answerComplete, answerRejection } from "../✅️validation/🟦️.ts";

/** 📐️ A value on its scale: itself when linear, its decimal logarithm when logarithmic. */
export function scaled(value: number, scale: Scale): number {
  return scale === "logarithmic" ? Math.log10(value) : value;
}

/** ⚖️ `1 − discordant / total`; when no pair carries weight, 0 if any item misses and 1 otherwise. */
function concordance(total: number, discordant: number, missed: readonly boolean[]): number {
  return total > 0 ? 1 - discordant / total : missed.includes(true) ? 0 : 1;
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

/** 🗝️ The entry of `record` under `key`, when it holds one. */
function entry<T>(record: Readonly<Record<string, T>> | undefined, key: string): T | undefined {
  return record && Object.hasOwn(record, key) ? record[key] : undefined;
}

/** 📶️ Magnitude-weighted pair concordance of the learner's order: for i < j, weight |s(vᵢ) − s(vⱼ)| is discordant when vᵢ > vⱼ or when either item misses. Where the sheet task hides the keys an item misses without a guess or with a guess beyond the reach of the presented values; without an answer (timed sheets only) the items stand in sheet order and all miss. */
function scoreSorting(task: SortingTask, sheetTask: SheetSortingTask, answer: SortingAnswer | undefined): SortingTaskResult | undefined {
  const order = every(answer ? answer.order : sheetTask.items.map((item) => item.id), (id) => task.items.find((item) => item.id === id));
  if (!order) return undefined;
  const scale = task.quantity.scale;
  const hidden = sheetTask.keys === undefined;
  const within = hidden
    ? reach(
        order.map((item) => item.value),
        scale,
      )
    : Infinity;
  const guesses = order.map((item) => (hidden ? entry(answer?.guesses, item.id) : undefined));
  const missed = order.map((item, index) => (hidden ? guesses[index] === undefined || misses(guesses[index]!, item.value, scale, within) : answer === undefined));
  const values = order.map((item) => scaled(item.value, scale));
  let total = 0;
  let discordant = 0;
  for (let i = 0; i < order.length; i++) {
    for (let j = i + 1; j < order.length; j++) {
      const weight = Math.abs(values[i]! - values[j]!);
      total += weight;
      if (order[i]!.value > order[j]!.value || missed[i]! || missed[j]!) discordant += weight;
    }
  }
  const ranks = new Map(ascendingItems(task, order).map((item, rank) => [item.id, rank]));
  const items = order.map((item, position) => ({
    item: item.id,
    value: item.value,
    position,
    rank: ranks.get(item.id)!,
    ...(guesses[position] !== undefined ? { guess: guesses[position]! } : {}),
    ...(hidden ? { miss: missed[position]! } : {}),
    ...explained(item.explanation),
  }));
  return { kind: "sorting", task: task.id, score: concordance(total, discordant, missed), items };
}

/** 🔗️ Per presented dimension, items in sheet order: weight |s(tᵢ) − s(tⱼ)| is discordant when either item misses or the assigned values order the pair the other way, half discordant when they tie on distinct true values. The assigned value is the card where the sheet task shows the keys and the guess where it hides them; an item misses without one, or with a guess beyond the reach of the presented true values. */
function scoreMatching(task: MatchingTask, sheetTask: SheetMatchingTask, answer: MatchingAnswer | undefined): MatchingTaskResult | undefined {
  const items = every(sheetTask.items, (presented) => task.items.find((item) => item.id === presented.id));
  if (!items) return undefined;
  const dimensions = every(sheetTask.dimensions, (presented): DimensionResult | undefined => {
    const scale = task.dimensions.find((dimension) => dimension.id === presented.id)?.quantity.scale;
    const truth = every(items, (item) => entry(item.values, presented.id));
    if (scale === undefined || !truth) return undefined;
    const cards = presented.cards;
    const given = entry(cards ? answer?.assignments : answer?.guesses, presented.id);
    const assigned = items.map((item) => {
      const value = entry(given, item.id);
      return cards && value !== undefined ? cards[value] : value;
    });
    const within = cards ? Infinity : reach(truth, scale);
    const missed = items.map((_, index) => assigned[index] === undefined || misses(assigned[index]!, truth[index]!, scale, within));
    let total = 0;
    let discordant = 0;
    for (let i = 0; i < items.length; i++) {
      for (let j = i + 1; j < items.length; j++) {
        const weight = Math.abs(scaled(truth[i]!, scale) - scaled(truth[j]!, scale));
        total += weight;
        if (missed[i]! || missed[j]!) discordant += weight;
        else if ((truth[i]! > truth[j]! && assigned[i]! < assigned[j]!) || (truth[i]! < truth[j]! && assigned[i]! > assigned[j]!)) discordant += weight;
        else if (assigned[i] === assigned[j] && truth[i] !== truth[j]) discordant += weight / 2;
      }
    }
    return {
      dimension: presented.id,
      score: concordance(total, discordant, missed),
      items: items.map((item, index) => ({
        item: item.id,
        ...(assigned[index] !== undefined ? { assigned: assigned[index]! } : {}),
        correct: truth[index]!,
        ...(cards ? {} : { miss: missed[index]! }),
        ...explained(item.explanation),
      })),
    };
  });
  if (!dimensions) return undefined;
  return { kind: "matching", task: task.id, score: mean(dimensions.map((dimension) => dimension.score)), dimensions };
}

/** 📍️ The Euclidean distance of two normalised profiles, summed in axis order. */
function distance(left: readonly number[], right: readonly number[]): number {
  let sum = 0;
  for (let k = 0; k < left.length; k++) {
    const delta = left[k]! - right[k]!;
    sum += delta * delta;
  }
  return Math.sqrt(sum);
}

/** 🗃️ Credit 1 for the correct category; a wrong one earns `max(0, 1 − d / d_max)` when both carry profiles complete on every axis, else 0; an item left unassigned (timed sheets only) earns 0 and carries no `assigned`. */
function scoreClassification(task: ClassificationTask, sheetTask: SheetClassificationTask, answer: ClassificationAnswer | undefined): ClassificationTaskResult | undefined {
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
    const assigned = entry(answer?.assignments, presented.id);
    if (!item) return undefined;
    if (assigned === undefined) return { item: item.id, correct: item.category, credit: 0, ...explained(item.explanation) };
    const left = profileOf(assigned);
    const right = profileOf(item.category);
    const credit = assigned === item.category ? 1 : left && right && farthest > 0 ? Math.max(0, 1 - distance(left, right) / farthest) : 0;
    return { item: item.id, assigned, correct: item.category, credit, ...explained(item.explanation) };
  });
  if (!items) return undefined;
  return { kind: "classification", task: task.id, score: mean(items.map((item) => item.credit)), items };
}

/** 🧮️ The result of one task, or `undefined` unless the sheet task presents it and the answer is valid and complete. A timed sheet task (one that carries `seconds`) is also scored without an answer or with an incomplete one: what is missing scores as a miss. */
export function scoreTask(task: Task, sheetTask: SheetTask, answer?: Answer): TaskResult | undefined {
  if (task.id !== sheetTask.id) return undefined;
  if (answer ? answerRejection(sheetTask, answer) !== undefined : sheetTask.seconds === undefined) return undefined;
  if (sheetTask.seconds === undefined && !answerComplete(sheetTask, answer)) return undefined;
  if (task.kind === "classification" && sheetTask.kind === "classification" && (!answer || answer.kind === "classification")) return scoreClassification(task, sheetTask, answer);
  if (task.kind === "sorting" && sheetTask.kind === "sorting" && (!answer || answer.kind === "sorting")) return scoreSorting(task, sheetTask, answer);
  if (task.kind === "matching" && sheetTask.kind === "matching" && (!answer || answer.kind === "matching")) return scoreMatching(task, sheetTask, answer);
  return undefined;
}

/** 🏁️ The result of a run at the challenge of its sheet — every sheet task scored in sheet order, their mean and the points it earns — or `undefined` unless every sheet task has a valid, complete answer; on a timed sheet a task may have no answer or an incomplete one. */
export function scoreRun(quiz: Quiz, sheet: Sheet, answers: Readonly<Record<string, Answer>>): RunResult | undefined {
  const tasks = every(sheet.tasks, (sheetTask) => {
    const task = quiz.tasks.find((candidate) => candidate.id === sheetTask.id);
    return task && scoreTask(task, sheetTask, entry(answers, sheetTask.id));
  });
  if (!tasks) return undefined;
  const score = mean(tasks.map((task) => task.score));
  return { quiz: sheet.quiz, challenge: sheet.challenge, score, points: points(score, sheet.challenge), tasks };
}
