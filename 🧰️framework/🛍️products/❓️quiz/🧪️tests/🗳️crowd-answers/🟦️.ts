import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  Mt19937,
  THINKING_LIMIT,
  crowdView,
  cursorIssues,
  valueKey,
  thinkingAnswer,
  thinkingCrowd,
  thinkingIssues,
  thinkingProblem,
  thinkingScope,
  type Quiz,
  type RunResult,
  type SheetTask,
  type Text,
  type ThinkingState,
  type ValidationIssue,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

type Mutable = Record<string, any>;

const SCHEMA = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧬️schema/🔣️.json"), "utf8")) as { readonly $id: string };
const AJV = new Ajv({ strict: false, allErrors: true });
AJV.addSchema(SCHEMA);
const AJV_THINKING = AJV.getSchema(`${SCHEMA.$id}#/$defs/ThinkingState`)!;
const AJV_CURSOR = AJV.getSchema(`${SCHEMA.$id}#/$defs/CursorState`)!;
const AJV_CROWD = AJV.getSchema(`${SCHEMA.$id}#/$defs/CrowdView`)!;
const T = (en: string): Text => ({ en, de: en });
const TAG = "1097e4ce";

const QUIZ: Quiz = {
  schema: "semio.quiz/v1",
  id: "physics",
  emoji: "🧲",
  title: T("Physics"),
  description: T("Physics"),
  tasks: [
    { kind: "classification", id: "units", title: T("Units"), prompt: T("Classify"), categories: [{ id: "power", label: T("Power") }, { id: "energy", label: T("Energy") }], items: [{ id: "watt", label: T("W"), category: "power" }, { id: "kwh", label: T("kWh"), category: "energy" }, { id: "joule", label: T("J"), category: "energy" }] },
    { kind: "sorting", id: "power", title: T("Power"), prompt: T("Sort"), quantity: { label: T("P"), unit: "W", scale: "logarithmic", prefixed: true }, items: [{ id: "bulb", label: T("Bulb"), value: 60 }, { id: "kettle", label: T("Kettle"), value: 2000 }, { id: "plant", label: T("Plant"), value: 1e9 }, { id: "phone", label: T("Phone"), value: 5 }], draw: 3 },
    {
      kind: "matching",
      id: "walls",
      title: T("Walls"),
      prompt: T("Match"),
      dimensions: [
        { id: "u", quantity: { label: T("U"), unit: "W/(m²K)", scale: "linear", prefixed: false } },
        { id: "demand", quantity: { label: T("Demand"), unit: "kWh/(m²a)", scale: "logarithmic", prefixed: false } },
      ],
      items: [
        { id: "old", label: T("Old"), values: { u: 1.4, demand: 250 } },
        { id: "new", label: T("New"), values: { u: 0.2, demand: 9 } },
        { id: "mid", label: T("Mid"), values: { u: 0.5, demand: 10 } },
      ],
    },
  ],
};

/** 🗃️ A classification result assigning categories to items. */
function classified(assignments: Readonly<Record<string, string>>): RunResult["tasks"][number] {
  return { kind: "classification", task: "units", score: 0, items: Object.entries(assignments).map(([item, assigned]) => ({ item, assigned, correct: assigned, credit: 1 })) };
}

/** 📶️ A sorting result in the learner's order. */
function sorted(order: readonly string[]): RunResult["tasks"][number] {
  return { kind: "sorting", task: "power", score: 0, items: order.map((item, position) => ({ item, value: 0, position, rank: position })) };
}

/** 🔗️ A matching result with the assigned value per dimension and item. */
function matched(dimensions: Readonly<Record<string, Readonly<Record<string, number>>>>): RunResult["tasks"][number] {
  return { kind: "matching", task: "walls", score: 0, dimensions: Object.entries(dimensions).map(([dimension, items]) => ({ dimension, score: 0, items: Object.entries(items).map(([item, assigned]) => ({ item, assigned, correct: assigned })) })) };
}

