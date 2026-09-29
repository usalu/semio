import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import { answerComplete, answerRejection, catalogIssues, quizIssues, sheetOf, type Answer, type Quiz, type SheetTask, type ValidationIssue } from "../../📦️packages/🟦️typescript/🟦️.ts";

type Mutable = Record<string, any>;

const SCHEMA = JSON.parse(readFileSync(resolve(dirname(fileURLToPath(import.meta.url)), "../../🧬️schema/🔣️.json"), "utf8")) as { readonly $id: string };
const AJV = new Ajv({ strict: false, allErrors: true });
AJV.addSchema(SCHEMA);
const AJV_QUIZ = AJV.getSchema(`${SCHEMA.$id}#/$defs/Quiz`)!;
const AJV_CATALOG = AJV.getSchema(`${SCHEMA.$id}#/$defs/Catalog`)!;

/** 🧱️ The codes that restate a JSON Schema keyword; every other code is a semantic finding beyond the schema. */
const STRUCTURAL = new Set(["type-invalid", "required", "property-unknown", "value-invalid", "slug-invalid", "length-invalid", "integer-invalid", "below-minimum", "items-too-few", "properties-too-few", "duplicate-path"]);

const T = (en: string, de: string = en) => ({ en, de });

/** 📝️ A valid quiz with one task of every kind, profiles and a draw. */
function physics(): Mutable {
  return {
    $schema: "../../🧬️schema/🔣️.json",
    schema: "semio.quiz/v1",
    id: "physics",
    title: T("Physics", "Physik"),
    description: T("Power and energy", "Leistung und Energie"),
    tasks: [
      {
        kind: "classification",
        id: "standards",
        title: T("Standards", "Standards"),
        prompt: T("Classify", "Ordne zu"),
        axes: [
          { id: "heating", label: T("Heating", "Heizen"), unit: "kWh", min: 0, max: 100 },
          { id: "cooling", label: T("Cooling", "Kühlen"), unit: "kWh", min: 0, max: 50 },
          { id: "cost", label: T("Cost", "Kosten"), unit: "€", min: 0, max: 10 },
        ],
        categories: [
          { id: "passive", label: T("Passive house", "Passivhaus"), profile: { heating: 15, cooling: 5, cost: 8 } },
          { id: "old", label: T("Old building", "Altbau"), description: T("Before 1977", "Vor 1977"), profile: { heating: 100, cooling: 20, cost: 2 } },
          { id: "unknown", label: T("Unknown", "Unbekannt") },
        ],
        items: [
          { id: "villa", label: T("Villa"), category: "old", explanation: T("Built 1920", "Gebaut 1920") },
          { id: "office", label: T("Office", "Büro"), category: "passive" },
          { id: "school", label: T("School", "Schule"), category: "passive" },
        ],
        draw: 2,
      },
      {
        kind: "sorting",
        id: "power",
        title: T("Power", "Leistung"),
        prompt: T("Sort", "Sortiere"),
        quantity: { label: T("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true },
        items: [
          { id: "tea-light", label: T("Tea light", "Teelicht"), value: 30 },
          { id: "kettle", label: T("Kettle", "Wasserkocher"), value: 2000 },
          { id: "plant", label: T("Plant", "Kraftwerk"), value: 1.4e9 },
        ],
      },
      {
        kind: "matching",
        id: "walls",
        title: T("Walls", "Wände"),
        prompt: T("Match", "Zuordnen"),
        dimensions: [
          { id: "u-value", quantity: { label: T("U-value", "U-Wert"), unit: "W/(m²K)", scale: "linear", prefixed: false } },
          { id: "demand", quantity: { label: T("Demand", "Bedarf"), unit: "kWh/(m²a)", scale: "logarithmic", prefixed: false } },
        ],
        items: [
          { id: "old-wall", label: T("Old wall", "Altbauwand"), values: { "u-value": 1.4, demand: 250 } },
          { id: "passive-wall", label: T("Passive wall", "Passivhauswand"), values: { "u-value": 0.12, demand: 15 } },
        ],
      },
    ],
  };
}

/** 📚️ A valid catalog over the physics quiz and a one-task heating quiz. */
function catalog(): Mutable {
  return {
    schema: "semio.quiz.catalog/v1",
    id: "architecture",
    title: T("Architecture", "Architektur"),
    introduction: { title: T("Welcome", "Willkommen"), paragraphs: [T("Hello", "Hallo")] },
    quizzes: ["⚡️energy/🧲️physics/❓️quiz/🔣️.json", "⚡️energy/🔥️heating/❓️quiz/🔣️.json"],
    badges: [
      { id: "physicist", emoji: "🧲", label: T("Physicist", "Physikerin"), description: T("Perfect physics", "Physik perfekt"), rule: { kind: "perfect-quiz", quiz: "physics" } },
      { id: "sorter", emoji: "📶", label: T("Sorter", "Sortiererin"), description: T("Every sorting", "Jede Sortierung"), rule: { kind: "perfect-tasks", taskKind: "sorting" } },
      { id: "matcher", emoji: "🔗", label: T("Matcher"), description: T("Physics matchings"), rule: { kind: "perfect-tasks", taskKind: "matching", quiz: "physics" } },
      { id: "completionist", emoji: "🏁", label: T("Completionist"), description: T("Every quiz", "Jedes Quiz"), rule: { kind: "completed-quizzes" } },
    ],
  };
}

/** 🔥️ A one-task sorting quiz. */
function heating(): Quiz {
  const quiz = physics();
  return { ...quiz, id: "heating", tasks: [quiz.tasks[1]] } as unknown as Quiz;
}

/** 🧾️ The quiz issues, checked against ajv: a document is schema-valid exactly when no structural issue is reported. */
function checkedQuiz(document: unknown): ValidationIssue[] {
  const issues = quizIssues(document);
  expect(AJV_QUIZ(document), JSON.stringify(issues)).toBe(issues.every((issue) => !STRUCTURAL.has(issue.code)));
  return issues;
}

/** 🧾️ The catalog issues, checked against ajv like {@link checkedQuiz}. */
function checkedCatalog(document: unknown, quizzes: readonly Quiz[]): ValidationIssue[] {
  const issues = catalogIssues(document, quizzes);
  expect(AJV_CATALOG(document), JSON.stringify(issues)).toBe(issues.every((issue) => !STRUCTURAL.has(issue.code)));
  return issues;
}

/** 🦠️ The physics quiz after `mutate`. */
function mutated(mutate: (quiz: Mutable) => void): Mutable {
  const quiz = physics();
  mutate(quiz);
  return quiz;
}

describe("quizIssues", () => {
  it("accepts a valid quiz", () => {
    expect(checkedQuiz(physics())).toEqual([]);
  });

  it("rejects what is not a quiz object", () => {
    for (const value of [null, [], "quiz", 3, true]) expect(checkedQuiz(value)).toEqual([{ path: "", code: "type-invalid" }]);
  });

  const structural: readonly [string, (quiz: Mutable) => void, readonly ValidationIssue[]][] = [
    ["a missing title", (quiz) => delete quiz.title, [{ path: "/title", code: "required" }]],
    ["an unknown property", (quiz) => (quiz.author = "x"), [{ path: "/author", code: "property-unknown" }]],
    ["a foreign schema", (quiz) => (quiz.schema = "semio.quiz/v2"), [{ path: "/schema", code: "value-invalid" }]],
    ["an invalid slug", (quiz) => (quiz.id = "Physics"), [{ path: "/id", code: "slug-invalid" }]],
    ["a too long slug", (quiz) => (quiz.id = "a".repeat(65)), [{ path: "/id", code: "slug-invalid" }]],
    ["an empty text", (quiz) => (quiz.title.de = ""), [{ path: "/title/de", code: "length-invalid" }]],
    ["a text without German", (quiz) => delete quiz.description.de, [{ path: "/description/de", code: "required" }]],
    ["a text with a third language", (quiz) => (quiz.description.fr = "x"), [{ path: "/description/fr", code: "property-unknown" }]],
    ["no tasks", (quiz) => (quiz.tasks = []), [{ path: "/tasks", code: "items-too-few" }]],
    ["an unknown task kind", (quiz) => (quiz.tasks[0].kind = "essay"), [{ path: "/tasks/0/kind", code: "value-invalid" }]],
    ["a task without kind", (quiz) => delete quiz.tasks[2].kind, [{ path: "/tasks/2/kind", code: "required" }]],
    ["axes on a sorting", (quiz) => (quiz.tasks[1].axes = quiz.tasks[0].axes), [{ path: "/tasks/1/axes", code: "property-unknown" }]],
    ["two axes", (quiz) => quiz.tasks[0].axes.pop(), [{ path: "/tasks/0/axes", code: "items-too-few" }, { path: "/tasks/0/categories/0/profile/cost", code: "axis-unknown" }, { path: "/tasks/0/categories/1/profile/cost", code: "axis-unknown" }]],
    ["one category", (quiz) => quiz.tasks[0].categories.splice(1), [{ path: "/tasks/0/categories", code: "items-too-few" }, { path: "/tasks/0/items/0/category", code: "category-unknown" }]],
    ["a draw of one", (quiz) => (quiz.tasks[0].draw = 1), [{ path: "/tasks/0/draw", code: "below-minimum" }]],
    ["a fractional draw", (quiz) => (quiz.tasks[0].draw = 2.5), [{ path: "/tasks/0/draw", code: "integer-invalid" }]],
    ["a string value", (quiz) => (quiz.tasks[1].items[0].value = "30"), [{ path: "/tasks/1/items/0/value", code: "type-invalid" }]],
    ["an unknown scale", (quiz) => (quiz.tasks[1].quantity.scale = "decibel"), [{ path: "/tasks/1/quantity/scale", code: "value-invalid" }]],
    ["a too long unit", (quiz) => (quiz.tasks[1].quantity.unit = "W".repeat(33)), [{ path: "/tasks/1/quantity/unit", code: "length-invalid" }]],
    ["a non-boolean prefix flag", (quiz) => (quiz.tasks[1].quantity.prefixed = "yes"), [{ path: "/tasks/1/quantity/prefixed", code: "type-invalid" }]],
    ["an empty profile", (quiz) => (quiz.tasks[0].categories[0].profile = {}), [{ path: "/tasks/0/categories/0/profile", code: "properties-too-few" }, ...["cooling", "cost", "heating"].map((axis) => ({ path: `/tasks/0/categories/0/profile/${axis}`, code: "profile-incomplete" }))]],
    ["a profile key with a slash", (quiz) => (quiz.tasks[0].categories[0].profile["a/b~c"] = 1), [{ path: "/tasks/0/categories/0/profile/a~1b~0c", code: "axis-unknown" }, { path: "/tasks/0/categories/0/profile/a~1b~0c", code: "slug-invalid" }]],
    ["no matching values", (quiz) => (quiz.tasks[2].items[0].values = {}), [{ path: "/tasks/2/items/0/values", code: "properties-too-few" }, { path: "/tasks/2/items/0/values/demand", code: "value-missing" }, { path: "/tasks/2/items/0/values/u-value", code: "value-missing" }]],
    ["a single matching item", (quiz) => quiz.tasks[2].items.pop(), [{ path: "/tasks/2/items", code: "items-too-few" }]],
    ["no dimensions", (quiz) => (quiz.tasks[2].dimensions = []), [{ path: "/tasks/2/dimensions", code: "items-too-few" }, ...[0, 1].flatMap((item) => ["demand", "u-value"].map((key) => ({ path: `/tasks/2/items/${item}/values/${key}`, code: "dimension-unknown" })))]],
  ];
  for (const [name, mutate, expected] of structural) it(`reports ${name}`, () => expect(checkedQuiz(mutated(mutate))).toEqual(expected));

  const semantic: readonly [string, (quiz: Mutable) => void, readonly ValidationIssue[]][] = [
    ["a duplicate task id", (quiz) => (quiz.tasks[2].id = "power"), [{ path: "/tasks/2/id", code: "duplicate-id" }]],
    ["a duplicate item id", (quiz) => (quiz.tasks[1].items[2].id = "kettle"), [{ path: "/tasks/1/items/2/id", code: "duplicate-id" }]],
    ["a duplicate category id", (quiz) => (quiz.tasks[0].categories[2].id = "old"), [{ path: "/tasks/0/categories/2/id", code: "duplicate-id" }]],
    ["a duplicate axis id", (quiz) => (quiz.tasks[0].axes[2].id = "cooling"), [{ path: "/tasks/0/axes/2/id", code: "duplicate-id" }, { path: "/tasks/0/categories/0/profile/cost", code: "axis-unknown" }, { path: "/tasks/0/categories/1/profile/cost", code: "axis-unknown" }]],
    ["a duplicate dimension id", (quiz) => (quiz.tasks[2].dimensions[1].id = "u-value"), [{ path: "/tasks/2/dimensions/1/id", code: "duplicate-id" }, { path: "/tasks/2/items/0/values/demand", code: "dimension-unknown" }, { path: "/tasks/2/items/1/values/demand", code: "dimension-unknown" }]],
    ["an unknown category", (quiz) => (quiz.tasks[0].items[1].category = "modern"), [{ path: "/tasks/0/items/1/category", code: "category-unknown" }]],
    ["a profile without axes", (quiz) => delete quiz.tasks[0].axes, [{ path: "/tasks/0/categories/0/profile", code: "axes-missing" }, { path: "/tasks/0/categories/1/profile", code: "axes-missing" }]],
    ["an incomplete profile", (quiz) => delete quiz.tasks[0].categories[1].profile.cost, [{ path: "/tasks/0/categories/1/profile/cost", code: "profile-incomplete" }]],
    ["a profile on an unknown axis", (quiz) => (quiz.tasks[0].categories[1].profile.venting = 3), [{ path: "/tasks/0/categories/1/profile/venting", code: "axis-unknown" }]],
    ["an empty axis range", (quiz) => (quiz.tasks[0].axes[1].max = 0), [{ path: "/tasks/0/axes/1/max", code: "axis-range-invalid" }]],
    ["a profile value off its axis", (quiz) => (quiz.tasks[0].categories[0].profile.heating = 120), [{ path: "/tasks/0/categories/0/profile/heating", code: "profile-out-of-range" }]],
    ["a draw beyond the items", (quiz) => (quiz.tasks[0].draw = 4), [{ path: "/tasks/0/draw", code: "draw-exceeds-items" }]],
    ["a zero on a logarithmic sorting", (quiz) => (quiz.tasks[1].items[0].value = 0), [{ path: "/tasks/1/items/0/value", code: "value-not-positive" }]],
    ["a negative on a logarithmic dimension", (quiz) => (quiz.tasks[2].items[1].values.demand = -1), [{ path: "/tasks/2/items/1/values/demand", code: "value-not-positive" }]],
    ["a missing dimension value", (quiz) => delete quiz.tasks[2].items[1].values["u-value"], [{ path: "/tasks/2/items/1/values/u-value", code: "value-missing" }]],
    ["a value for an unknown dimension", (quiz) => (quiz.tasks[2].items[1].values.cost = 3), [{ path: "/tasks/2/items/1/values/cost", code: "dimension-unknown" }]],
  ];
  for (const [name, mutate, expected] of semantic) it(`reports ${name} beyond the schema`, () => expect(checkedQuiz(mutated(mutate))).toEqual(expected));

  it("accepts zero and negative values on linear scales and a draw equal to the item count", () => {
    expect(
      checkedQuiz(
        mutated((quiz) => {
          quiz.tasks[1].quantity.scale = "linear";
          quiz.tasks[1].items[0].value = -5;
          quiz.tasks[2].items[0].values["u-value"] = 0;
          quiz.tasks[0].draw = 3;
        }),
      ),
    ).toEqual([]);
  });

  it("returns issues deduplicated and sorted by path, then code", () => {
    const issues = checkedQuiz(
      mutated((quiz) => {
        quiz.tasks[1].items[0].value = 0;
        quiz.id = "X";
        quiz.tasks[0].items[1].category = "modern";
        quiz.tasks[0].items[1].id = "villa";
      }),
    );
    expect(issues).toEqual([
      { path: "/id", code: "slug-invalid" },
      { path: "/tasks/0/items/1/category", code: "category-unknown" },
      { path: "/tasks/0/items/1/id", code: "duplicate-id" },
      { path: "/tasks/1/items/0/value", code: "value-not-positive" },
    ]);
  });
});

describe("catalogIssues", () => {
  const quizzes = [physics() as Quiz, heating()];

  it("accepts a valid catalog", () => {
    expect(checkedCatalog(catalog(), quizzes)).toEqual([]);
  });

  const cases: readonly [string, (catalog: Mutable) => void, readonly Quiz[], readonly ValidationIssue[]][] = [
    ["a foreign schema", (document) => (document.schema = "semio.quiz/v1"), quizzes, [{ path: "/schema", code: "value-invalid" }]],
    ["no quizzes", (document) => (document.quizzes = []), [], [{ path: "/badges/0/rule/quiz", code: "quiz-unknown" }, { path: "/badges/1/rule", code: "badge-unreachable" }, { path: "/badges/2/rule/quiz", code: "quiz-unknown" }, { path: "/quizzes", code: "items-too-few" }]],
    ["a repeated quiz path", (document) => document.quizzes.push(document.quizzes[0]), [...quizzes, quizzes[0]!], [{ path: "/quizzes/2", code: "duplicate-id" }, { path: "/quizzes/2", code: "duplicate-path" }]],
    ["an empty quiz path", (document) => (document.quizzes[1] = ""), quizzes, [{ path: "/quizzes/1", code: "length-invalid" }]],
    ["an introduction without paragraphs", (document) => (document.introduction.paragraphs = []), quizzes, [{ path: "/introduction/paragraphs", code: "items-too-few" }]],
    ["a too long badge emoji", (document) => (document.badges[0].emoji = "🧲".repeat(17)), quizzes, [{ path: "/badges/0/emoji", code: "length-invalid" }]],
    ["a perfect-quiz rule without quiz", (document) => delete document.badges[0].rule.quiz, quizzes, [{ path: "/badges/0/rule/quiz", code: "required" }]],
    ["an unknown rule kind", (document) => (document.badges[3].rule.kind = "streak"), quizzes, [{ path: "/badges/3/rule/kind", code: "value-invalid" }]],
    ["an unknown selector kind", (document) => (document.badges[1].rule.taskKind = "essay"), quizzes, [{ path: "/badges/1/rule/taskKind", code: "value-invalid" }]],
    ["a selector on completed quizzes", (document) => (document.badges[3].rule.quiz = "physics"), quizzes, [{ path: "/badges/3/rule/quiz", code: "property-unknown" }]],
    ["a duplicate badge id", (document) => (document.badges[2].id = "sorter"), quizzes, [{ path: "/badges/2/id", code: "duplicate-id" }]],
    ["a badge on an unknown quiz", (document) => (document.badges[0].rule.quiz = "cooling"), quizzes, [{ path: "/badges/0/rule/quiz", code: "quiz-unknown" }]],
    ["a selector on an unknown quiz", (document) => (document.badges[2].rule.quiz = "cooling"), quizzes, [{ path: "/badges/2/rule/quiz", code: "quiz-unknown" }]],
    ["a selector no task matches", (document) => (document.badges[2].rule.quiz = "heating"), quizzes, [{ path: "/badges/2/rule", code: "badge-unreachable" }]],
    ["loaded quizzes that do not match the paths", (document) => document, [quizzes[0]!], [{ path: "/quizzes", code: "quiz-count-mismatch" }]],
    ["two quizzes with one id", (document) => document, [quizzes[0]!, quizzes[0]!], [{ path: "/quizzes/1", code: "duplicate-id" }]],
  ];
  for (const [name, mutate, loaded, expected] of cases) {
    it(`reports ${name}`, () => {
      const document = catalog();
      mutate(document);
      expect(checkedCatalog(document, loaded)).toEqual(expected);
    });
  }
});

describe("answers", () => {
  const quiz = physics() as Quiz;
  const sheet = sheetOf(quiz, 5489);
  const task = (kind: SheetTask["kind"]): SheetTask => sheet.tasks.find((candidate) => candidate.kind === kind)!;
  const classification = task("classification");
  const sorting = task("sorting");
  const matching = task("matching");
  const drawn = classification.items.map((item) => item.id);
  const hidden = ["villa", "office", "school"].find((id) => !drawn.includes(id))!;

  it("accepts partial and complete answers of the right kind", () => {
    const valid: readonly [SheetTask, Answer][] = [
      [classification, { kind: "classification", assignments: {} }],
      [classification, { kind: "classification", assignments: { [drawn[0]!]: "unknown" } }],
      [sorting, { kind: "sorting", order: sorting.items.map((item) => item.id).reverse() }],
      [matching, { kind: "matching", assignments: { "u-value": { "old-wall": 1 } } }],
      [matching, { kind: "matching", assignments: { "u-value": { "old-wall": 1, "passive-wall": 0 }, demand: { "old-wall": 0, "passive-wall": 1 } } }],
    ];
    for (const [sheetTask, answer] of valid) expect(answerRejection(sheetTask, answer)).toBeUndefined();
  });

  it("rejects answers that do not fit the sheet task", () => {
    const invalid: readonly [SheetTask, unknown][] = [
      [classification, { kind: "sorting", order: [] }],
      [classification, null],
      [classification, { kind: "classification", assignments: { [hidden]: "old" } }],
      [classification, { kind: "classification", assignments: { [drawn[0]!]: "modern" } }],
      [classification, { kind: "classification", assignments: [] }],
      [sorting, { kind: "sorting", order: sorting.items.map((item) => item.id).slice(1) }],
      [sorting, { kind: "sorting", order: [sorting.items[0]!.id, ...sorting.items.map((item) => item.id).slice(1, -1), sorting.items[0]!.id] }],
      [sorting, { kind: "sorting", order: [...sorting.items.map((item) => item.id).slice(1), "unknown"] }],
      [matching, { kind: "matching", assignments: { cost: { "old-wall": 0 } } }],
      [matching, { kind: "matching", assignments: { "u-value": { "old-wall": 2 } } }],
      [matching, { kind: "matching", assignments: { "u-value": { "old-wall": -1 } } }],
      [matching, { kind: "matching", assignments: { "u-value": { "old-wall": 0.5 } } }],
      [matching, { kind: "matching", assignments: { "u-value": { "old-wall": 0, "passive-wall": 0 } } }],
      [matching, { kind: "matching", assignments: { "u-value": { attic: 0 } } }],
    ];
    for (const [sheetTask, answer] of invalid) expect(answerRejection(sheetTask, answer as Answer), JSON.stringify(answer)).toBe("answer-invalid");
  });

  it("knows when an answer completes its task", () => {
    expect(answerComplete(classification)).toBe(false);
    expect(answerComplete(classification, { kind: "classification", assignments: { [drawn[0]!]: "old" } })).toBe(false);
    expect(answerComplete(classification, { kind: "classification", assignments: Object.fromEntries(drawn.map((id) => [id, "old"])) })).toBe(true);
    expect(answerComplete(sorting, { kind: "sorting", order: sorting.items.map((item) => item.id) })).toBe(true);
    expect(answerComplete(sorting, { kind: "classification", assignments: {} })).toBe(false);
    expect(answerComplete(matching, { kind: "matching", assignments: { "u-value": { "old-wall": 1, "passive-wall": 0 } } })).toBe(false);
    expect(answerComplete(matching, { kind: "matching", assignments: { "u-value": { "old-wall": 1, "passive-wall": 0 }, demand: { "old-wall": 0 } } })).toBe(false);
    expect(answerComplete(matching, { kind: "matching", assignments: { "u-value": { "old-wall": 1, "passive-wall": 0 }, demand: { "old-wall": 0, "passive-wall": 1 } } })).toBe(true);
  });
});
