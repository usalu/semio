/** 🃏️ The sheet of a run: the randomized, solution-free presentation of a quiz, a pure function of (quiz, seed).
 *
 * @see ../🎲️randomness/🟦️.ts — the generator and shuffle every draw goes through
 * @see ./🦀️.rs — the Rust twin
 */
import type { Icon, Quiz, SheetItem, SheetTask, Sheet, SortingItem, Task } from "../../🧬️schema/🟦️.ts";
import { Mt19937, shuffle, type RandomSource } from "../🎲️randomness/🟦️.ts";

/** 🪧️ The solution-free projection of an item. */
function sheetItem(item: { readonly id: string; readonly label: SheetItem["label"]; readonly icon?: Icon }): SheetItem {
  return { id: item.id, label: item.label, ...iconOf(item) };
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

/** 🧱️ One task as presented, consuming the generator in the normative order. */
function sheetTask(task: Task, random: RandomSource): SheetTask {
  switch (task.kind) {
    case "classification": {
      const items = drawn(shuffle(random, task.items), task.draw);
      const categories = shuffle(random, task.categories);
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), ...(task.axes ? { axes: task.axes } : {}), categories, items: items.map(sheetItem) };
    }
    case "sorting": {
      const shuffled = drawn(shuffle(random, task.items), task.draw);
      const sorted = shuffled.length >= 2 && ascendingItems(task, shuffled).every((item, position) => item.id === shuffled[position]!.id);
      const items = sorted ? [...shuffled.slice(1), shuffled[0]!] : shuffled;
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), quantity: task.quantity, items: items.map(sheetItem) };
    }
    case "matching": {
      const items = drawn(shuffle(random, task.items), task.draw);
      const dimensions = task.dimensions.map((dimension) => ({ id: dimension.id, quantity: dimension.quantity, ...iconOf(dimension), cards: shuffle(random, items.map((item) => (Object.hasOwn(item.values, dimension.id) ? item.values[dimension.id]! : Number.NaN))) }));
      return { kind: task.kind, id: task.id, title: task.title, prompt: task.prompt, ...iconOf(task), dimensions, items: items.map(sheetItem) };
    }
  }
}

/** 🃏️ The sheet of `quiz` for `seed`: the first task stays first; the rest are shuffled before every task is presented in definition order. */
export function sheetOf(quiz: Quiz, seed: number): Sheet {
  const random = new Mt19937(seed);
  const shuffledOrder = shuffle(random, quiz.tasks.map((_, index) => index));
  const order = shuffledOrder.length > 1 ? [0, ...shuffledOrder.filter((index) => index !== 0)] : shuffledOrder;
  const tasks = quiz.tasks.map((task) => sheetTask(task, random));
  return { quiz: quiz.id, seed: seed >>> 0, title: quiz.title, description: quiz.description, tasks: order.map((index) => tasks[index]!) };
}