const RESULTS: readonly RunResult[] = [
  { quiz: "physics", score: 0, tasks: [classified({ watt: "power", kwh: "power" }), sorted(["kettle", "bulb", "plant"]), matched({ u: { old: 1.4, new: 0.5 }, demand: { old: 10, new: 9 } })] },
  { quiz: "physics", score: 0, tasks: [classified({ watt: "energy", joule: "energy" }), sorted(["phone", "bulb"]), matched({ u: { old: 1.4, new: 0.2, mid: 0.5 }, demand: { old: 9, new: 9, mid: 10 } })] },
  { quiz: "heating", score: 0, tasks: [classified({ watt: "power", kwh: "power", joule: "power" }), sorted(["bulb", "plant"])] },
  { quiz: "physics", score: 0, tasks: [classified({ kwh: "energy" }), { ...classified({ bulb: "power" }), task: "power" }] },
  { quiz: "physics", score: 0, tasks: [sorted(["plant"])] },
];

describe("valueKey", () => {
  it("renders the shortest round-trip JSON number of ECMAScript Number::toString", () => {
    const cases: readonly [number, string][] = [
      [0.12, "0.12"],
      [250, "250"],
      [1.4e9, "1400000000"],
      [1e21, "1e+21"],
      [1e-7, "1e-7"],
      [0.000001, "0.000001"],
      [-0, "0"],
      [-2.5, "-2.5"],
      [0.1 + 0.2, "0.30000000000000004"],
      [123e-20, "1.23e-18"],
      [Number.MAX_VALUE, "1.7976931348623157e+308"],
      [5e-324, "5e-324"],
    ];
    for (const [value, text] of cases) expect(valueKey(value), String(value)).toBe(text);
  });

  it("round-trips and equals JSON.stringify for random doubles", () => {
    const random = new Mt19937(17);
    const bytes = new DataView(new ArrayBuffer(8));
    for (let i = 0; i < 2000; i++) {
      bytes.setUint32(0, random.next());
      bytes.setUint32(4, random.next());
      const value = bytes.getFloat64(0);
      if (!Number.isFinite(value)) continue;
      expect(Number(valueKey(value))).toBe(value);
      expect(valueKey(value)).toBe(JSON.stringify(value));
    }
  });
});

