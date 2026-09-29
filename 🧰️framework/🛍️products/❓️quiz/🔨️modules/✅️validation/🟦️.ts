/** ✅️ Structural and semantic validation of quizzes, catalogs, answers, handles and presence states without any schema library.
 *
 * Issues carry a JSON pointer into the validated document and a stable kebab-case code; they are returned
 * deduplicated and sorted by path, then code, in Unicode code point order.
 *
 * @see ../../🧬️schema/🔣️.json — the structure checked here
 * @see ../../README.md — the issue code table
 * @see ./🦀️.rs — the Rust twin
 */
import { SCALES, SCREENS, TASK_KINDS, type Answer, type Quiz, type Rejection, type SheetTask } from "../../🧬️schema/🟦️.ts";

/** 🩺️ One finding: where (JSON pointer) and what (kebab-case code). */
export type ValidationIssue = { readonly path: string; readonly code: string };

type Json = Readonly<Record<string, unknown>>;
type Report = (path: string, code: string) => void;

/** 🪪️ A normalised handle: what learners see and the key the roster indexes. */
export type NormalizedHandle = { readonly display: string; readonly key: string };

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const TAG = /^[0-9a-f]{8}$/u;
const ANCHOR = /^[a-z0-9]+(?:[:-][a-z0-9]+)*$/u;
const WHITESPACE = /\p{White_Space}+/gu;
const HANDLE_MAX = 64;
const IDENTITY_KINDS = ["anonymous", "pseudonym", "name"] as const;
const QUIZ_SCHEMA = "semio.quiz/v1";
const CATALOG_SCHEMA = "semio.quiz.catalog/v1";
const BADGE_RULE_KINDS = ["perfect-quiz", "perfect-tasks", "completed-quizzes"] as const;

/** 🧷️ A JSON pointer one step below `base`, escaping `~` and `/`. */
function at(base: string, key: string | number): string {
  return `${base}/${String(key).replaceAll("~", "~0").replaceAll("/", "~1")}`;
}

/** 🔠️ Code point order, identical to UTF-8 byte order; the order of issues and of emitted map keys. */
export function compareCodePoints(left: string, right: string): number {
  const a = [...left];
  const b = [...right];
  for (let i = 0; i < Math.min(a.length, b.length); i++) {
    const difference = a[i]!.codePointAt(0)! - b[i]!.codePointAt(0)!;
    if (difference !== 0) return difference;
  }
  return a.length - b.length;
}

/** 🧹️ Runs `check` with a reporter and returns its issues deduplicated and sorted. */
function collect(check: (report: Report) => void): ValidationIssue[] {
  const seen = new Map<string, ValidationIssue>();
  check((path, code) => seen.set(`${path}\u0000${code}`, { path, code }));
  return [...seen.values()].sort((left, right) => compareCodePoints(left.path, right.path) || compareCodePoints(left.code, right.code));
}

