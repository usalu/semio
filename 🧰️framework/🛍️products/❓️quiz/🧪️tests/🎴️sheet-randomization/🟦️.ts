import { describe, expect, it } from "vitest";
import {
  CHALLENGES,
  Mt19937,
  challengeRules,
  runSeed,
  sheetOf,
  shuffle,
  taskSeconds,
  type Challenge,
  type ClassificationTask,
  type MatchingTask,
  type Quantity,
  type Quiz,
  type Sheet,
  type SheetTask,
  type SortingTask,
  type Text,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 🌍️ A two-language text. */
function text(en: string, de: string = en): Text {
  return { en, de };
}

/** ⚖️ A quantity on the given scale. */
function quantity(unit: string, scale: Quantity["scale"]): Quantity {
  return { label: text(unit), unit, scale, prefixed: false, additive: false };
}

const CLASSIFICATION: ClassificationTask = {
  kind: "classification",
  id: "power-or-energy",
  title: text("Power or energy", "Leistung oder Energie"),
  prompt: text("Classify", "Ordne zu"),
  axes: [
    { id: "heating", label: text("Heating", "Heizen"), unit: "kWh", min: 0, max: 100 },
    { id: "cooling", label: text("Cooling", "Kühlen"), unit: "kWh", min: 5, max: 50 },
    { id: "cost", label: text("Cost", "Kosten"), unit: "€", min: -10, max: 10 },
  ],
  categories: [
    { id: "power", label: text("Power", "Leistung"), description: text("Watts", "Watt"), profile: { heating: 10, cooling: 5, cost: 1 } },
    { id: "energy", label: text("Energy", "Energie"), description: text("Watt hours", "Wattstunden"), icon: { emoji: "🔋", motion: "pulse" }, profile: { heating: 90, cooling: 45, cost: 9 } },
    { id: "neither", label: text("Neither", "Keines") },
  ],
  items: ["kettle", "fridge-year", "bulb", "car-tank", "phone", "house-year"].map((id, index) => ({ id, label: text(id), category: index % 2 === 0 ? "power" : "energy", explanation: text(`because ${id}`) })),
  draw: 4,
};

const SORTING: SortingTask = {
  kind: "sorting",
  id: "power-ladder",
  title: text("Power ladder", "Leistungsleiter"),
  prompt: text("Sort ascending", "Aufsteigend sortieren"),
  quantity: quantity("W", "logarithmic"),
  items: [
    { id: "tea-light", label: text("Tea light", "Teelicht"), value: 30 },
    { id: "bulb", label: text("Bulb", "Glühbirne"), value: 60 },
    { id: "kettle", label: text("Kettle", "Wasserkocher"), value: 2000 },
    { id: "car", label: text("Car", "Auto"), value: 100000 },
    { id: "wind-turbine", label: text("Wind turbine", "Windrad"), value: 3000000 },
    { id: "nuclear-plant", label: text("Nuclear plant", "Kernkraftwerk"), value: 1400000000 },
  ],
  draw: 5,
};

const MATCHING: MatchingTask = {
  kind: "matching",
  id: "u-values",
  title: text("U-values", "U-Werte"),
  prompt: text("Match", "Zuordnen"),
  dimensions: [
    { id: "u-value", quantity: quantity("W/(m²K)", "linear") },
    { id: "demand", quantity: quantity("kWh/(m²a)", "logarithmic") },
  ],
  items: [
    { id: "old-wall", label: text("Old wall", "Altbauwand"), values: { "u-value": 1.4, demand: 250 } },
    { id: "wsvo-wall", label: text("WSVO wall", "WSVO-Wand"), values: { "u-value": 0.5, demand: 120 } },
    { id: "enev-wall", label: text("EnEV wall", "EnEV-Wand"), values: { "u-value": 0.24, demand: 70 } },
    { id: "passive-wall", label: text("Passive wall", "Passivhauswand"), values: { "u-value": 0.12, demand: 15 } },
  ],
};

const QUIZ: Quiz = { schema: "semio.quiz/v1", id: "physics", emoji: "🧲", title: text("Physics", "Physik"), description: text("Understanding", "Verständnis"), tasks: [CLASSIFICATION, SORTING, MATCHING] };

/** 🔎️ The presented task with the given id. */
function presented(tasks: readonly SheetTask[], id: string): SheetTask {
  const task = tasks.find((candidate) => candidate.id === id);
  if (!task) throw new Error(`missing ${id}`);
  return task;
}

/** 📶️ Whether sorting items are listed ascending by value, ties by definition index. */
function ascending(task: SortingTask, ids: readonly string[]): boolean {
  const index = (id: string) => task.items.findIndex((item) => item.id === id);
  const value = (id: string) => task.items[index(id)]!.value;
  return ids.every((id, position) => position === 0 || value(ids[position - 1]!) < value(id) || (value(ids[position - 1]!) === value(id) && index(ids[position - 1]!) < index(id)));
}

/** 🔢️ A numeric multiset as a sorted list. */
function sorted(values: readonly number[]): number[] {
  return [...values].sort((left, right) => left - right);
}

/** 🦴️ What a seed deals whatever the challenge: the task order and, per task, the item order and the category order. */
function dealt(sheet: Sheet): unknown {
  return sheet.tasks.map((task) => ({ id: task.id, items: task.items, categories: task.kind === "classification" ? task.categories.map((category) => category.id) : undefined }));
}

describe("sheetOf", () => {
  it("is a pure function of quiz, seed and challenge", () => {
    const seed = runSeed("0123456789abcdef0123456789abcdef");
    for (const challenge of CHALLENGES) expect(sheetOf(QUIZ, seed, challenge)).toEqual(sheetOf(QUIZ, seed, challenge));
    expect(sheetOf(QUIZ, 1, "medium")).not.toEqual(sheetOf(QUIZ, 2, "medium"));
    expect(sheetOf(QUIZ, 1, "medium")).not.toEqual(sheetOf(QUIZ, 1, "hard"));
  });

  it("copies the quiz head, the seed and the challenge", () => {
    for (const challenge of CHALLENGES) {
      const sheet = sheetOf(QUIZ, 4294967295, challenge);
      expect({ quiz: sheet.quiz, seed: sheet.seed, challenge: sheet.challenge, title: sheet.title, description: sheet.description }).toEqual({ quiz: "physics", seed: 4294967295, challenge, title: QUIZ.title, description: QUIZ.description });
      expect(Object.keys(sheet)).toEqual(["quiz", "seed", "challenge", "title", "description", "tasks"]);
    }
  });

  it("keeps the first task first and consumes the generator before presenting tasks in definition order, the same draws at every challenge", () => {
    for (const challenge of CHALLENGES) {
      const { keys, timed } = challengeRules(challenge);
      for (const seed of [0, 1, 5489, runSeed("ffffffffffffffffffffffffffffffff")]) {
        const random = new Mt19937(seed);
        const shuffledOrder = shuffle(random, [0, 1, 2]);
        const order = [0, ...shuffledOrder.filter((index) => index !== 0)];
        const classificationItems = shuffle(random, CLASSIFICATION.items).slice(0, 4);
        const categories = shuffle(random, CLASSIFICATION.categories);
        const shuffledSorting = shuffle(random, SORTING.items).slice(0, 5);
        const sortingIds = shuffledSorting.map((item) => item.id);
        const sortingItems = ascending(SORTING, sortingIds) ? [...sortingIds.slice(1), sortingIds[0]!] : sortingIds;
        const matchingItems = shuffle(random, MATCHING.items);
        const cards = MATCHING.dimensions.map((dimension) => shuffle(random, matchingItems.map((item) => item.values[dimension.id]!)));
        const shares = { power: { heating: 0.1, cooling: 0, cost: 0.55 }, energy: { heating: 0.9, cooling: 40 / 45, cost: 0.95 } } as const;
        const expected: SheetTask[] = [
          {
            kind: "classification",
            id: CLASSIFICATION.id,
            title: CLASSIFICATION.title,
            prompt: CLASSIFICATION.prompt,
            axes: keys ? CLASSIFICATION.axes! : CLASSIFICATION.axes!.map((axis) => ({ id: axis.id, label: axis.label })),
            categories: keys ? categories : categories.map((category) => ({ id: category.id, label: category.label, ...(category.icon ? { icon: category.icon } : {}), ...(category.id === "neither" ? {} : { profile: shares[category.id as "power" | "energy"] }) })),
            items: classificationItems.map((item) => ({ id: item.id, label: item.label })),
            ...(timed ? { seconds: 30 + 8 * 4 } : {}),
          },
          {
            kind: "sorting",
            id: SORTING.id,
            title: SORTING.title,
            prompt: SORTING.prompt,
            quantity: SORTING.quantity,
            ...(keys ? { keys: sorted(shuffledSorting.map((item) => item.value)) } : {}),
            items: sortingItems.map((id) => ({ id, label: SORTING.items.find((item) => item.id === id)!.label })),
            ...(timed ? { seconds: 30 + 12 * 5 } : {}),
          },
          {
            kind: "matching",
            id: MATCHING.id,
            title: MATCHING.title,
            prompt: MATCHING.prompt,
            dimensions: MATCHING.dimensions.map((dimension, index) => ({ id: dimension.id, quantity: dimension.quantity, ...(keys ? { cards: cards[index]! } : {}) })),
            items: matchingItems.map((item) => ({ id: item.id, label: item.label })),
            ...(timed ? { seconds: 30 + 12 * 4 * 2 } : {}),
          },
        ];
        expect(sheetOf(QUIZ, seed, challenge).tasks).toEqual(order.map((index) => expected[index]));
      }
    }
  });

  it("deals the same tasks, items and categories in the same order at every challenge", () => {
    for (let seed = 0; seed < 300; seed++) {
      const medium = dealt(sheetOf(QUIZ, seed, "medium"));
      for (const challenge of CHALLENGES) expect(dealt(sheetOf(QUIZ, seed, challenge))).toEqual(medium);
      const easy = sheetOf(QUIZ, seed, "easy");
      expect({ ...easy, challenge: "medium" }).toEqual(sheetOf(QUIZ, seed, "medium"));
      const expert = sheetOf(QUIZ, seed, "expert");
      expect({ ...expert, challenge: "hard", tasks: expert.tasks.map(({ seconds: _, ...task }) => task) }).toEqual(sheetOf(QUIZ, seed, "hard"));
    }
  });

  it("always presents the first defined task first while shuffling the remaining tasks", () => {
    const orders = new Set<string>();
    for (let seed = 0; seed < 100; seed++) {
      const tasks = sheetOf(QUIZ, seed, "medium").tasks;
      expect(tasks[0]!.id).toBe(CLASSIFICATION.id);
      orders.add(tasks.slice(1).map((task) => task.id).join(","));
    }
    expect(orders.size).toBeGreaterThan(1);
  });

  it("never says which item a number belongs to, and carries no number at all where the keys are hidden", () => {
    const numbers = (value: unknown): number[] => (typeof value === "number" ? [value] : typeof value === "object" && value !== null ? Object.values(value).flatMap(numbers) : []);
    for (let seed = 0; seed < 50; seed++) {
      for (const challenge of CHALLENGES) {
        const sheet = sheetOf(QUIZ, seed, challenge);
        const serialized = JSON.stringify(sheet);
        for (const secret of ['"category"', '"value"', '"values"', '"explanation"', '"draw"']) expect(serialized).not.toContain(secret);
        for (const task of sheet.tasks) for (const item of task.items) expect(Object.keys(item)).toEqual(["id", "label"]);
        if (challengeRules(challenge).keys) continue;
        for (const secret of ['"keys"', '"cards"', '"unit":"kWh"', '"min"', '"max"']) expect(serialized).not.toContain(secret);
        for (const task of sheet.tasks) {
          const { seconds: _, ...rest } = task;
          const shown = task.kind === "classification" ? { ...rest, categories: task.categories.map(({ profile: __, ...category }) => category) } : rest;
          expect(numbers(shown)).toEqual([]);
        }
      }
    }
  });

  it("shows the keys on easy and medium: an ascending ladder for a sorting, cards for a matching, descriptions and whole axes for a classification", () => {
    for (let seed = 0; seed < 200; seed++) {
      for (const challenge of ["easy", "medium"] as const) {
        const sheet = sheetOf(QUIZ, seed, challenge);
        const sorting = presented(sheet.tasks, SORTING.id);
        if (sorting.kind !== "sorting") throw new Error("kind");
        expect(sorting.keys).toEqual(sorted(sorting.items.map((item) => SORTING.items.find((candidate) => candidate.id === item.id)!.value)));
        expect(Object.keys(sorting)).toEqual(["kind", "id", "title", "prompt", "quantity", "keys", "items"]);
        const matching = presented(sheet.tasks, MATCHING.id);
        if (matching.kind !== "matching") throw new Error("kind");
        for (const dimension of matching.dimensions) expect(dimension.cards).toHaveLength(4);
        const classification = presented(sheet.tasks, CLASSIFICATION.id);
        if (classification.kind !== "classification") throw new Error("kind");
        expect(classification.axes).toEqual(CLASSIFICATION.axes);
        expect([...classification.categories].sort((left, right) => left.id.localeCompare(right.id))).toEqual([...CLASSIFICATION.categories].sort((left, right) => left.id.localeCompare(right.id)));
        for (const task of sheet.tasks) expect(task).not.toHaveProperty("seconds");
      }
    }
  });

  it("hides the keys on hard and expert: no ladder, no cards, no descriptions, axes without numbers and profiles as shares of the axis range", () => {
    for (let seed = 0; seed < 200; seed++) {
      for (const challenge of ["hard", "expert"] as const) {
        const sheet = sheetOf(QUIZ, seed, challenge);
        expect(presented(sheet.tasks, SORTING.id)).not.toHaveProperty("keys");
        const matching = presented(sheet.tasks, MATCHING.id);
        if (matching.kind !== "matching") throw new Error("kind");
        for (const dimension of matching.dimensions) expect(Object.keys(dimension)).toEqual(["id", "quantity"]);
        const classification = presented(sheet.tasks, CLASSIFICATION.id);
        if (classification.kind !== "classification") throw new Error("kind");
        expect(classification.axes).toEqual(CLASSIFICATION.axes!.map((axis) => ({ id: axis.id, label: axis.label })));
        const byId = Object.fromEntries(classification.categories.map((category) => [category.id, category]));
        expect(byId.power).toEqual({ id: "power", label: text("Power", "Leistung"), profile: { heating: (10 - 0) / (100 - 0), cooling: (5 - 5) / (50 - 5), cost: (1 - -10) / (10 - -10) } });
        expect(byId.energy).toEqual({ id: "energy", label: text("Energy", "Energie"), icon: { emoji: "🔋", motion: "pulse" }, profile: { heating: (90 - 0) / (100 - 0), cooling: (45 - 5) / (50 - 5), cost: (9 - -10) / (10 - -10) } });
        expect(byId.neither).toEqual({ id: "neither", label: text("Neither", "Keines") });
      }
    }
  });

  it("gives every task of a timed sheet its seconds, and no task of any other sheet", () => {
    for (const challenge of CHALLENGES) {
      for (let seed = 0; seed < 50; seed++) {
        for (const task of sheetOf(QUIZ, seed, challenge).tasks) {
          if (challenge !== "expert") expect(task).not.toHaveProperty("seconds");
          else {
            expect(task.seconds).toBe(taskSeconds(task.kind, task.items.length, task.kind === "matching" ? task.dimensions.length : 1));
            expect(Object.keys(task).at(-1)).toBe("seconds");
          }
        }
      }
    }
    const expert = sheetOf(QUIZ, 3, "expert");
    expect(Object.fromEntries(expert.tasks.map((task) => [task.id, task.seconds]))).toEqual({ "power-or-energy": 62, "power-ladder": 90, "u-values": 126 });
  });

  it("carries every short label right after its label at every challenge and never says which item is familiar", () => {
    const short = (en: string) => text(`${en} (short)`, `${en} (kurz)`);
    const classification: ClassificationTask = {
      ...CLASSIFICATION,
      axes: CLASSIFICATION.axes!.map((axis) => (axis.id === "cost" ? axis : { id: axis.id, label: axis.label, short: short(axis.id), unit: axis.unit, min: axis.min, max: axis.max })),
      categories: CLASSIFICATION.categories.map(({ id, label, ...category }) => (id === "neither" ? { id, label, ...category } : { id, label, short: short(id), ...category })),
      items: CLASSIFICATION.items.map((item, index) => (index % 2 === 0 ? { id: item.id, label: item.label, short: short(item.id), category: item.category } : item)),
    };
    const sorting: SortingTask = { ...SORTING, quantity: { ...SORTING.quantity, short: short("W") }, items: SORTING.items.map((item, index) => ({ id: item.id, label: item.label, ...(index < 3 ? { short: short(item.id), familiar: true } : { familiar: false }), value: item.value })) };
    const matching: MatchingTask = { ...MATCHING, items: MATCHING.items.map((item) => ({ id: item.id, label: item.label, short: short(item.id), values: item.values, familiar: item.id === "old-wall" })) };
    const quiz: Quiz = { ...QUIZ, tasks: [classification, sorting, matching] };
    for (const challenge of CHALLENGES) {
      for (let seed = 0; seed < 20; seed++) {
        const sheet = sheetOf(quiz, seed, challenge);
        expect(JSON.stringify(sheet)).not.toContain('"familiar"');
        for (const task of sheet.tasks) {
          const source: readonly { readonly id: string; readonly label: Text; readonly short?: Text }[] = [classification, sorting, matching].find((candidate) => candidate.id === task.id)!.items;
          for (const item of task.items) {
            const own = source.find((candidate) => candidate.id === item.id)!;
            expect(item).toEqual({ id: item.id, label: own.label, ...(own.short ? { short: own.short } : {}) });
            expect(Object.keys(item)).toEqual(own.short ? ["id", "label", "short"] : ["id", "label"]);
          }
        }
        const presentedSorting = presented(sheet.tasks, SORTING.id);
        if (presentedSorting.kind !== "sorting") throw new Error("kind");
        expect(presentedSorting.quantity.short).toEqual(short("W"));
        const presentedClassification = presented(sheet.tasks, CLASSIFICATION.id);
        if (presentedClassification.kind !== "classification") throw new Error("kind");
        for (const axis of presentedClassification.axes!) {
          expect(axis.short).toEqual(axis.id === "cost" ? undefined : short(axis.id));
          if (!challengeRules(challenge).keys) expect(Object.keys(axis)).toEqual(axis.id === "cost" ? ["id", "label"] : ["id", "label", "short"]);
        }
        for (const category of presentedClassification.categories) {
          expect(category.short).toEqual(category.id === "neither" ? undefined : short(category.id));
          if (!challengeRules(challenge).keys) expect(Object.keys(category).slice(0, 3)).toEqual(category.id === "neither" ? ["id", "label"] : ["id", "label", "short"]);
        }
      }
    }
  });

  it("presents a task without axes or profiles alike at every challenge, but for the descriptions", () => {
    const plain: ClassificationTask = { ...CLASSIFICATION, axes: undefined, categories: CLASSIFICATION.categories.map(({ profile: _, ...category }) => category) };
    const quiz: Quiz = { ...QUIZ, tasks: [plain] };
    const hard = sheetOf(quiz, 7, "hard").tasks[0]!;
    const medium = sheetOf(quiz, 7, "medium").tasks[0]!;
    if (hard.kind !== "classification" || medium.kind !== "classification") throw new Error("kind");
    expect(hard).not.toHaveProperty("axes");
    expect(hard.categories).toEqual(medium.categories.map(({ description: _, ...category }) => category));
    expect(medium.categories.some((category) => category.description !== undefined)).toBe(true);
  });

  it("keeps every structural invariant over many seeds", () => {
    for (let seed = 0; seed < 500; seed++) {
      const sheet = sheetOf(QUIZ, seed, "medium");
      expect(sorted(sheet.tasks.map((task) => QUIZ.tasks.findIndex((candidate) => candidate.id === task.id)))).toEqual([0, 1, 2]);
      const classification = presented(sheet.tasks, CLASSIFICATION.id);
      if (classification.kind !== "classification") throw new Error("kind");
      expect(classification.items).toHaveLength(4);
      expect(new Set(classification.items.map((item) => item.id)).size).toBe(4);
      expect(classification.items.every((item) => CLASSIFICATION.items.some((candidate) => candidate.id === item.id))).toBe(true);
      expect([...classification.categories].sort((left, right) => left.id.localeCompare(right.id))).toEqual([...CLASSIFICATION.categories].sort((left, right) => left.id.localeCompare(right.id)));
      expect(classification.axes).toEqual(CLASSIFICATION.axes);
      const sorting = presented(sheet.tasks, SORTING.id);
      expect(sorting.items).toHaveLength(5);
      expect(ascending(SORTING, sorting.items.map((item) => item.id))).toBe(false);
      const matching = presented(sheet.tasks, MATCHING.id);
      if (matching.kind !== "matching") throw new Error("kind");
      expect(matching.items).toHaveLength(4);
      for (const dimension of matching.dimensions) {
        const truth = matching.items.map((item) => MATCHING.items.find((candidate) => candidate.id === item.id)!.values[dimension.id]!);
        expect(sorted(dimension.cards!)).toEqual(sorted(truth));
      }
    }
  });

  it("takes every item when draw is absent or not smaller than the item count", () => {
    const whole = { ...QUIZ, tasks: [{ ...SORTING, draw: 6 }, { ...CLASSIFICATION, draw: undefined }] } as Quiz;
    for (let seed = 0; seed < 20; seed++) {
      for (const challenge of CHALLENGES) {
        const sheet = sheetOf(whole, seed, challenge);
        expect(presented(sheet.tasks, SORTING.id).items).toHaveLength(6);
        expect(presented(sheet.tasks, CLASSIFICATION.id).items).toHaveLength(6);
      }
    }
  });

  it("rotates an already ascending sorting by one, so two items always show the larger first, while the ladder stays ascending", () => {
    const pair: SortingTask = { ...SORTING, draw: undefined, items: [SORTING.items[0]!, SORTING.items[5]!] };
    const quiz: Quiz = { ...QUIZ, tasks: [pair] };
    const seen = new Set<string>();
    for (let seed = 0; seed < 200; seed++) {
      const task = sheetOf(quiz, seed, "easy").tasks[0]!;
      if (task.kind !== "sorting") throw new Error("kind");
      expect(task.keys).toEqual([30, 1400000000]);
      seen.add(task.items.map((item) => item.id).join(","));
    }
    expect([...seen]).toEqual(["nuclear-plant,tea-light"]);
  });

  it("orders ties by definition index before deciding to rotate", () => {
    const tied: SortingTask = { ...SORTING, draw: undefined, items: [{ id: "first", label: text("First"), value: 5 }, { id: "second", label: text("Second"), value: 5 }] };
    const quiz: Quiz = { ...QUIZ, tasks: [tied] };
    const seen = new Set<string>();
    for (const challenge of CHALLENGES) for (let seed = 0; seed < 200; seed++) seen.add(sheetOf(quiz, seed, challenge).tasks[0]!.items.map((item) => item.id).join(","));
    expect([...seen]).toEqual(["second,first"]);
  });

  it("never presents a drawn sorting subset in ascending order", () => {
    for (let seed = 0; seed < 2000; seed++) {
      const sheet = sheetOf({ ...QUIZ, tasks: [{ ...SORTING, draw: 2 }] }, seed, "hard");
      expect(ascending(SORTING, sheet.tasks[0]!.items.map((item) => item.id))).toBe(false);
    }
  });
});

describe("the sheet of a run id", () => {
  it("is the same at every challenge but for what the challenge shows", () => {
    const seed = runSeed("00112233445566778899aabbccddeeff");
    const challenges: readonly Challenge[] = CHALLENGES;
    expect(new Set(challenges.map((challenge) => JSON.stringify(dealt(sheetOf(QUIZ, seed, challenge))))).size).toBe(1);
  });
});