describe("crowdView", () => {
  const view = crowdView(QUIZ, RESULTS);

  it("aggregates the submitted results of one quiz per task, dimension and item in definition order", () => {
    expect(view).toEqual({
      quiz: "physics",
      runs: 4,
      tasks: [
        {
          task: "units",
          kind: "classification",
          items: [
            { item: "watt", answers: 2, counts: [{ key: "energy", count: 1 }, { key: "power", count: 1 }] },
            { item: "kwh", answers: 2, counts: [{ key: "energy", count: 1 }, { key: "power", count: 1 }] },
            { item: "joule", answers: 1, counts: [{ key: "energy", count: 1 }] },
          ],
        },
        {
          task: "power",
          kind: "sorting",
          items: [
            { item: "bulb", answers: 2, meanPosition: 0.75 },
            { item: "kettle", answers: 1, meanPosition: 0 },
            { item: "plant", answers: 2, meanPosition: 0.5 },
            { item: "phone", answers: 1, meanPosition: 0 },
          ],
        },
        {
          task: "walls",
          kind: "matching",
          dimension: "u",
          items: [
            { item: "old", answers: 2, counts: [{ key: "1.4", count: 2 }] },
            { item: "new", answers: 2, counts: [{ key: "0.2", count: 1 }, { key: "0.5", count: 1 }] },
            { item: "mid", answers: 1, counts: [{ key: "0.5", count: 1 }] },
          ],
        },
        {
          task: "walls",
          kind: "matching",
          dimension: "demand",
          items: [
            { item: "old", answers: 2, counts: [{ key: "10", count: 1 }, { key: "9", count: 1 }] },
            { item: "new", answers: 2, counts: [{ key: "9", count: 2 }] },
            { item: "mid", answers: 1, counts: [{ key: "10", count: 1 }] },
          ],
        },
      ],
    });
  });

  it("is a valid CrowdView (ajv) and keeps every task even when nobody answered", () => {
    expect(AJV_CROWD(view), JSON.stringify(AJV_CROWD.errors)).toBe(true);
    const empty = crowdView(QUIZ, []);
    expect(empty).toEqual({ quiz: "physics", runs: 0, tasks: [{ task: "units", kind: "classification", items: [] }, { task: "power", kind: "sorting", items: [] }, { task: "walls", kind: "matching", dimension: "u", items: [] }, { task: "walls", kind: "matching", dimension: "demand", items: [] }] });
    expect(AJV_CROWD(empty)).toBe(true);
  });

  it("orders value keys and category keys by code point, and merges -0 with 0", () => {
    const results: RunResult[] = [100, 9, 10, 0.5, -0, 0, 1e21].map((value) => ({ quiz: "physics", score: 0, tasks: [matched({ u: { old: value } })] }));
    const u = crowdView(QUIZ, results).tasks[2]!;
    expect(u.items[0]!.counts).toEqual([{ key: "0", count: 2 }, { key: "0.5", count: 1 }, { key: "10", count: 1 }, { key: "100", count: 1 }, { key: "1e+21", count: 1 }, { key: "9", count: 1 }]);
    const categories = crowdView(QUIZ, ["b", "B", "a", "ä", "aa"].map((assigned) => ({ quiz: "physics", score: 0, tasks: [classified({ watt: assigned })] }))).tasks[0]!;
    expect(categories.items[0]!.counts!.map((count) => count.key)).toEqual(["B", "a", "aa", "b", "ä"]);
  });

  it("sums sorting positions in result order and recomputes like a brute-force reading", () => {
    const random = new Mt19937(3);
    const ids = ["bulb", "kettle", "plant", "phone"];
    const results: RunResult[] = Array.from({ length: 40 }, () => {
      const length = 1 + (random.next() % 4);
      const order = ids.slice(0, length).sort(() => 0).map((_, index, all) => all[(index + (random.next() % length)) % length]!);
      return { quiz: "physics", score: 0, tasks: [sorted([...new Set(order)])] };
    });
    const crowd = crowdView(QUIZ, results).tasks[1]!;
    for (const id of ids) {
      const positions = results.flatMap((result) => {
        const task = result.tasks[0]!;
        if (task.kind !== "sorting") return [];
        const index = task.items.findIndex((item) => item.item === id);
        return index < 0 ? [] : [task.items.length > 1 ? index / (task.items.length - 1) : 0];
      });
      const item = crowd.items.find((candidate) => candidate.item === id);
      if (positions.length === 0) expect(item).toBeUndefined();
      else expect(item).toEqual({ item: id, answers: positions.length, meanPosition: positions.reduce((sum, position) => sum + position, 0) / positions.length });
    }
  });
});