/** 🧱️ Whether `value` is a JSON object. */
function isObject(value: unknown): value is Json {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** 📦️ An object with exactly the given required and optional properties, or `undefined` after reporting `type-invalid`. */
function object(value: unknown, path: string, report: Report, required: readonly string[], optional: readonly string[] = []): Json | undefined {
  if (!isObject(value)) return void report(path, "type-invalid");
  for (const key of required) if (!Object.hasOwn(value, key)) report(at(path, key), "required");
  for (const key of Object.keys(value)) if (!required.includes(key) && !optional.includes(key)) report(at(path, key), "property-unknown");
  return value;
}

/** 🔡️ A string whose length in code points lies in `[min, max]`. */
function string(value: unknown, path: string, report: Report, min = 0, max = Infinity): value is string {
  if (typeof value !== "string") return report(path, "type-invalid"), false;
  const length = [...value].length;
  if (length < min || length > max) return report(path, "length-invalid"), false;
  return true;
}

/** 🏷️ A slug. */
function slug(value: unknown, path: string, report: Report): value is string {
  if (typeof value !== "string") return report(path, "type-invalid"), false;
  if (value.length > 64 || !SLUG.test(value)) return report(path, "slug-invalid"), false;
  return true;
}

/** 🔢️ A finite number. */
function number(value: unknown, path: string, report: Report): value is number {
  if (typeof value !== "number" || !Number.isFinite(value)) return report(path, "type-invalid"), false;
  return true;
}

/** 🧮️ An integer of at least `minimum`. */
function integer(value: unknown, path: string, report: Report, minimum: number): value is number {
  if (!number(value, path, report)) return false;
  if (!Number.isInteger(value)) return report(path, "integer-invalid"), false;
  if (value < minimum) return report(path, "below-minimum"), false;
  return true;
}

/** ☯️ A boolean. */
function boolean(value: unknown, path: string, report: Report): void {
  if (typeof value !== "boolean") report(path, "type-invalid");
}

/** 📌️ One of the allowed literal values. */
function literal(value: unknown, path: string, report: Report, allowed: readonly string[]): value is string {
  if (typeof value !== "string" || !allowed.includes(value)) return report(path, "value-invalid"), false;
  return true;
}

/** 🌍️ A text in every language. */
function text(value: unknown, path: string, report: Report): void {
  const json = object(value, path, report, ["en", "de"]);
  if (json) for (const language of ["en", "de"]) if (Object.hasOwn(json, language)) string(json[language], at(path, language), report, 1);
}

/** 🗄️ An array of at least `minItems` entries, each checked by `each`. */
function array(value: unknown, path: string, report: Report, minItems: number, each: (entry: unknown, path: string) => void): readonly unknown[] | undefined {
  if (!Array.isArray(value)) return void report(path, "type-invalid");
  if (value.length < minItems) report(path, "items-too-few");
  value.forEach((entry, index) => each(entry, at(path, index)));
  return value;
}

/** 🗝️ A slug-keyed map of numbers with at least one entry. */
function numberMap(value: unknown, path: string, report: Report): Json | undefined {
  if (!isObject(value)) return void report(path, "type-invalid");
  if (Object.keys(value).length < 1) report(path, "properties-too-few");
  for (const [key, entry] of Object.entries(value)) if (slug(key, at(path, key), report)) number(entry, at(path, key), report);
  return value;
}

/** 👯️ Reports `duplicate-id` at every repeated `id` of the given entries. */
function uniqueIds(entries: readonly unknown[] | undefined, path: string, report: Report): Set<string> {
  const ids = new Set<string>();
  entries?.forEach((entry, index) => {
    const id = isObject(entry) ? entry.id : undefined;
    if (typeof id !== "string") return;
    if (ids.has(id)) report(at(at(path, index), "id"), "duplicate-id");
    ids.add(id);
  });
  return ids;
}

/** ⚖️ A quantity; returns its scale when valid. */
function quantity(value: unknown, path: string, report: Report): string | undefined {
  const json = object(value, path, report, ["label", "unit", "scale", "prefixed"]);
  if (!json) return undefined;
  if (Object.hasOwn(json, "label")) text(json.label, at(path, "label"), report);
  if (Object.hasOwn(json, "unit")) string(json.unit, at(path, "unit"), report, 1, 32);
  if (Object.hasOwn(json, "prefixed")) boolean(json.prefixed, at(path, "prefixed"), report);
  return Object.hasOwn(json, "scale") && literal(json.scale, at(path, "scale"), report, SCALES) ? json.scale : undefined;
}

/** 🪧️ The shared head of an item: id, label and optional explanation. */
function itemHead(json: Json, path: string, report: Report): void {
  if (Object.hasOwn(json, "id")) slug(json.id, at(path, "id"), report);
  if (Object.hasOwn(json, "label")) text(json.label, at(path, "label"), report);
  if (Object.hasOwn(json, "explanation")) text(json.explanation, at(path, "explanation"), report);
}

/** 🎲️ `draw` is an integer ≥ 2 that does not exceed the item count. */
function draw(json: Json, path: string, report: Report, items: readonly unknown[] | undefined): void {
  if (!Object.hasOwn(json, "draw") || !integer(json.draw, at(path, "draw"), report, 2)) return;
  if (items && (json.draw as number) > items.length) report(at(path, "draw"), "draw-exceeds-items");
}

/** 🗃️ A classification task: categories, optional axes and profiles, items referencing categories. */
function classificationTask(json: Json, path: string, report: Report): void {
  const axes = Object.hasOwn(json, "axes")
    ? array(json.axes, at(path, "axes"), report, 3, (entry, entryPath) => {
        const axis = object(entry, entryPath, report, ["id", "label", "unit", "min", "max"]);
        if (!axis) return;
        if (Object.hasOwn(axis, "id")) slug(axis.id, at(entryPath, "id"), report);
        if (Object.hasOwn(axis, "label")) text(axis.label, at(entryPath, "label"), report);
        if (Object.hasOwn(axis, "unit")) string(axis.unit, at(entryPath, "unit"), report, 1, 32);
        const min = Object.hasOwn(axis, "min") && number(axis.min, at(entryPath, "min"), report);
        const max = Object.hasOwn(axis, "max") && number(axis.max, at(entryPath, "max"), report);
        if (min && max && (axis.max as number) <= (axis.min as number)) report(at(entryPath, "max"), "axis-range-invalid");
      })
    : undefined;
  const axisIds = uniqueIds(axes, at(path, "axes"), report);
  const axisRanges = new Map<string, readonly [number, number] | undefined>();
  axes?.forEach((axis) => {
    if (!isObject(axis) || typeof axis.id !== "string" || axisRanges.has(axis.id)) return;
    axisRanges.set(axis.id, typeof axis.min === "number" && typeof axis.max === "number" && axis.max > axis.min ? [axis.min, axis.max] : undefined);
  });
  const categories = Object.hasOwn(json, "categories")
    ? array(json.categories, at(path, "categories"), report, 2, (entry, entryPath) => {
        const category = object(entry, entryPath, report, ["id", "label"], ["description", "profile"]);
        if (!category) return;
        if (Object.hasOwn(category, "id")) slug(category.id, at(entryPath, "id"), report);
        if (Object.hasOwn(category, "label")) text(category.label, at(entryPath, "label"), report);
        if (Object.hasOwn(category, "description")) text(category.description, at(entryPath, "description"), report);
        if (!Object.hasOwn(category, "profile")) return;
        const profilePath = at(entryPath, "profile");
        const profile = numberMap(category.profile, profilePath, report);
        if (!profile) return;
        if (!Object.hasOwn(json, "axes")) return report(profilePath, "axes-missing");
        if (!axes) return;
        for (const axisId of axisIds) if (!Object.hasOwn(profile, axisId)) report(at(profilePath, axisId), "profile-incomplete");
        for (const [key, value] of Object.entries(profile)) {
          if (!axisIds.has(key)) report(at(profilePath, key), "axis-unknown");
          const range = axisRanges.get(key);
          if (range && typeof value === "number" && (value < range[0] || value > range[1])) report(at(profilePath, key), "profile-out-of-range");
        }
      })
    : undefined;
  const categoryIds = uniqueIds(categories, at(path, "categories"), report);
  const items = Object.hasOwn(json, "items")
    ? array(json.items, at(path, "items"), report, 2, (entry, entryPath) => {
        const item = object(entry, entryPath, report, ["id", "label", "category"], ["explanation"]);
        if (!item) return;
        itemHead(item, entryPath, report);
        if (Object.hasOwn(item, "category") && slug(item.category, at(entryPath, "category"), report) && categories && !categoryIds.has(item.category)) report(at(entryPath, "category"), "category-unknown");
      })
    : undefined;
  uniqueIds(items, at(path, "items"), report);
  draw(json, path, report, items);
}

/** 📶️ A sorting task: a quantity and items with values, positive on a logarithmic scale. */
function sortingTask(json: Json, path: string, report: Report): void {
  const scale = Object.hasOwn(json, "quantity") ? quantity(json.quantity, at(path, "quantity"), report) : undefined;
  const items = Object.hasOwn(json, "items")
    ? array(json.items, at(path, "items"), report, 2, (entry, entryPath) => {
        const item = object(entry, entryPath, report, ["id", "label", "value"], ["explanation"]);
        if (!item) return;
        itemHead(item, entryPath, report);
        if (Object.hasOwn(item, "value") && number(item.value, at(entryPath, "value"), report) && scale === "logarithmic" && item.value <= 0) report(at(entryPath, "value"), "value-not-positive");
      })
    : undefined;
  uniqueIds(items, at(path, "items"), report);
  draw(json, path, report, items);
}

/** 🔗️ A matching task: dimensions and items carrying a value for exactly every dimension. */
function matchingTask(json: Json, path: string, report: Report): void {
  const scales = new Map<string, string | undefined>();
  const dimensions = Object.hasOwn(json, "dimensions")
    ? array(json.dimensions, at(path, "dimensions"), report, 1, (entry, entryPath) => {
        const dimension = object(entry, entryPath, report, ["id", "quantity"]);
        if (!dimension) return;
        const scale = Object.hasOwn(dimension, "quantity") ? quantity(dimension.quantity, at(entryPath, "quantity"), report) : undefined;
        if (Object.hasOwn(dimension, "id") && slug(dimension.id, at(entryPath, "id"), report) && !scales.has(dimension.id)) scales.set(dimension.id, scale);
      })
    : undefined;
  uniqueIds(dimensions, at(path, "dimensions"), report);
  const items = Object.hasOwn(json, "items")
    ? array(json.items, at(path, "items"), report, 2, (entry, entryPath) => {
        const item = object(entry, entryPath, report, ["id", "label", "values"], ["explanation"]);
        if (!item) return;
        itemHead(item, entryPath, report);
        if (!Object.hasOwn(item, "values")) return;
        const valuesPath = at(entryPath, "values");
        const values = numberMap(item.values, valuesPath, report);
        if (!values || !dimensions) return;
        for (const dimensionId of scales.keys()) if (!Object.hasOwn(values, dimensionId)) report(at(valuesPath, dimensionId), "value-missing");
        for (const [key, value] of Object.entries(values)) {
          if (!scales.has(key)) report(at(valuesPath, key), "dimension-unknown");
          else if (scales.get(key) === "logarithmic" && typeof value === "number" && value <= 0) report(at(valuesPath, key), "value-not-positive");
        }
      })
    : undefined;
  uniqueIds(items, at(path, "items"), report);
  draw(json, path, report, items);
}

/** 🧩️ A task, dispatched on its kind. */
function task(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, TASK_KINDS)) return;
  const shared = ["kind", "id", "title", "prompt"];
  const json =
    value.kind === "classification"
      ? object(value, path, report, [...shared, "categories", "items"], ["axes", "draw"])
      : value.kind === "sorting"
        ? object(value, path, report, [...shared, "quantity", "items"], ["draw"])
        : object(value, path, report, [...shared, "dimensions", "items"], ["draw"]);
  if (!json) return;
  if (Object.hasOwn(json, "id")) slug(json.id, at(path, "id"), report);
  if (Object.hasOwn(json, "title")) text(json.title, at(path, "title"), report);
  if (Object.hasOwn(json, "prompt")) text(json.prompt, at(path, "prompt"), report);
  if (value.kind === "classification") classificationTask(json, path, report);
  else if (value.kind === "sorting") sortingTask(json, path, report);
  else matchingTask(json, path, report);
}

