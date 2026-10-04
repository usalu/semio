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
import type { Axis, Category, Challenge, Icon, Quiz, SheetAxis, SheetItem, SheetTask, Sheet, ShortText, SortingItem, Task, Text } from "../../🧬️schema/🟦️.ts";
import { challengeRules, taskSeconds, type ChallengeRules } from "../⛰️challenge/🟦️.ts";
import { Mt19937, shuffle, type RandomSource } from "../🎲️randomness/🟦️.ts";

/** 🪧️ The solution-free projection of an item: id, label, short label and icon. */
function sheetItem(item: { readonly id: string; readonly label: Text; readonly short?: ShortText; readonly icon?: Icon }): SheetItem {
  return { id: item.id, label: item.label, ...shortOf(item), ...iconOf(item) };
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
function sheetTask(task: Task, random: RandomSource, rules: ChallengeRules): SheetTask {
  switch (task.kind) {
    case "classification": {
      const items = drawn(shuffle(random, task.items), task.draw);
      const categories = shuffle(random, task.categories).map((category) => sheetCategory(category, task.axes ?? [], rules.keys));
      const axes = task.axes ? { axes: task.axes.map((axis) => sheetAxis(axis, rules.keys)) } : {};
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), ...axes, categories, items: items.map(sheetItem), ...secondsOf(rules, task.kind, items.length, 1) };
    }
    case "sorting": {
      const shuffled = drawn(shuffle(random, task.items), task.draw);
      const ascending = ascendingItems(task, shuffled);
      const sorted = shuffled.length >= 2 && ascending.every((item, position) => item.id === shuffled[position]!.id);
      const items = sorted ? [...shuffled.slice(1), shuffled[0]!] : shuffled;
      const keys = rules.keys ? { keys: ascending.map((item) => item.value) } : {};
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), quantity: task.quantity, ...keys, items: items.map(sheetItem), ...secondsOf(rules, task.kind, items.length, 1) };
    }
    case "matching": {
      const items = drawn(shuffle(random, task.items), task.draw);
      const dimensions = task.dimensions.map((dimension) => {
        const cards = shuffle(
          random,
          items.map((item) => (Object.hasOwn(item.values, dimension.id) ? item.values[dimension.id]! : Number.NaN)),
        );
        return { id: dimension.id, quantity: dimension.quantity, ...iconOf(dimension), ...(rules.keys ? { cards } : {}) };
      });
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), dimensions, items: items.map(sheetItem), ...secondsOf(rules, task.kind, items.length, task.dimensions.length) };
    }
  }
}

/** 🎴️ The sheet of `quiz` for `seed` at `challenge`: the first task stays first; the rest are shuffled before every task is presented in definition order. */
export function sheetOf(quiz: Quiz, seed: number, challenge: Challenge): Sheet {
  const random = new Mt19937(seed);
  const rules = challengeRules(challenge);
  const shuffledOrder = shuffle(random, quiz.tasks.map((_, index) => index));
  const order = shuffledOrder.length > 1 ? [0, ...shuffledOrder.filter((index) => index !== 0)] : shuffledOrder;
  const tasks = quiz.tasks.map((task) => sheetTask(task, random, rules));
  return { quiz: quiz.id, seed: seed >>> 0, challenge, title: quiz.title, description: quiz.description, tasks: order.map((index) => tasks[index]!) };
}
