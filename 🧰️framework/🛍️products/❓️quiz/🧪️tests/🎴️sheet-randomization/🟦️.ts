import { describe, expect, it } from "vitest";
import { Mt19937, runSeed, sheetOf, shuffle, type ClassificationTask, type MatchingTask, type Quantity, type Quiz, type SheetTask, type SortingTask, type Text } from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 🌍️ A two-language text. */
function text(en: string, de: string = en): Text {
  return { en, de };
}

/** ⚖️ A quantity on the given scale. */
function quantity(unit: string, scale: Quantity["scale"]): Quantity {
  return { label: text(unit), unit, scale, prefixed: false };
}

const CLASSIFICATION: ClassificationTask = {
  kind: "classification",
  id: "power-or-energy",
  title: text("Power or energy", "Leistung oder Energie"),
  prompt: text("Classify", "Ordne zu"),
  axes: [
    { id: "heating", label: text("Heating", "Heizen"), unit: "kWh", min: 0, max: 100 },
    { id: "cooling", label: text("Cooling", "Kühlen"), unit: "kWh", min: 0, max: 50 },
    { id: "cost", label: text("Cost", "Kosten"), unit: "€", min: 0, max: 10 },
  ],
  categories: [
    { id: "power", label: text("Power", "Leistung"), profile: { heating: 10, cooling: 5, cost: 1 } },
    { id: "energy", label: text("Energy", "Energie"), profile: { heating: 90, cooling: 45, cost: 9 } },
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

describe("sheetOf", () => {
  it("is a pure function of quiz and seed", () => {
    const seed = runSeed("0123456789abcdef0123456789abcdef");
    expect(sheetOf(QUIZ, seed)).toEqual(sheetOf(QUIZ, seed));
    expect(sheetOf(QUIZ, 1)).not.toEqual(sheetOf(QUIZ, 2));
  });

  it("copies the quiz head and the seed", () => {
    const sheet = sheetOf(QUIZ, 4294967295);
    expect({ quiz: sheet.quiz, seed: sheet.seed, title: sheet.title, description: sheet.description }).toEqual({ quiz: "physics", seed: 4294967295, title: QUIZ.title, description: QUIZ.description });
  });

  it("consumes the generator in the normative order: task order, then every task in definition order", () => {
    for (const seed of [0, 1, 5489, runSeed("ffffffffffffffffffffffffffffffff")]) {
      const random = new Mt19937(seed);
      const order = shuffle(random, [0, 1, 2]);
      const classificationItems = shuffle(random, CLASSIFICATION.items).slice(0, 4);
      const categories = shuffle(random, CLASSIFICATION.categories);
      const shuffledSorting = shuffle(random, SORTING.items).slice(0, 5);
      const sortingIds = shuffledSorting.map((item) => item.id);
      const sortingItems = ascending(SORTING, sortingIds) ? [...sortingIds.slice(1), sortingIds[0]!] : sortingIds;
      const matchingItems = shuffle(random, MATCHING.items);
      const cards = MATCHING.dimensions.map((dimension) => shuffle(random, matchingItems.map((item) => item.values[dimension.id]!)));
      const expected: SheetTask[] = [
        { kind: "classification", id: CLASSIFICATION.id, title: CLASSIFICATION.title, prompt: CLASSIFICATION.prompt, axes: CLASSIFICATION.axes!, categories, items: classificationItems.map((item) => ({ id: item.id, label: item.label })) },
        { kind: "sorting", id: SORTING.id, title: SORTING.title, prompt: SORTING.prompt, quantity: SORTING.quantity, items: sortingItems.map((id) => ({ id, label: SORTING.items.find((item) => item.id === id)!.label })) },
        { kind: "matching", id: MATCHING.id, title: MATCHING.title, prompt: MATCHING.prompt, dimensions: MATCHING.dimensions.map((dimension, index) => ({ id: dimension.id, quantity: dimension.quantity, cards: cards[index]! })), items: matchingItems.map((item) => ({ id: item.id, label: item.label })) },
      ];
      expect(sheetOf(QUIZ, seed).tasks).toEqual(order.map((index) => expected[index]));
    }
  });

  it("never reveals a solution", () => {
    for (let seed = 0; seed < 50; seed++) {
      const serialized = JSON.stringify(sheetOf(QUIZ, seed));
      for (const secret of ['"category"', '"value"', '"values"', '"explanation"', '"draw"']) expect(serialized).not.toContain(secret);
      for (const task of sheetOf(QUIZ, seed).tasks) for (const item of task.items) expect(Object.keys(item)).toEqual(["id", "label"]);
    }
  });

  it("keeps every structural invariant over many seeds", () => {
    for (let seed = 0; seed < 500; seed++) {
      const sheet = sheetOf(QUIZ, seed);
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
        expect(sorted(dimension.cards)).toEqual(sorted(truth));
      }
    }
  });

  it("takes every item when draw is absent or not smaller than the item count", () => {
    const whole = { ...QUIZ, tasks: [{ ...SORTING, draw: 6 }, { ...CLASSIFICATION, draw: undefined }] } as Quiz;
    for (let seed = 0; seed < 20; seed++) {
      const sheet = sheetOf(whole, seed);
      expect(presented(sheet.tasks, SORTING.id).items).toHaveLength(6);
      expect(presented(sheet.tasks, CLASSIFICATION.id).items).toHaveLength(6);
    }
  });

  it("rotates an already ascending sorting by one, so two items always show the larger first", () => {
    const pair: SortingTask = { ...SORTING, draw: undefined, items: [SORTING.items[0]!, SORTING.items[5]!] };
    const quiz: Quiz = { ...QUIZ, tasks: [pair] };
    const seen = new Set<string>();
    for (let seed = 0; seed < 200; seed++) seen.add(sheetOf(quiz, seed).tasks[0]!.items.map((item) => item.id).join(","));
    expect([...seen]).toEqual(["nuclear-plant,tea-light"]);
  });

  it("orders ties by definition index before deciding to rotate", () => {
    const tied: SortingTask = { ...SORTING, draw: undefined, items: [{ id: "first", label: text("First"), value: 5 }, { id: "second", label: text("Second"), value: 5 }] };
    const quiz: Quiz = { ...QUIZ, tasks: [tied] };
    const seen = new Set<string>();
    for (let seed = 0; seed < 200; seed++) seen.add(sheetOf(quiz, seed).tasks[0]!.items.map((item) => item.id).join(","));
    expect([...seen]).toEqual(["second,first"]);
  });

  it("never presents a drawn sorting subset in ascending order", () => {
    for (let seed = 0; seed < 2000; seed++) {
      const sheet = sheetOf({ ...QUIZ, tasks: [{ ...SORTING, draw: 2 }] }, seed);
      expect(ascending(SORTING, sheet.tasks[0]!.items.map((item) => item.id))).toBe(false);
    }
  });
});