/** 📝️ Every issue of a quiz document: structure per the schema, unique ids, references, positive logarithmic values, complete profiles, matching values for every dimension and draws within the item count. */
export function quizIssues(quiz: unknown): ValidationIssue[] {
  return collect((report) => {
    const json = object(quiz, "", report, ["schema", "id", "emoji", "title", "description", "tasks"], ["$schema"]);
    if (!json) return;
    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);
    if (Object.hasOwn(json, "schema") && json.schema !== QUIZ_SCHEMA) report("/schema", "value-invalid");
    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);
    if (Object.hasOwn(json, "emoji")) string(json.emoji, "/emoji", report, 1, 16);
    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);
    if (Object.hasOwn(json, "description")) text(json.description, "/description", report);
    const tasks = Object.hasOwn(json, "tasks") ? array(json.tasks, "/tasks", report, 1, (entry, path) => task(entry, path, report)) : undefined;
    uniqueIds(tasks, "/tasks", report);
  });
}

/** 📜️ A badge rule; returns it when structurally valid. */
function badgeRule(value: unknown, path: string, report: Report): Json | undefined {
  if (!isObject(value)) return void report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return void report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, BADGE_RULE_KINDS)) return undefined;
  const json = value.kind === "perfect-quiz" ? object(value, path, report, ["kind", "quiz"]) : value.kind === "perfect-tasks" ? object(value, path, report, ["kind"], ["taskKind", "quiz"]) : object(value, path, report, ["kind"]);
  if (!json) return undefined;
  const quizValid = !Object.hasOwn(json, "quiz") || slug(json.quiz, at(path, "quiz"), report);
  const kindValid = !Object.hasOwn(json, "taskKind") || literal(json.taskKind, at(path, "taskKind"), report, TASK_KINDS);
  return quizValid && kindValid && !(value.kind === "perfect-quiz" && !Object.hasOwn(json, "quiz")) ? json : undefined;
}

