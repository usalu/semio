/** 🃏️ The sheet of a run: the randomized presentation of a quiz at a challenge, a pure function of (quiz, seed, challenge).
 *
 * The generator is consumed in the same order whatever the challenge, so one seed deals the same items in the same
 * order at every challenge; the challenge only decides what a task shows afterwards. Where it shows the keys a sorting
 * carries the ascending true values of its presented items (a ladder, never which item holds which), a matching its
 * cards, a classification its category descriptions and whole axes. Where it hides them the sheet carries none of
 * these numbers: no sorting keys, no cards, no descriptions, no axis numbers, and profiles only as shares of their axis
 * range. Where it is timed every task carries its seconds.
 *
 * @see ../🎲️randomness/🟦️.ts — the generator and shuffle every draw goes through
 * @see ../⛰️challenge/🟦️.ts — the rules of a challenge and the seconds of a task
 * @see ./🦀️.rs — the Rust twin
 */
import type { Axis, Category, Challenge, Icon, Mockup, QuestionPool, Quiz, SheetAxis, SheetItem, SheetTask, Sheet, ShortText, SortingItem, Task, Text } from "../../🧬️schema/🟦️.ts";
import { challengeRules, taskSeconds, type ChallengeRules } from "../⛰️challenge/🟦️.ts";
import { Mt19937, fnv1a32, shuffle, weightedSample, type RandomSource } from "../🎲️randomness/🟦️.ts";

/** 🪧️ The solution-free projection of an item: id, label, short label and icon. */
function sheetItem(item: { readonly id: string; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon; readonly mockup?: Mockup }): SheetItem {
  return { id: item.id, label: item.label, ...(item.mockup ? { mockup: item.mockup } : {}), ...shortOf(item), ...iconOf(item) };
}

/** 🩳️ The short label of an item, a category or an axis as the optional field every presentation of it carries. */
function shortOf(source: { readonly short?: ShortText }): { readonly short?: ShortText } {
  return source.short ? { short: source.short } : {};
}

/** 🖼️ The icon of a task, an item or a dimension as the optional field every presentation of it carries. */
export function iconOf(source: { readonly icon?: Icon }): { readonly icon?: Icon } {
  return source.icon ? { icon: source.icon } : {};
}

/** ✂️ The first `draw` items when `draw` is set and smaller than the item count, else all items. */
function drawn<T>(items: readonly T[], draw: number | undefined): readonly T[] {
  return draw !== undefined && draw < items.length ? items.slice(0, draw) : items;
}

/** 📏️ The magnitude that ranks an item in a distribution: a sorting item's absolute value, a matching item's absolute value on its first dimension, a classification item's definition position. */
function magnitude(task: Task, position: number): number {
  const item = task.items[position]!;
  if ("value" in item) return Math.abs(item.value);
  if ("values" in item && task.kind === "matching") return Math.abs(item.values[task.dimensions[0]!.id] ?? Number.POSITIVE_INFINITY);
  return position;
}

/** 📉️ The indices of the slots a sheet draws: all of them, or `task.draw` of them by the Zipf weight `1 / rank^exponent` of their rank by smallest magnitude, ties by slot order. */
function drawnSlots(task: Task, pool: QuestionPool, seed: number): readonly number[] {
  const { distribution, slots } = pool;
  if (!distribution) return slots.map((_, index) => index);
  const smallest = slots.map((slot) => Math.min(...slot.map((id) => magnitude(task, task.items.findIndex((item) => item.id === id)))));
  const ranked = slots.map((_, index) => index).sort((left, right) => smallest[left]! - smallest[right]! || left - right);
  const picked = weightedSample(new Mt19937((seed ^ fnv1a32(task.id + ":zipf")) >>> 0), ranked.map((_, rank) => Math.pow(rank + 1, -distribution.exponent)), task.draw!);
  return picked.map((rank) => ranked[rank]!);
}

/** 🎠️ One shuffled deck per drawn semantic slot, indexed by attempt; presentation order follows the run shuffle. */
function pooled<T extends { readonly id: string }>(task: Task, seed: number, shuffled: readonly T[]): readonly T[] {
  if (!task.pool) return drawn(shuffled, task.draw);
  const pool = task.pool;
  const selected = new Set(drawnSlots(task, pool, seed).map((index) => {
    const deck = shuffle(new Mt19937((seed >>> 16) ^ fnv1a32(task.id + ":" + index)), pool.slots[index]!);
    return deck[(seed & 65535) % deck.length]!;
  }));
  return shuffled.filter(item => selected.has(item.id));
}

/** 📶️ The ascending order of sorting items: by value, ties by definition index. */
export function ascendingItems(task: { readonly items: readonly SortingItem[] }, items: readonly SortingItem[]): SortingItem[] {
  const index = new Map(task.items.map((item, position) => [item.id, position]));
  return [...items].sort((left, right) => left.value - right.value || index.get(left.id)! - index.get(right.id)!);
}

/** 🛞️ An axis as a sheet presents it: whole where the keys show, its id, label and short label only where they are hidden. */
function sheetAxis(axis: Axis, keys: boolean): SheetAxis {
  return keys ? axis : { id: axis.id, label: axis.label, ...shortOf(axis) };
}