describe("thinking", () => {
  /** 💭️ The thinking issues, checked against ajv: schema-valid exactly when only the size bound fires. */
  function checked(state: unknown): ValidationIssue[] {
    const issues = thinkingIssues(state);
    expect(AJV_THINKING(state), JSON.stringify(issues)).toBe(issues.every((issue) => ["too-many", "duplicate-id"].includes(issue.code)));
    expect(thinkingProblem(state)).toEqual(issues[0]);
    return issues;
  }

  /** 💭️ A valid state with a partial draft of every kind. */
  function thinking(): Mutable {
    return { tag: TAG, answers: { units: { kind: "classification", assignments: { watt: "power" } }, power: { kind: "sorting", order: ["bulb", "plant"] }, walls: { kind: "matching", values: { u: { old: 1.4 }, demand: {} } } } };
  }

  it("names the thinking room of a quiz", () => {
    expect(thinkingScope("architecture", "physics")).toBe("architecture/quiz/physics/thinking");
  });

  it("admits empty and partial drafts", () => {
    expect(checked({ tag: TAG, answers: {} })).toEqual([]);
    expect(checked(thinking())).toEqual([]);
  });

  const cases: readonly [string, (state: Mutable) => void, readonly ValidationIssue[]][] = [
    ["missing answers", (state) => delete state.answers, [{ path: "/answers", code: "required" }]],
    ["a learner id as tag", (state) => (state.tag = "8d3ab6ac9d4fd9067ae6a4deac46190d"), [{ path: "/tag", code: "tag-invalid" }]],
    ["a smuggled learner id", (state) => (state.learner = "8d3ab6ac9d4fd9067ae6a4deac46190d"), [{ path: "/learner", code: "property-unknown" }]],
    ["answers as a list", (state) => (state.answers = []), [{ path: "/answers", code: "type-invalid" }]],
    ["a task id that is no slug", (state) => (state.answers.Units = state.answers.units), [{ path: "/answers/Units", code: "slug-invalid" }]],
    ["an answer without kind", (state) => delete state.answers.units.kind, [{ path: "/answers/units/kind", code: "required" }]],
    ["an unknown answer kind", (state) => (state.answers.units.kind = "essay"), [{ path: "/answers/units/kind", code: "value-invalid" }]],
    ["an extra answer member", (state) => (state.answers.power.score = 1), [{ path: "/answers/power/score", code: "property-unknown" }]],
    ["a category that is no slug", (state) => (state.answers.units.assignments.watt = "Power"), [{ path: "/answers/units/assignments/watt", code: "slug-invalid" }]],
    ["an item that is no slug", (state) => (state.answers.units.assignments["W att"] = "power"), [{ path: "/answers/units/assignments/W att", code: "slug-invalid" }]],
    ["an order that is no list", (state) => (state.answers.power.order = "bulb"), [{ path: "/answers/power/order", code: "type-invalid" }]],
    ["an order entry that is no slug", (state) => (state.answers.power.order[1] = 7), [{ path: "/answers/power/order/1", code: "type-invalid" }]],
    ["a textual value", (state) => (state.answers.walls.values.u.old = "1.4"), [{ path: "/answers/walls/values/u/old", code: "type-invalid" }]],
    ["a dimension that is no map", (state) => (state.answers.walls.values.u = [0]), [{ path: "/answers/walls/values/u", code: "type-invalid" }]],
    ["card indices instead of values", (state) => (state.answers.walls = { kind: "matching", assignments: { u: { old: 0 } } }), [{ path: "/answers/walls/assignments", code: "property-unknown" }, { path: "/answers/walls/values", code: "required" }]],
  ];
  for (const [name, mutate, expected] of cases) {
    it(`reports ${name}`, () => {
      const state = thinking();
      mutate(state);
      expect(checked(state)).toEqual(expected);
    });
  }

  it("refuses matching values JSON cannot carry, like the Rust twin (type-invalid)", () => {
    for (const value of [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY]) {
      const state = thinking();
      state.answers.walls.values.u.old = value;
      expect(thinkingIssues(state)).toEqual([{ path: "/answers/walls/values/u/old", code: "type-invalid" }]);
    }
  });

  it("bounds tasks and entries by THINKING_LIMIT = 64 and refuses repeated sorting items", () => {
    const ids = (count: number) => Array.from({ length: count }, (_, index) => `item-${index}`);
    const map = <V>(count: number, value: V) => Object.fromEntries(ids(count).map((id) => [id, value]));
    expect(THINKING_LIMIT).toBe(64);
    expect(checked({ tag: TAG, answers: map(64, { kind: "sorting", order: [] }) })).toEqual([]);
    expect(checked({ tag: TAG, answers: map(65, { kind: "sorting", order: [] }) })).toEqual([{ path: "/answers", code: "too-many" }]);
    expect(checked({ tag: TAG, answers: { units: { kind: "classification", assignments: map(64, "power") } } })).toEqual([]);
    expect(checked({ tag: TAG, answers: { units: { kind: "classification", assignments: map(65, "power") } } })).toEqual([{ path: "/answers/units/assignments", code: "too-many" }]);
    expect(checked({ tag: TAG, answers: { power: { kind: "sorting", order: ids(64) } } })).toEqual([]);
    expect(checked({ tag: TAG, answers: { power: { kind: "sorting", order: ids(65) } } })).toEqual([{ path: "/answers/power/order", code: "too-many" }]);
    expect(checked({ tag: TAG, answers: { power: { kind: "sorting", order: ["bulb", "plant", "bulb", "bulb"] } } })).toEqual([
      { path: "/answers/power/order/2", code: "duplicate-id" },
      { path: "/answers/power/order/3", code: "duplicate-id" },
    ]);
    expect(checked({ tag: TAG, answers: { walls: { kind: "matching", values: map(65, {}) } } })).toEqual([{ path: "/answers/walls/values", code: "too-many" }]);
    expect(checked({ tag: TAG, answers: { walls: { kind: "matching", values: { u: map(65, 0.5) } } } })).toEqual([{ path: "/answers/walls/values/u", code: "too-many" }]);
    expect(checked({ tag: TAG, answers: { walls: { kind: "matching", values: { u: map(64, -1e300) } } } })).toEqual([]);
  });
});