/** 📚️ Every issue of a catalog document given its quizzes in catalog order: structure per the schema, unique paths, quiz ids and badge ids, and badge rules that reference existing quizzes and select at least one task. */
export function catalogIssues(catalog: unknown, quizzes: readonly Quiz[]): ValidationIssue[] {
  return collect((report) => {
    const json = object(catalog, "", report, ["schema", "id", "title", "introduction", "quizzes", "badges"], ["$schema"]);
    if (!json) return;
    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);
    if (Object.hasOwn(json, "schema") && json.schema !== CATALOG_SCHEMA) report("/schema", "value-invalid");
    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);
    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);
    if (Object.hasOwn(json, "introduction")) {
      const introduction = object(json.introduction, "/introduction", report, ["title", "paragraphs"]);
      if (introduction && Object.hasOwn(introduction, "title")) text(introduction.title, "/introduction/title", report);
      if (introduction && Object.hasOwn(introduction, "paragraphs")) array(introduction.paragraphs, "/introduction/paragraphs", report, 1, (entry, path) => text(entry, path, report));
    }
    const paths = new Set<string>();
    const quizPaths = Object.hasOwn(json, "quizzes")
      ? array(json.quizzes, "/quizzes", report, 1, (entry, path) => {
          if (!string(entry, path, report, 1)) return;
          if (paths.has(entry)) report(path, "duplicate-path");
          paths.add(entry);
        })
      : undefined;
    if (quizPaths && quizPaths.length !== quizzes.length) report("/quizzes", "quiz-count-mismatch");
    const quizIds = new Set<string>();
    quizzes.forEach((quiz, index) => {
      if (quizIds.has(quiz.id)) report(at("/quizzes", index), "duplicate-id");
      quizIds.add(quiz.id);
    });
    const badges = Object.hasOwn(json, "badges")
      ? array(json.badges, "/badges", report, 0, (entry, path) => {
          const badge = object(entry, path, report, ["id", "emoji", "label", "description", "rule"]);
          if (!badge) return;
          if (Object.hasOwn(badge, "id")) slug(badge.id, at(path, "id"), report);
          if (Object.hasOwn(badge, "emoji")) string(badge.emoji, at(path, "emoji"), report, 1, 16);
          if (Object.hasOwn(badge, "label")) text(badge.label, at(path, "label"), report);
          if (Object.hasOwn(badge, "description")) text(badge.description, at(path, "description"), report);
          if (!Object.hasOwn(badge, "rule")) return;
          const rulePath = at(path, "rule");
          const rule = badgeRule(badge.rule, rulePath, report);
          if (!rule || rule.kind === "completed-quizzes") return;
          if (typeof rule.quiz === "string" && !quizIds.has(rule.quiz)) return report(at(rulePath, "quiz"), "quiz-unknown");
          if (rule.kind === "perfect-tasks" && !quizzes.some((quiz) => (rule.quiz === undefined || quiz.id === rule.quiz) && quiz.tasks.some((task) => rule.taskKind === undefined || task.kind === rule.taskKind))) report(rulePath, "badge-unreachable");
        })
      : undefined;
    uniqueIds(badges, "/badges", report);
  });
}

