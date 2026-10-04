import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { describe, expect, it } from "vitest";
import {
  CHALLENGES,
  DEFAULT_LIMITS,
  REJECTIONS,
  VERDICTS,
  answerComplete,
  answerRejection,
  catalogIssues,
  catalogView,
  challengeRules,
  crowdView,
  decideLearner,
  emptyLearnerState,
  evolveLearner,
  leaderboard,
  learnerView,
  quizIssues,
  runSeed,
  runView,
  sheetOf,
  transcript,
  type Answer,
  type Catalog,
  type Challenge,
  type Command,
  type Event,
  type LearnerContext,
  type LearnerState,
  type Quiz,
  type SheetTask,
  type ValidationIssue,
} from "../../📦️packages/🟦️typescript/🟦️.ts";

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
    emoji: "🧲",
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
        quantity: { label: T("Power", "Leistung"), unit: "W", scale: "logarithmic", prefixed: true, additive: true },
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
          { id: "u-value", quantity: { label: T("U-value", "U-Wert"), unit: "W/(m²K)", scale: "linear", prefixed: false, additive: false } },
          { id: "demand", quantity: { label: T("Demand", "Bedarf"), unit: "kWh/(m²a)", scale: "logarithmic", prefixed: false, additive: false } },
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
    ["a missing emoji", (quiz) => delete quiz.emoji, [{ path: "/emoji", code: "required" }]],
    ["an empty emoji", (quiz) => (quiz.emoji = ""), [{ path: "/emoji", code: "length-invalid" }]],
    ["an emoji of 17 code points", (quiz) => (quiz.emoji = "🧲".repeat(17)), [{ path: "/emoji", code: "length-invalid" }]],
    ["a numeric emoji", (quiz) => (quiz.emoji = 1), [{ path: "/emoji", code: "type-invalid" }]],
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
    ["a non-boolean additive flag", (quiz) => (quiz.tasks[2].dimensions[1].quantity.additive = 1), [{ path: "/tasks/2/dimensions/1/quantity/additive", code: "type-invalid" }]],
    ["a quantity without its additive flag", (quiz) => delete quiz.tasks[1].quantity.additive, [{ path: "/tasks/1/quantity/additive", code: "required" }]],
    ["a short item label of 41 code points", (quiz) => (quiz.tasks[1].items[0].short = T("t".repeat(41), "Teelicht")), [{ path: "/tasks/1/items/0/short/en", code: "length-invalid" }]],
    ["a short quantity label of 41 code points in German", (quiz) => (quiz.tasks[2].dimensions[0].quantity.short = T("U", "ü".repeat(41))), [{ path: "/tasks/2/dimensions/0/quantity/short/de", code: "length-invalid" }]],
    ["an empty short axis label", (quiz) => (quiz.tasks[0].axes[0].short = T("", "Heizen")), [{ path: "/tasks/0/axes/0/short/en", code: "length-invalid" }]],
    ["a short category label without German", (quiz) => (quiz.tasks[0].categories[0].short = { en: "Passive" }), [{ path: "/tasks/0/categories/0/short/de", code: "required" }]],
    ["a short label that is no text", (quiz) => (quiz.tasks[0].items[0].short = "Villa"), [{ path: "/tasks/0/items/0/short", code: "type-invalid" }]],
    ["a short label on a matching item with a third language", (quiz) => (quiz.tasks[2].items[1].short = { en: "Passive", de: "Passiv", fr: "Passif" }), [{ path: "/tasks/2/items/1/short/fr", code: "property-unknown" }]],
    ["a short label on a task", (quiz) => (quiz.tasks[1].short = T("Power")), [{ path: "/tasks/1/short", code: "property-unknown" }]],
    ["a non-boolean familiar flag", (quiz) => (quiz.tasks[2].items[0].familiar = "yes"), [{ path: "/tasks/2/items/0/familiar", code: "type-invalid" }]],
    ["a familiar flag on a classification item", (quiz) => (quiz.tasks[0].items[0].familiar = true), [{ path: "/tasks/0/items/0/familiar", code: "property-unknown" }]],
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

  it("counts emoji length in code points, so joined and flagged emojis and 16 code points pass", () => {
    for (const emoji of ["🧑‍🏫", "❄️", "🇨🇭", "🧲".repeat(16)]) expect(checkedQuiz(mutated((quiz) => (quiz.emoji = emoji))), emoji).toEqual([]);
  });

  it("accepts short labels of up to 40 code points on quantities, axes, categories and items, and familiar flags on sorting and matching items", () => {
    expect(
      checkedQuiz(
        mutated((quiz) => {
          quiz.tasks[0].axes[0].short = T("heating", "Heizen");
          quiz.tasks[0].categories[2].short = T("x".repeat(40), "ß".repeat(40));
          quiz.tasks[0].items[1].short = T("🧲".repeat(40), "Büro");
          quiz.tasks[1].quantity.short = T("power", "Leistung");
          quiz.tasks[1].items[2].short = T("Plant", "Kraftwerk");
          quiz.tasks[1].items[0].familiar = true;
          quiz.tasks[1].items[1].familiar = false;
          quiz.tasks[2].dimensions[1].quantity.short = T("demand", "Bedarf");
          quiz.tasks[2].items[0].short = T("Old wall", "Altbau");
          quiz.tasks[2].items[0].familiar = true;
        }),
      ),
    ).toEqual([]);
  });

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

  it("accepts every challenge as the least challenge of a perfect-quiz and a perfect-tasks rule", () => {
    for (const challenge of CHALLENGES) {
      const document = catalog();
      document.badges[0].rule.challenge = challenge;
      document.badges[1].rule.challenge = challenge;
      document.badges[2].rule.challenge = challenge;
      expect(checkedCatalog(document, quizzes)).toEqual([]);
    }
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
    ["a least challenge that is no challenge", (document) => (document.badges[0].rule.challenge = "legendary"), quizzes, [{ path: "/badges/0/rule/challenge", code: "value-invalid" }]],
    ["a least challenge that is no string", (document) => (document.badges[2].rule.challenge = 3), quizzes, [{ path: "/badges/2/rule/challenge", code: "value-invalid" }]],
    ["a least challenge on completed quizzes", (document) => (document.badges[3].rule.challenge = "hard"), quizzes, [{ path: "/badges/3/rule/challenge", code: "property-unknown" }]],
    ["a least challenge beside an unknown quiz", (document) => Object.assign(document.badges[0].rule, { quiz: "cooling", challenge: "hard" }), quizzes, [{ path: "/badges/0/rule/quiz", code: "quiz-unknown" }]],
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
  const sheet = sheetOf(quiz, 5489, "medium");
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

describe("answers by challenge", () => {
  const quiz = physics() as Quiz;
  const task = (challenge: Challenge, kind: SheetTask["kind"]): SheetTask => sheetOf(quiz, 5489, challenge).tasks.find((candidate) => candidate.kind === kind)!;
  const SHOWN = ["easy", "medium"] as const;
  const HIDDEN = ["hard", "expert"] as const;
  const order = ["tea-light", "kettle", "plant"];
  const cards = { "u-value": { "old-wall": 1, "passive-wall": 0 }, demand: { "old-wall": 0, "passive-wall": 1 } };
  const guessed = { "u-value": { "old-wall": 1.2, "passive-wall": 0.1 }, demand: { "old-wall": 300, "passive-wall": 20 } };

  it("takes a sorting without guesses where the keys show and refuses any guesses there, also none at all", () => {
    for (const challenge of SHOWN) {
      const sorting = task(challenge, "sorting");
      expect(answerRejection(sorting, { kind: "sorting", order })).toBeUndefined();
      expect(answerComplete(sorting, { kind: "sorting", order })).toBe(true);
      for (const guesses of [{}, { kettle: 2000 }, { "tea-light": 30, kettle: 2000, plant: 1.4e9 }, null, 7]) expect(answerRejection(sorting, { kind: "sorting", order, guesses } as unknown as Answer), JSON.stringify(guesses)).toBe("answer-invalid");
    }
  });

  it("takes a sorting with guesses where the keys are hidden: of presented items, finite, positive on a logarithmic scale, in non-decreasing order", () => {
    for (const challenge of HIDDEN) {
      const sorting = task(challenge, "sorting");
      const valid: readonly (Readonly<Record<string, number>> | undefined)[] = [undefined, {}, { kettle: 5 }, { "tea-light": 1, plant: 1 }, { "tea-light": 1e-9, kettle: 2, plant: 1e30 }, { "tea-light": 7, kettle: 7, plant: 7 }];
      for (const guesses of valid) expect(answerRejection(sorting, { kind: "sorting", order, ...(guesses ? { guesses } : {}) }), JSON.stringify(guesses)).toBeUndefined();
      const invalid: readonly unknown[] = [{ lamp: 5 }, { kettle: 0 }, { kettle: -3 }, { kettle: Number.NaN }, { kettle: Number.POSITIVE_INFINITY }, { kettle: "5" }, { kettle: null }, { "tea-light": 9, kettle: 3 }, { "tea-light": 9, plant: 8.999 }, [], null, 5, "x"];
      for (const guesses of invalid) expect(answerRejection(sorting, { kind: "sorting", order, guesses } as unknown as Answer), JSON.stringify(guesses)).toBe("answer-invalid");
      expect(answerRejection(sorting, { kind: "sorting", order: order.slice(1), guesses: { kettle: 5 } })).toBe("answer-invalid");
    }
  });

  it("allows zero and negative guesses on a linear scale only", () => {
    const linear = structuredClone(physics());
    linear.tasks[1].quantity.scale = "linear";
    for (const challenge of HIDDEN) {
      const sorting = sheetOf(linear as Quiz, 5489, challenge).tasks.find((candidate) => candidate.kind === "sorting")!;
      expect(answerRejection(sorting, { kind: "sorting", order, guesses: { "tea-light": -40, kettle: 0, plant: 0.5 } })).toBeUndefined();
      expect(answerRejection(sorting, { kind: "sorting", order, guesses: { kettle: Number.NEGATIVE_INFINITY } })).toBe("answer-invalid");
      const matching = task(challenge, "matching");
      expect(answerRejection(matching, { kind: "matching", guesses: { "u-value": { "old-wall": -1, "passive-wall": 0 } } })).toBeUndefined();
      expect(answerRejection(matching, { kind: "matching", guesses: { demand: { "old-wall": 0 } } })).toBe("answer-invalid");
      expect(answerRejection(matching, { kind: "matching", guesses: { demand: { "old-wall": -1 } } })).toBe("answer-invalid");
    }
  });

  it("completes a sorting whose keys are hidden only with a guess for every item", () => {
    for (const challenge of HIDDEN) {
      const sorting = task(challenge, "sorting");
      expect(answerComplete(sorting)).toBe(false);
      expect(answerComplete(sorting, { kind: "sorting", order })).toBe(false);
      expect(answerComplete(sorting, { kind: "sorting", order, guesses: {} })).toBe(false);
      expect(answerComplete(sorting, { kind: "sorting", order, guesses: { "tea-light": 30, plant: 1e9 } })).toBe(false);
      expect(answerComplete(sorting, { kind: "sorting", order, guesses: { "tea-light": 30, kettle: 2000, plant: 1e9 } })).toBe(true);
      expect(answerComplete(sorting, { kind: "matching", guesses: {} })).toBe(false);
    }
  });

  it("takes card assignments and no guesses for a matching where the keys show", () => {
    for (const challenge of SHOWN) {
      const matching = task(challenge, "matching");
      expect(answerRejection(matching, { kind: "matching", assignments: cards })).toBeUndefined();
      expect(answerRejection(matching, { kind: "matching", assignments: {} })).toBeUndefined();
      expect(answerComplete(matching, { kind: "matching", assignments: cards })).toBe(true);
      expect(answerRejection(matching, { kind: "matching" })).toBe("answer-invalid");
      expect(answerRejection(matching, { kind: "matching", guesses: guessed })).toBe("answer-invalid");
      expect(answerRejection(matching, { kind: "matching", guesses: {} })).toBe("answer-invalid");
      expect(answerRejection(matching, { kind: "matching", assignments: cards, guesses: guessed })).toBe("answer-invalid");
      expect(answerRejection(matching, { kind: "matching", assignments: cards, guesses: {} })).toBe("answer-invalid");
      expect(answerComplete(matching, { kind: "matching", guesses: guessed })).toBe(false);
    }
  });

  it("takes guesses and no card assignments for a matching where the keys are hidden: of presented dimensions and items, finite, positive on a logarithmic scale", () => {
    for (const challenge of HIDDEN) {
      const matching = task(challenge, "matching");
      const valid: readonly Answer[] = [{ kind: "matching" }, { kind: "matching", guesses: {} }, { kind: "matching", guesses: { demand: {} } }, { kind: "matching", guesses: { demand: { "old-wall": 1e-12 } } }, { kind: "matching", guesses: guessed }, { kind: "matching", guesses: { "u-value": { "old-wall": 3, "passive-wall": 3 } } }];
      for (const answer of valid) expect(answerRejection(matching, answer), JSON.stringify(answer)).toBeUndefined();
      const invalid: readonly unknown[] = [
        { kind: "matching", assignments: cards },
        { kind: "matching", assignments: {} },
        { kind: "matching", assignments: {}, guesses: guessed },
        { kind: "matching", guesses: { cost: { "old-wall": 1 } } },
        { kind: "matching", guesses: { demand: { attic: 1 } } },
        { kind: "matching", guesses: { demand: { "old-wall": 0 } } },
        { kind: "matching", guesses: { demand: { "old-wall": Number.NaN } } },
        { kind: "matching", guesses: { "u-value": { "old-wall": Number.POSITIVE_INFINITY } } },
        { kind: "matching", guesses: { "u-value": { "old-wall": "1.4" } } },
        { kind: "matching", guesses: { "u-value": 1.4 } },
        { kind: "matching", guesses: [] },
        { kind: "matching", guesses: null },
        { kind: "sorting", order: [], guesses: {} },
      ];
      for (const answer of invalid) expect(answerRejection(matching, answer as Answer), JSON.stringify(answer)).toBe("answer-invalid");
    }
  });

  it("completes a matching whose keys are hidden only with a guess for every item in every dimension", () => {
    for (const challenge of HIDDEN) {
      const matching = task(challenge, "matching");
      expect(answerComplete(matching)).toBe(false);
      expect(answerComplete(matching, { kind: "matching" })).toBe(false);
      expect(answerComplete(matching, { kind: "matching", guesses: {} })).toBe(false);
      expect(answerComplete(matching, { kind: "matching", guesses: { "u-value": guessed["u-value"] } })).toBe(false);
      expect(answerComplete(matching, { kind: "matching", guesses: { ...guessed, demand: { "old-wall": 300 } } })).toBe(false);
      expect(answerComplete(matching, { kind: "matching", guesses: guessed })).toBe(true);
      expect(answerComplete(matching, { kind: "matching", assignments: cards })).toBe(false);
    }
  });

  it("validates and completes a classification alike at every challenge", () => {
    for (const challenge of CHALLENGES) {
      const classification = task(challenge, "classification");
      const drawn = classification.items.map((item) => item.id);
      expect(answerRejection(classification, { kind: "classification", assignments: {} })).toBeUndefined();
      expect(answerRejection(classification, { kind: "classification", assignments: { [drawn[0]!]: "modern" } })).toBe("answer-invalid");
      expect(answerComplete(classification, { kind: "classification", assignments: { [drawn[0]!]: "old" } })).toBe(false);
      expect(answerComplete(classification, { kind: "classification", assignments: Object.fromEntries(drawn.map((id) => [id, "old"])) })).toBe(true);
    }
  });
});

describe("the contract of a run at every challenge — what the core emits is what the schema admits", () => {
  const definition = (name: string) => AJV.getSchema(`${SCHEMA.$id}#/$defs/${name}`)!;
  const admits = (name: string, value: unknown): void => {
    const validate = definition(name);
    expect(validate(JSON.parse(JSON.stringify(value))), `${name}: ${JSON.stringify(validate.errors)}`).toBe(true);
  };
  const quiz = physics() as Quiz;
  const [learner, run, command] = ["1".repeat(32), "2".repeat(32), "3".repeat(32)];
  const context = (now: number): LearnerContext => ({ now, catalog: catalog() as Catalog, quizzes: { physics: { quiz, revision: "a".repeat(64) } }, limits: DEFAULT_LIMITS });

  /** 🎬️ A whole run at a challenge: every command, every event it decided, and the state after each. */
  function played(challenge: Challenge): { readonly commands: Command[]; readonly events: Event[]; readonly states: LearnerState[] } {
    const sheet = sheetOf(quiz, runSeed(run), challenge);
    const { keys, timed } = challengeRules(challenge);
    const answers = sheet.tasks.map((sheetTask): Answer => {
      const ids = sheetTask.items.map((item) => item.id);
      if (sheetTask.kind === "classification") return { kind: "classification", assignments: Object.fromEntries(ids.slice(timed ? 1 : 0).map((id) => [id, "old"])) };
      if (sheetTask.kind === "sorting") return keys ? { kind: "sorting", order: ids } : { kind: "sorting", order: ids, guesses: Object.fromEntries(ids.slice(0, timed ? 1 : ids.length).map((id) => [id, 5e12])) };
      return keys ? { kind: "matching", assignments: Object.fromEntries(sheetTask.dimensions.map((dimension) => [dimension.id, Object.fromEntries(ids.map((id, index) => [id, index]))])) } : { kind: "matching", guesses: Object.fromEntries(sheetTask.dimensions.map((dimension) => [dimension.id, Object.fromEntries(ids.slice(0, timed ? 1 : ids.length).map((id, index) => [id, 1 + index]))])) };
    });
    const commands: Command[] = [
      { type: "identify-learner", id: command, learner, identity: { kind: "anonymous" } },
      { type: "start-run", id: command, learner, run, quiz: "physics", challenge, at: 500 },
      ...sheet.tasks.flatMap((sheetTask, index): Command[] => [...(timed ? [{ type: "open-task" as const, id: command, learner, run, task: sheetTask.id, at: 1_000 + index }] : []), { type: "record-answer", id: command, learner, run, task: sheetTask.id, answer: answers[index]!, at: 2_000 + index }]),
      { type: "submit-run", id: command, learner, run },
    ];
    const events: Event[] = [];
    const states: LearnerState[] = [emptyLearnerState(learner)];
    for (const [index, step] of commands.entries()) {
      const decision = decideLearner(states.at(-1)!, step, context(1_000 + index * 500));
      if ("rejection" in decision) throw new Error(`${challenge} ${step.type}: ${decision.rejection}`);
      events.push(...decision.events);
      states.push(decision.events.reduce(evolveLearner, states.at(-1)!));
    }
    return { commands, events, states };
  }

  it("admits every sheet, with its keys, cards, axes and seconds as the challenge has them", () => {
    for (const challenge of CHALLENGES) for (let seed = 0; seed < 25; seed++) admits("Sheet", sheetOf(quiz, seed, challenge));
    expect(definition("Sheet")({ ...sheetOf(quiz, 1, "medium"), challenge: undefined })).toBe(false);
    expect(definition("Sheet")({ ...sheetOf(quiz, 1, "medium"), challenge: "legendary" })).toBe(false);
    expect(definition("SheetAxis")({ id: "heating", label: T("Heating"), unit: "kWh" })).toBe(false);
    expect(definition("SheetAxis")({ id: "heating", label: T("Heating"), min: 0, max: 1 })).toBe(false);
    expect(definition("SheetAxis")({ id: "heating", label: T("Heating") })).toBe(true);
    expect(definition("SheetAxis")({ id: "heating", label: T("Heating"), unit: "kWh", min: 0, max: 1 })).toBe(true);
  });

  it("admits every command, event, result and view of a whole run at every challenge", () => {
    for (const challenge of CHALLENGES) {
      const { commands, events, states } = played(challenge);
      for (const step of commands) admits("Command", step);
      for (const event of events) admits("Event", event);
      expect(events.map((event) => event.type)).toContain("run-submitted");
      expect(events.some((event) => event.type === "task-opened")).toBe(challenge === "expert");
      for (const event of events) if (event.type === "run-submitted") admits("RunResult", event.result);
      const view = catalogView(catalog() as Catalog, [quiz]);
      for (const state of states.slice(2)) {
        admits("RunView", runView(state, run, context(0).quizzes));
        admits("LearnerView", learnerView(state, view));
      }
      const record = transcript(states.at(-1)!)!;
      admits("Leaderboard", leaderboard([record], view, { period: "all-time" }, 0, learner));
      admits("CrowdView", crowdView(quiz, [states.at(-1)!.runs[0]!.result!]));
    }
  });

  it("admits the hints of an easy run and refuses what the contract excludes", () => {
    const { states } = played("easy");
    const open = states.at(-2)!;
    const hinted = runView(open, run, context(0).quizzes)!;
    expect(hinted.hints).toBeDefined();
    admits("RunView", hinted);
    for (const hints of Object.values(hinted.hints!)) for (const hint of hints) admits("Hint", hint);
    expect(definition("Hint")({ kind: "misplaced", count: 1 })).toBe(false);
    expect(definition("Hint")({ kind: "magnitude", item: "kettle", direction: "high" })).toBe(false);
    for (const verdict of VERDICTS) expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", factor: 0.001, verdict }), verdict).toBe(true);
    expect(definition("Verdict")("sideways")).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "old-wall", other: "passive-wall", dimension: "u-value", difference: -0.5, verdict: "over" })).toBe(true);
    expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", factor: 0.001, under: true })).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", factor: 0.001, verdict: "sideways" })).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", factor: 0.001 })).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", factor: 2, difference: 1, verdict: "under" })).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", verdict: "under" })).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "kettle", other: "plant", factor: 0, verdict: "under" })).toBe(false);
    expect(definition("Hint")({ kind: "compare", item: "kettle", factor: 2, verdict: "under" })).toBe(false);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old", axis: "heating" })).toBe(true);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old", axis: "heating", other: "villa", above: true })).toBe(true);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old", axis: "heating", other: "villa", above: false })).toBe(true);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old", axis: "heating", other: "villa" })).toBe(false);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old", axis: "heating", above: true })).toBe(false);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old", axis: "heating", other: "villa", above: "yes" })).toBe(false);
    expect(definition("Hint")({ kind: "profile", item: "kettle", category: "old" })).toBe(false);
    expect(definition("SheetItem")({ id: "kettle", label: T("Kettle"), short: T("Kettle") })).toBe(true);
    expect(definition("SheetItem")({ id: "kettle", label: T("Kettle"), short: T("k".repeat(41)) })).toBe(false);
    expect(definition("SheetItem")({ id: "kettle", label: T("Kettle"), familiar: true })).toBe(false);
    expect(definition("SheetAxis")({ id: "heating", label: T("Heating"), short: T("heating") })).toBe(true);
    expect(definition("ShortText")(T("k".repeat(40), "🧲".repeat(40)))).toBe(true);
    expect(definition("ShortText")(T("k".repeat(41)))).toBe(false);
    expect(definition("Hint")({ kind: "group", item: "kettle", other: "plant", together: false })).toBe(true);
    expect(definition("Hint")({ kind: "group", item: "kettle", other: "plant", together: "no" })).toBe(false);
    expect(definition("Hint")({ kind: "category", item: "kettle", category: "old" })).toBe(true);
    expect(definition("Hint")({ kind: "category", item: "kettle", category: "old", count: 1 })).toBe(false);
    expect(definition("Quantity")({ label: T("Power"), unit: "W", scale: "logarithmic", prefixed: true, additive: true })).toBe(true);
    expect(definition("Quantity")({ label: T("Power"), unit: "W", scale: "logarithmic", prefixed: true })).toBe(false);
    expect(definition("RunView")({ ...hinted, hints: {} })).toBe(false);
    expect(definition("Rejection")("time-up") && definition("Rejection")("task-unopened") && definition("Rejection")("run-untimed") && definition("Rejection")("already-opened")).toBe(true);
    expect(definition("Command")({ type: "start-run", id: command, learner, run, quiz: "physics", at: 1 })).toBe(false);
    expect(definition("Command")({ type: "start-run", id: command, learner, run, quiz: "physics", challenge: "hard" })).toBe(false);
    expect(definition("Command")({ type: "start-run", id: command, learner, run, quiz: "physics", challenge: "hard", at: 2 ** 53 - 1 })).toBe(true);
    expect(definition("Command")({ type: "start-run", id: command, learner, run, quiz: "physics", challenge: "hard", at: 2 ** 53 })).toBe(false);
    expect(definition("Command")({ type: "record-answer", id: command, learner, run, task: "power", answer: { kind: "sorting", order: [] } })).toBe(false);
    expect(definition("Command")({ type: "open-task", id: command, learner, run, task: "power" })).toBe(false);
    expect(definition("Best")({ challenge: "hard", score: 0.5, points: 150 })).toBe(true);
    expect(definition("Best")({ challenge: "hard", score: 0.5 })).toBe(false);
    for (const rejection of REJECTIONS) expect(definition("Rejection")(rejection), rejection).toBe(true);
    for (const challenge of CHALLENGES) expect(definition("Challenge")(challenge), challenge).toBe(true);
    expect(definition("Challenge")("legendary")).toBe(false);
  });
});