describe("cursors on items and drags", () => {
  /** 👆️ The cursor issues, checked against ajv. */
  function checked(state: unknown): ValidationIssue[] {
    const issues = cursorIssues(state);
    expect(AJV_CURSOR(state), JSON.stringify(issues)).toBe(issues.length === 0);
    return issues;
  }

  it("admits item and category anchors and a dragged item", () => {
    expect(checked({ tag: TAG, cursor: { anchor: "item:joule", x: 0.5, y: 0.5 }, focus: "category:energy", drag: { item: "joule" } })).toEqual([]);
  });

  const cases: readonly [string, unknown, readonly ValidationIssue[]][] = [
    ["an item anchor without id", { tag: TAG, cursor: { anchor: "item:", x: 0.5, y: 0.5 } }, [{ path: "/cursor/anchor", code: "anchor-invalid" }]],
    ["a dragged item that is no slug", { tag: TAG, drag: { item: "Joule" } }, [{ path: "/drag/item", code: "slug-invalid" }]],
    ["a drag without item", { tag: TAG, drag: {} }, [{ path: "/drag/item", code: "required" }]],
    ["a drag with a target", { tag: TAG, drag: { item: "joule", target: "energy" } }, [{ path: "/drag/target", code: "property-unknown" }]],
    ["a drag that is no object", { tag: TAG, drag: "joule" }, [{ path: "/drag", code: "type-invalid" }]],
  ];
  for (const [name, state, expected] of cases) it(`reports ${name}`, () => expect(checked(state)).toEqual(expected));
});