/** ✂️ The display handle (trimmed, inner Unicode whitespace runs collapsed to one space) and its key (lowercased), or `undefined` outside 1…64 code points. */
export function normalizeHandle(handle: string): NormalizedHandle | undefined {
  const display = handle.replace(WHITESPACE, " ").replace(/^ | $/gu, "");
  const length = [...display].length;
  return length >= 1 && length <= HANDLE_MAX ? { display, key: display.toLowerCase() } : undefined;
}

/** 🔰️ Whether `value` is a public learner tag: 8 lowercase hex digits. */
export function isTag(value: string): boolean {
  return TAG.test(value);
}

/** ⛵️ Whether `value` is an anchor: `^[a-z0-9]+(?:[:-][a-z0-9]+)*$` and at most 64 characters. */
export function isAnchor(value: string): boolean {
  return value.length <= 64 && ANCHOR.test(value);
}

/** 🔖️ A public learner tag (`tag-invalid`). */
function tag(value: unknown, path: string, report: Report): void {
  if (typeof value !== "string") return report(path, "type-invalid");
  if (!isTag(value)) report(path, "tag-invalid");
}

/** ⚓️ An anchor key (`anchor-invalid`). */
function anchor(value: unknown, path: string, report: Report): void {
  if (typeof value !== "string") return report(path, "type-invalid");
  if (!isAnchor(value)) report(path, "anchor-invalid");
}