/** 🗂️ A category as a sheet presents it: whole where the keys show; where they are hidden without its description and with every profile value as its share of the axis range, `(value − min) / (max − min)`, values of no axis left out. */
function sheetCategory(category: Category, axes: readonly Axis[], keys: boolean): Category {
  if (keys) return category;
  const profile = category.profile;
  const shares =
    profile &&
    Object.fromEntries(
      Object.entries(profile).flatMap(([id, value]) => {
        const axis = axes.find((candidate) => candidate.id === id);
        return axis ? [[id, (value - axis.min) / (axis.max - axis.min)] as const] : [];
      }),
    );
  return { id: category.id, label: category.label, ...shortOf(category), ...iconOf(category), ...(shares ? { profile: shares } : {}) };
}

/** ⏱️ The seconds of a task as the optional field a timed sheet task carries. */
function secondsOf(rules: ChallengeRules, kind: Task["kind"], items: number, dimensions: number): { readonly seconds?: number } {
  return rules.timed ? { seconds: taskSeconds(kind, items, dimensions) } : {};
}

/** 🧱️ One task as presented under the rules of the challenge, consuming the generator in the normative order whatever the rules. */
function sheetTask(task: Task, random: RandomSource, rules: ChallengeRules, seed: number): SheetTask {
  switch (task.kind) {
    case "classification": {
      const items = pooled(task, seed, shuffle(random, task.items));
      const categories = shuffle(random, task.categories).map((category) => sheetCategory(category, task.axes ?? [], rules.keys));
      const axes = task.axes ? { axes: task.axes.map((axis) => sheetAxis(axis, rules.keys)) } : {};
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), ...axes, categories, items: items.map(sheetItem), ...(rules.timed && task.pool ? { seconds: task.pool.seconds } : secondsOf(rules, task.kind, items.length, 1)) };
    }
    case "sorting": {
      const shuffled = pooled(task, seed, shuffle(random, task.items));
      const ascending = ascendingItems(task, shuffled);
      const sorted = shuffled.length >= 2 && ascending.every((item, position) => item.id === shuffled[position]!.id);
      const items = sorted ? [...shuffled.slice(1), shuffled[0]!] : shuffled;
      const keys = rules.keys ? { keys: ascending.map((item) => item.value) } : {};
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), quantity: task.quantity, ...keys, items: items.map(sheetItem), ...(rules.timed && task.pool ? { seconds: task.pool.seconds } : secondsOf(rules, task.kind, items.length, 1)) };
    }
    case "matching": {
      const items = pooled(task, seed, shuffle(random, task.items));
      const dimensions = task.dimensions.map((dimension) => {
        const cards = shuffle(
          random,
          items.map((item) => (Object.hasOwn(item.values, dimension.id) ? item.values[dimension.id]! : Number.NaN)),
        );
        return { id: dimension.id, quantity: dimension.quantity, ...iconOf(dimension), ...(rules.keys ? { cards } : {}) };
      });
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), dimensions, items: items.map(sheetItem), ...(rules.timed && task.pool ? { seconds: task.pool.seconds } : secondsOf(rules, task.kind, items.length, task.dimensions.length)) };
    }
  }
}

/** 🗄️ How many sheets {@link sheetOf} keeps per quiz: the runs a device shows at once, with room to spare. */
export const SHEETS_KEPT = 16;

const DEALT = new WeakMap<Quiz, Map<string, Sheet>>();

/** 🧮️ The sheet of `quiz` for `seed` at `challenge`, dealt anew. */
function deal(quiz: Quiz, seed: number, challenge: Challenge): Sheet {
  const random = new Mt19937(seed);
  const rules = challengeRules(challenge);
  const shuffledOrder = shuffle(random, quiz.tasks.map((_, index) => index));
  const order = shuffledOrder.length > 1 ? [0, ...shuffledOrder.filter((index) => index !== 0)] : shuffledOrder;
  const tasks = quiz.tasks.map((task) => sheetTask(task, random, rules, seed));
  return { quiz: quiz.id, seed: seed >>> 0, challenge, title: quiz.title, description: quiz.description, tasks: order.map((index) => tasks[index]!) };
}

/** 🎴️ The sheet of `quiz` for `seed` at `challenge`, dealt once and handed out again for as long as it is among the last {@link SHEETS_KEPT} of its quiz — a screen, a view and a decision that ask for the same run share one sheet, which nobody may change: the first task stays first; the rest are shuffled before every task is presented in definition order. */
export function sheetOf(quiz: Quiz, seed: number, challenge: Challenge): Sheet {
  const kept = DEALT.get(quiz) ?? new Map<string, Sheet>();
  const key = `${seed}:${challenge}`;
  const known = kept.get(key);
  if (known !== undefined) return known;
  const sheet = deal(quiz, seed, challenge);
  if (kept.size >= SHEETS_KEPT) kept.delete(kept.keys().next().value!);
  kept.set(key, sheet);
  DEALT.set(quiz, kept);
  return sheet;
}