describe("thinkingCrowd", () => {
  const head = { title: T("Task"), prompt: T("Task") };
  const units: SheetTask = { kind: "classification", id: "units", ...head, categories: [{ id: "power", label: T("Power") }, { id: "energy", label: T("Energy") }], items: [{ id: "watt", label: T("W") }, { id: "kwh", label: T("kWh") }] };
  const power: SheetTask = { kind: "sorting", id: "power", ...head, quantity: { label: T("P"), unit: "W", scale: "logarithmic", prefixed: true }, items: [{ id: "plant", label: T("Plant") }, { id: "bulb", label: T("Bulb") }, { id: "phone", label: T("Phone") }] };
  const walls: SheetTask = { kind: "matching", id: "walls", ...head, dimensions: [{ id: "u", quantity: { label: T("U"), unit: "u", scale: "linear", prefixed: false }, cards: [1.4, 0.2] }, { id: "demand", quantity: { label: T("D"), unit: "d", scale: "logarithmic", prefixed: false }, cards: [9, 250] }], items: [{ id: "old", label: T("Old") }, { id: "new", label: T("New") }] };
  const states: readonly ThinkingState[] = [
    { tag: "0000000a", answers: { units: { kind: "classification", assignments: { watt: "power", joule: "energy" } } } },
    { tag: "0000000b", answers: { units: { kind: "classification", assignments: { watt: "energy", kwh: "energy" } }, power: { kind: "sorting", order: ["phone", "bulb"] }, walls: { kind: "matching", values: { u: { old: 0.2, new: 1.4 }, demand: { new: 250 } } } } },
    { tag: "0000000c", answers: { units: { kind: "sorting", order: ["watt"] } } },
    { tag: "0000000a", answers: { units: { kind: "classification", assignments: { watt: "energy" } }, power: { kind: "sorting", order: ["bulb", "kettle", "plant"] }, walls: { kind: "matching", values: { u: { old: 1.4 } } } } },
  ];

  it("counts category votes per item of the viewer's sheet, the latest state per tag winning", () => {
    expect(thinkingCrowd(states, units)).toEqual([{ task: "units", kind: "classification", items: [{ item: "watt", tags: ["0000000a", "0000000b"], votes: [{ key: "energy", tags: ["0000000a", "0000000b"] }] }, { item: "kwh", tags: ["0000000b"], votes: [{ key: "energy", tags: ["0000000b"] }] }] }]);
  });

  it("places sorting items at each learner's normalized position", () => {
    expect(thinkingCrowd(states, power)).toEqual([
      {
        task: "power",
        kind: "sorting",
        items: [
          { item: "plant", tags: ["0000000a"], positions: [{ tag: "0000000a", position: 1 }] },
          { item: "bulb", tags: ["0000000a", "0000000b"], positions: [{ tag: "0000000a", position: 0 }, { tag: "0000000b", position: 1 }] },
          { item: "phone", tags: ["0000000b"], positions: [{ tag: "0000000b", position: 0 }] },
        ],
      },
    ]);
  });

  it("shows per dimension and item the distribution of the values the others matched", () => {
    expect(thinkingCrowd(states, walls)).toEqual([
      {
        task: "walls",
        kind: "matching",
        dimension: "u",
        items: [
          { item: "old", tags: ["0000000a", "0000000b"], votes: [{ key: "0.2", tags: ["0000000b"] }, { key: "1.4", tags: ["0000000a"] }] },
          { item: "new", tags: ["0000000b"], votes: [{ key: "1.4", tags: ["0000000b"] }] },
        ],
      },
      { task: "walls", kind: "matching", dimension: "demand", items: [{ item: "new", tags: ["0000000b"], votes: [{ key: "250", tags: ["0000000b"] }] }] },
    ]);
  });

  it("turns the publisher's own answer into a draft peers can read", () => {
    expect(thinkingAnswer(units, { kind: "classification", assignments: { watt: "power" } })).toEqual({ kind: "classification", assignments: { watt: "power" } });
    expect(thinkingAnswer(power, { kind: "sorting", order: ["bulb", "phone", "plant"] })).toEqual({ kind: "sorting", order: ["bulb", "phone", "plant"] });
    expect(thinkingAnswer(walls, { kind: "matching", assignments: { u: { old: 1, new: 0 }, demand: { new: 1 } } })).toEqual({ kind: "matching", values: { u: { old: 0.2, new: 1.4 }, demand: { new: 250 } } });
    expect(thinkingAnswer(walls, { kind: "matching", assignments: {} })).toEqual({ kind: "matching", values: {} });
    const broken: SheetTask = { ...walls, dimensions: [{ ...walls.dimensions[0]!, cards: [Number.NaN, 0.2] }] } as SheetTask;
    expect(thinkingAnswer(broken, { kind: "matching", assignments: { u: { old: 0, new: 1 } } })).toEqual({ kind: "matching", values: { u: { new: 0.2 } } });
    expect(thinkingAnswer(walls, { kind: "matching", assignments: { u: { old: 5 } } })).toBeUndefined();
    expect(thinkingAnswer(units, { kind: "sorting", order: ["watt", "kwh"] })).toBeUndefined();
    for (const [sheetTask, answer] of [[units, { kind: "classification", assignments: { kwh: "energy" } }], [walls, { kind: "matching", assignments: { u: { old: 0 } } }]] as const) {
      const state = { tag: "0000000f", answers: { [sheetTask.id]: thinkingAnswer(sheetTask, answer)! } };
      expect(thinkingProblem(state)).toBeUndefined();
    }
  });

  it("does not depend on the order of different tags and is empty without drafts", () => {
    const [a, b, c, latest] = states;
    expect(thinkingCrowd([c!, b!, a!, latest!], units)).toEqual(thinkingCrowd(states, units));
    expect(thinkingCrowd([], power)).toEqual([{ task: "power", kind: "sorting", items: [] }]);
  });
});