/** 📐️ A coordinate: a number (`type-invalid`) that is finite and in [0, 1] (`out-of-range`). */
function unit(value: unknown, path: string, report: Report): void {
  if (typeof value !== "number") return report(path, "type-invalid");
  if (!(Number.isFinite(value) && value >= 0 && value <= 1)) report(path, "out-of-range");
}

/** 🎭️ An identity: a known kind, and a handle of 1…64 code points for pseudonyms and names (`length-invalid`). */
function identity(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, IDENTITY_KINDS)) return;
  const json = value.kind === "anonymous" ? object(value, path, report, ["kind"]) : object(value, path, report, ["kind", "handle"]);
  if (json && Object.hasOwn(json, "handle")) string(json.handle, at(path, "handle"), report, 1, HANDLE_MAX);
}

/** 🗺️ A place: a known screen and slugs for quiz and task; on a known screen the quiz page, a run and its results name their quiz (`required`), only they name a quiz (`quiz-outside-run`), only a run names a task (`task-without-run`). */
function place(value: unknown, path: string, report: Report): void {
  const json = object(value, path, report, ["screen"], ["quiz", "task"]);
  if (!json) return;
  if (Object.hasOwn(json, "quiz")) slug(json.quiz, at(path, "quiz"), report);
  if (Object.hasOwn(json, "task")) slug(json.task, at(path, "task"), report);
  if (!Object.hasOwn(json, "screen") || !literal(json.screen, at(path, "screen"), report, SCREENS)) return;
  const inQuiz = json.screen === "quiz" || json.screen === "run" || json.screen === "results";
  if (inQuiz && !Object.hasOwn(json, "quiz")) report(at(path, "quiz"), "required");
  if (!inQuiz && Object.hasOwn(json, "quiz")) report(at(path, "quiz"), "quiz-outside-run");
  if (json.screen !== "run" && Object.hasOwn(json, "task")) report(at(path, "task"), "task-without-run");
}

/** 🟢️ Every issue of a presence state: structure per the schema, tag, identity handle, place. */
export function presenceIssues(state: unknown): ValidationIssue[] {
  return collect((report) => {
    const json = object(state, "", report, ["tag", "identity", "place", "active"]);
    if (!json) return;
    if (Object.hasOwn(json, "tag")) tag(json.tag, "/tag", report);
    if (Object.hasOwn(json, "identity")) identity(json.identity, "/identity", report);
    if (Object.hasOwn(json, "place")) place(json.place, "/place", report);
    if (Object.hasOwn(json, "active")) boolean(json.active, "/active", report);
  });
}

/** 👆️ Every issue of a cursor state: structure per the schema, tag, anchors (cards, `item:<id>`, `category:<id>`), coordinates finite and in [0, 1], the dragged item a slug. */
export function cursorIssues(state: unknown): ValidationIssue[] {
  return collect((report) => {
    const json = object(state, "", report, ["tag"], ["cursor", "focus", "drag"]);
    if (!json) return;
    if (Object.hasOwn(json, "tag")) tag(json.tag, "/tag", report);
    if (Object.hasOwn(json, "focus")) anchor(json.focus, "/focus", report);
    if (Object.hasOwn(json, "drag")) {
      const drag = object(json.drag, "/drag", report, ["item"]);
      if (drag && Object.hasOwn(drag, "item")) slug(drag.item, "/drag/item", report);
    }
    if (!Object.hasOwn(json, "cursor")) return;
    const cursor = object(json.cursor, "/cursor", report, ["anchor", "x", "y"]);
    if (!cursor) return;
    if (Object.hasOwn(cursor, "anchor")) anchor(cursor.anchor, "/cursor/anchor", report);
    if (Object.hasOwn(cursor, "x")) unit(cursor.x, "/cursor/x", report);
    if (Object.hasOwn(cursor, "y")) unit(cursor.y, "/cursor/y", report);
  });
}

/** 🪣️ The most tasks one thinking state names, and the most entries (and the largest card index + 1) one answer or dimension holds. */
export const THINKING_LIMIT = 64;

/** 🧭️ A map whose keys are slugs, each value checked by `each`, at most {@link THINKING_LIMIT} entries (`too-many`). */
function slugMap(value: unknown, path: string, report: Report, each: (entry: unknown, path: string) => void): void {
  if (!isObject(value)) return report(path, "type-invalid");
  const entries = Object.entries(value);
  if (entries.length > THINKING_LIMIT) report(path, "too-many");
  for (const [key, entry] of entries) {
    slug(key, at(path, key), report);
    each(entry, at(path, key));
  }
}

/** ✍️ A draft answer: structurally valid per the schema (kind, members, slugs; matching values finite numbers, `type-invalid` otherwise), possibly partial, with at most {@link THINKING_LIMIT} entries per map or order (`too-many`) and no repeated item in an order (`duplicate-id`). */
function answer(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, TASK_KINDS)) return;
  const member = value.kind === "sorting" ? "order" : value.kind === "matching" ? "values" : "assignments";
  const json = object(value, path, report, ["kind", member]);
  if (!json || !Object.hasOwn(json, member)) return;
  if (value.kind === "sorting") {
    const seen = new Set<unknown>();
    const order = array(json.order, at(path, "order"), report, 0, (entry, entryPath) => {
      slug(entry, entryPath, report);
      if (seen.has(entry)) report(entryPath, "duplicate-id");
      seen.add(entry);
    });
    if (order && order.length > THINKING_LIMIT) report(at(path, "order"), "too-many");
  } else if (value.kind === "classification") slugMap(json.assignments, at(path, "assignments"), report, (entry, entryPath) => slug(entry, entryPath, report));
  else
    slugMap(json.values, at(path, "values"), report, (entry, entryPath) =>
      slugMap(entry, entryPath, report, (number, numberPath) => {
        if (typeof number !== "number" || !Number.isFinite(number)) report(numberPath, "type-invalid");
      }),
    );
}

/** 💭️ Every issue of a thinking state: structure per the schema, tag, and structurally valid (possibly partial) draft answers per task slug within {@link THINKING_LIMIT}. */
export function thinkingIssues(state: unknown): ValidationIssue[] {
  return collect((report) => {
    const json = object(state, "", report, ["tag", "answers"]);
    if (!json) return;
    if (Object.hasOwn(json, "tag")) tag(json.tag, "/tag", report);
    if (Object.hasOwn(json, "answers")) slugMap(json.answers, "/answers", report, (entry, entryPath) => answer(entry, entryPath, report));
  });
}

/** 🚧️ Why an answer is rejected for a sheet task (`answer-invalid`), or `undefined` when it is valid; partial classification and matching answers are valid. */
export function answerRejection(sheetTask: SheetTask, answer: Answer): Rejection | undefined {
  const invalid = "answer-invalid" as const;
  if (!isObject(answer) || answer.kind !== sheetTask.kind) return invalid;
  const items = new Set(sheetTask.items.map((item) => item.id));
  switch (sheetTask.kind) {
    case "classification": {
      const assignments = (answer as Json).assignments;
      if (!isObject(assignments)) return invalid;
      const categories = new Set(sheetTask.categories.map((category) => category.id));
      return Object.entries(assignments).every(([item, category]) => items.has(item) && typeof category === "string" && categories.has(category)) ? undefined : invalid;
    }
    case "sorting": {
      const order = (answer as Json).order;
      if (!Array.isArray(order) || order.length !== items.size) return invalid;
      const seen = new Set<unknown>(order);
      return seen.size === order.length && order.every((item) => typeof item === "string" && items.has(item)) ? undefined : invalid;
    }
    case "matching": {
      const assignments = (answer as Json).assignments;
      if (!isObject(assignments)) return invalid;
      const cards = new Map(sheetTask.dimensions.map((dimension) => [dimension.id, dimension.cards.length]));
      for (const [dimension, perItem] of Object.entries(assignments)) {
        const count = cards.get(dimension);
        if (count === undefined || !isObject(perItem)) return invalid;
        const used = new Set<number>();
        for (const [item, card] of Object.entries(perItem)) {
          if (!items.has(item) || typeof card !== "number" || !Number.isInteger(card) || card < 0 || card >= count || used.has(card)) return invalid;
          used.add(card);
        }
      }
      return undefined;
    }
  }
}

/** 🏁️ Whether an answer completes its sheet task: every item classified, every item matched in every dimension; a recorded sorting is always complete. */
export function answerComplete(sheetTask: SheetTask, answer?: Answer): boolean {
  if (!answer || answer.kind !== sheetTask.kind) return false;
  switch (answer.kind) {
    case "classification":
      return sheetTask.items.every((item) => Object.hasOwn(answer.assignments, item.id));
    case "sorting":
      return true;
    case "matching":
      return (
        sheetTask.kind === "matching" &&
        sheetTask.dimensions.every((dimension) => {
          const perItem = Object.hasOwn(answer.assignments, dimension.id) ? answer.assignments[dimension.id] : undefined;
          return isObject(perItem) && sheetTask.items.every((item) => Object.hasOwn(perItem, item.id));
        })
      );
  }
}
