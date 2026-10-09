/** ✅️ Structural and semantic validation of quizzes, catalogs, answers, ids, handles and presence states without any schema library.
 *
 * Issues carry a JSON pointer into the validated document and a stable kebab-case code; they are returned
 * deduplicated and sorted by path, then code, in Unicode code point order.
 *
 * The handle policy owns its tables: the White_Space set and the Latin letters of `$defs/Handle` are written out here
 * as code point ranges, so both cores accept the same handles whatever Unicode version their runtime ships. Every
 * string over the handle alphabet is in Normalization Form C (it holds no combining mark and no character with a
 * canonical decomposition that recomposes differently), which is how NFC equivalence holds without a normalizer.
 *
 * @see ../../🧬️schema/🔣️.json — the structure checked here, `Handle` for the alphabet
 * @see ../../README.md — the issue code table and the handle policy
 * @see https://www.unicode.org/reports/tr15/ — Unicode normalization forms
 * @see ./🦀️.rs — the Rust twin
 */
import { CHALLENGES, MAX_TIMESTAMP, MOTIONS, SCALES, SCREENS, SHORT_LENGTH, TASK_KINDS, type Answer, type Command, type Query, type Quiz, type Rejection, type Scale, type SheetMatchingTask, type SheetTask, type Timestamp } from "../../🧬️schema/🟦️.ts";

/** 🩺️ One finding: where (JSON pointer) and what (kebab-case code). */
export type ValidationIssue = { readonly path: string; readonly code: string };

type Json = Readonly<Record<string, unknown>>;
type Report = (path: string, code: string) => void;

/** 🪪️ A normalised handle: what learners see and the key the roster indexes. */
export type NormalizedHandle = { readonly display: string; readonly key: string };

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const ID = /^[0-9a-f]{32}$/u;
const TAG = /^[0-9a-f]{8}$/u;
const ANCHOR = /^[a-z0-9]+(?:[:-][a-z0-9]+)*$/u;

/** 📏️ The most code points a handle holds. */
export const HANDLE_MAX = 64;

/** 🧵️ The most code points a handle may be typed with before it is normalized. */
export const HANDLE_INPUT_MAX = 256;

/** ⬜️ The Unicode `White_Space` code points as inclusive ranges: what collapses to one space between the words of a handle. */
export const WHITE_SPACE: readonly (readonly [number, number])[] = [
  [0x0009, 0x000d],
  [0x0020, 0x0020],
  [0x0085, 0x0085],
  [0x00a0, 0x00a0],
  [0x1680, 0x1680],
  [0x2000, 0x200a],
  [0x2028, 0x2029],
  [0x202f, 0x202f],
  [0x205f, 0x205f],
  [0x3000, 0x3000],
];

/** 🔤️ The letters of a handle as inclusive code point ranges: the upper- and lowercase letters of Basic Latin, Latin-1 Supplement, Latin Extended-A, Latin Extended-B and Latin Extended Additional without a compatibility decomposition. */
export const HANDLE_LETTERS: readonly (readonly [number, number])[] = [
  [0x0041, 0x005a],
  [0x0061, 0x007a],
  [0x00c0, 0x00d6],
  [0x00d8, 0x00f6],
  [0x00f8, 0x0131],
  [0x0134, 0x013e],
  [0x0141, 0x0148],
  [0x014a, 0x017e],
  [0x0180, 0x01ba],
  [0x01bc, 0x01bf],
  [0x01cd, 0x01f0],
  [0x01f4, 0x024f],
  [0x1e00, 0x1e99],
  [0x1e9c, 0x1eff],
];

/** ❜️ The punctuation of a handle: apostrophe, hyphen-minus, full stop, underscore. */
export const HANDLE_PUNCTUATION: readonly number[] = [0x27, 0x2d, 0x2e, 0x5f];

const TYPOGRAPHIC_APOSTROPHE = 0x2019;
const APOSTROPHE = 0x27;
const SPACE = 0x20;
const UTF8 = new TextEncoder();
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
  if (!isSlug(value)) return report(path, "slug-invalid"), false;
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

/** 🌍️ A text in every language, each of at most `max` code points. */
function text(value: unknown, path: string, report: Report, max = Infinity): void {
  const json = object(value, path, report, ["en", "de"]);
  if (json) for (const language of ["en", "de"]) if (Object.hasOwn(json, language)) string(json[language], at(path, language), report, 1, max);
}

/** 🩳️ The optional short label of a quantity, an axis, a category or an item: a text of at most {@link SHORT_LENGTH} code points per language. */
function short(json: Json, path: string, report: Report): void {
  if (Object.hasOwn(json, "short")) text(json.short, at(path, "short"), report, SHORT_LENGTH);
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
  const json = object(value, path, report, ["label", "unit", "scale", "prefixed", "additive"], ["short"]);
  if (!json) return undefined;
  if (Object.hasOwn(json, "label")) text(json.label, at(path, "label"), report);
  short(json, path, report);
  if (Object.hasOwn(json, "unit")) string(json.unit, at(path, "unit"), report, 1, 32);
  if (Object.hasOwn(json, "prefixed")) boolean(json.prefixed, at(path, "prefixed"), report);
  if (Object.hasOwn(json, "additive")) boolean(json.additive, at(path, "additive"), report);
  return Object.hasOwn(json, "scale") && literal(json.scale, at(path, "scale"), report, SCALES) ? json.scale : undefined;
}

/** 🪧️ The shared head of an item: id, label and optional short label, icon and explanation. */
function itemHead(json: Json, path: string, report: Report): void {
  if (Object.hasOwn(json, "id")) slug(json.id, at(path, "id"), report);
  if (Object.hasOwn(json, "label")) text(json.label, at(path, "label"), report);
  short(json, path, report);
  if (Object.hasOwn(json, "icon")) icon(json.icon, at(path, "icon"), report);
  if (Object.hasOwn(json, "explanation")) text(json.explanation, at(path, "explanation"), report);
}

/** 🏡️ The optional familiarity of a numeric item: a boolean. */
function familiar(json: Json, path: string, report: Report): void {
  if (Object.hasOwn(json, "familiar")) boolean(json.familiar, at(path, "familiar"), report);
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
        const axis = object(entry, entryPath, report, ["id", "label", "unit", "min", "max"], ["short"]);
        if (!axis) return;
        if (Object.hasOwn(axis, "id")) slug(axis.id, at(entryPath, "id"), report);
        if (Object.hasOwn(axis, "label")) text(axis.label, at(entryPath, "label"), report);
        short(axis, entryPath, report);
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
        const category = object(entry, entryPath, report, ["id", "label"], ["short", "icon", "description", "profile"]);
        if (!category) return;
        if (Object.hasOwn(category, "id")) slug(category.id, at(entryPath, "id"), report);
        if (Object.hasOwn(category, "label")) text(category.label, at(entryPath, "label"), report);
        short(category, entryPath, report);
        if (Object.hasOwn(category, "icon")) icon(category.icon, at(entryPath, "icon"), report);
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
        const item = object(entry, entryPath, report, ["id", "label", "category"], ["short", "icon", "explanation"]);
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
        const item = object(entry, entryPath, report, ["id", "label", "value"], ["short", "icon", "familiar", "explanation"]);
        if (!item) return;
        itemHead(item, entryPath, report);
        familiar(item, entryPath, report);
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
        const dimension = object(entry, entryPath, report, ["id", "quantity"], ["icon"]);
        if (!dimension) return;
        const scale = Object.hasOwn(dimension, "quantity") ? quantity(dimension.quantity, at(entryPath, "quantity"), report) : undefined;
        if (Object.hasOwn(dimension, "icon")) icon(dimension.icon, at(entryPath, "icon"), report);
        if (Object.hasOwn(dimension, "id") && slug(dimension.id, at(entryPath, "id"), report) && !scales.has(dimension.id)) scales.set(dimension.id, scale);
      })
    : undefined;
  uniqueIds(dimensions, at(path, "dimensions"), report);
  const items = Object.hasOwn(json, "items")
    ? array(json.items, at(path, "items"), report, 2, (entry, entryPath) => {
        const item = object(entry, entryPath, report, ["id", "label", "values"], ["short", "icon", "familiar", "explanation"]);
        if (!item) return;
        itemHead(item, entryPath, report);
        familiar(item, entryPath, report);
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

/** 🖼️ An icon: an emoji of 1…16 code points and one of the motions. */
function icon(value: unknown, path: string, report: Report): void {
  const json = object(value, path, report, ["emoji", "motion"]);
  if (!json) return;
  if (Object.hasOwn(json, "emoji")) string(json.emoji, at(path, "emoji"), report, 1, 16);
  if (Object.hasOwn(json, "motion")) literal(json.motion, at(path, "motion"), report, MOTIONS);
}

/** 🧩️ A task, dispatched on its kind. */
function task(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, TASK_KINDS)) return;
  const shared = ["kind", "id", "title", "prompt"];
  const json =
    value.kind === "classification"
      ? object(value, path, report, [...shared, "categories", "items"], ["axes", "draw", "icon"])
      : value.kind === "sorting"
        ? object(value, path, report, [...shared, "quantity", "items"], ["draw", "icon"])
        : object(value, path, report, [...shared, "dimensions", "items"], ["draw", "icon"]);
  if (!json) return;
  if (Object.hasOwn(json, "id")) slug(json.id, at(path, "id"), report);
  if (Object.hasOwn(json, "title")) text(json.title, at(path, "title"), report);
  if (Object.hasOwn(json, "prompt")) text(json.prompt, at(path, "prompt"), report);
  if (Object.hasOwn(json, "icon")) icon(json.icon, at(path, "icon"), report);
  if (value.kind === "classification") classificationTask(json, path, report);
  else if (value.kind === "sorting") sortingTask(json, path, report);
  else matchingTask(json, path, report);
}

/** 📝️ Every issue of a quiz document: structure per the schema, unique ids, references, positive logarithmic values, complete profiles, matching values for every dimension and draws within the item count. */
export function quizIssues(quiz: unknown): ValidationIssue[] {
  return collect((report) => {
    const json = object(quiz, "", report, ["schema", "id", "emoji", "title", "description", "tasks"], ["$schema", "short"]);
    if (!json) return;
    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);
    if (Object.hasOwn(json, "schema") && json.schema !== QUIZ_SCHEMA) report("/schema", "value-invalid");
    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);
    if (Object.hasOwn(json, "emoji")) string(json.emoji, "/emoji", report, 1, 16);
    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);
    short(json, "", report);
    if (Object.hasOwn(json, "description")) text(json.description, "/description", report);
    const tasks = Object.hasOwn(json, "tasks") ? array(json.tasks, "/tasks", report, 1, (entry, path) => task(entry, path, report)) : undefined;
    uniqueIds(tasks, "/tasks", report);
  });
}

/** 📜️ A badge rule; returns it when structurally valid: a known kind, its members, a slug for the quiz, a task kind, and a challenge for the least challenge (`value-invalid`). */
function badgeRule(value: unknown, path: string, report: Report): Json | undefined {
  if (!isObject(value)) return void report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return void report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, BADGE_RULE_KINDS)) return undefined;
  const json = value.kind === "perfect-quiz" ? object(value, path, report, ["kind", "quiz"], ["challenge"]) : value.kind === "perfect-tasks" ? object(value, path, report, ["kind"], ["taskKind", "quiz", "challenge"]) : object(value, path, report, ["kind"]);
  if (!json) return undefined;
  const quizValid = !Object.hasOwn(json, "quiz") || slug(json.quiz, at(path, "quiz"), report);
  const kindValid = !Object.hasOwn(json, "taskKind") || literal(json.taskKind, at(path, "taskKind"), report, TASK_KINDS);
  const challengeValid = !Object.hasOwn(json, "challenge") || literal(json.challenge, at(path, "challenge"), report, CHALLENGES);
  return quizValid && kindValid && challengeValid && !(value.kind === "perfect-quiz" && !Object.hasOwn(json, "quiz")) ? json : undefined;
}

/** 📚️ Every issue of a catalog document given its quizzes in catalog order: structure per the schema, unique paths, quiz ids and badge ids, and badge rules that reference existing quizzes and select at least one task. */
export function catalogIssues(catalog: unknown, quizzes: readonly Quiz[]): ValidationIssue[] {
  return collect((report) => {
    const json = object(catalog, "", report, ["schema", "id", "title", "introduction", "quizzes", "badges"], ["$schema", "short"]);
    if (!json) return;
    if (Object.hasOwn(json, "$schema")) string(json.$schema, "/$schema", report);
    if (Object.hasOwn(json, "schema") && json.schema !== CATALOG_SCHEMA) report("/schema", "value-invalid");
    if (Object.hasOwn(json, "id")) slug(json.id, "/id", report);
    if (Object.hasOwn(json, "title")) text(json.title, "/title", report);
    short(json, "", report);
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

/** 🎯️ Whether `point` lies in one of the inclusive ranges. */
function within(ranges: readonly (readonly [number, number])[], point: number): boolean {
  return ranges.some(([low, high]) => point >= low && point <= high);
}

/** ✂️ The display handle and its key, or `undefined` for a handle outside the policy of `$defs/Handle`.
 *
 * `White_Space` runs collapse to one space and are trimmed, the typographic apostrophe U+2019 becomes `'`; what remains
 * must be 1…{@link HANDLE_MAX} code points of {@link HANDLE_LETTERS}, ASCII digits, {@link HANDLE_PUNCTUATION} and
 * single spaces, with at least one letter or digit. Control and format characters, combining marks (so every NFD
 * spelling), other scripts, lone surrogates and input over {@link HANDLE_INPUT_MAX} code points are refused. The key
 * is the lowercased display. */
export function normalizeHandle(handle: string): NormalizedHandle | undefined {
  if (typeof handle !== "string" || handle.length > 2 * HANDLE_INPUT_MAX) return undefined;
  const points = Array.from(handle, (character) => character.codePointAt(0)!);
  if (points.length > HANDLE_INPUT_MAX) return undefined;
  const display: number[] = [];
  let gap = false;
  let worded = false;
  for (const typed of points) {
    if (within(WHITE_SPACE, typed)) {
      gap = display.length > 0;
      continue;
    }
    const point = typed === TYPOGRAPHIC_APOSTROPHE ? APOSTROPHE : typed;
    const word = within(HANDLE_LETTERS, point) || (point >= 0x30 && point <= 0x39);
    if (!word && !HANDLE_PUNCTUATION.includes(point)) return undefined;
    if (gap) display.push(SPACE);
    gap = false;
    worded ||= word;
    display.push(point);
  }
  if (!worded || display.length > HANDLE_MAX) return undefined;
  const text = String.fromCodePoint(...display);
  return { display: text, key: text.toLowerCase() };
}

/** 🪝️ The id of the actor that holds a handle key: the lowercase hex of the key's UTF-8 bytes. */
export function handleActorId(key: string): string {
  return Array.from(UTF8.encode(key), (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/** 🔓️ The handle key an actor id names — the inverse of {@link handleActorId} — or `undefined` for anything but lowercase hex of well-formed UTF-8. */
export function handleKeyOf(actor: string): string | undefined {
  if (typeof actor !== "string" || actor === "" || !/^(?:[0-9a-f]{2})+$/u.test(actor)) return undefined;
  const bytes = Uint8Array.from(actor.match(/../gu)!, (pair) => Number.parseInt(pair, 16));
  try {
    return new TextDecoder("utf-8", { fatal: true, ignoreBOM: true }).decode(bytes);
  } catch {
    return undefined;
  }
}

/** 🆔️ Whether `value` is an id: exactly 32 lowercase hex characters. */
export function isId(value: unknown): value is string {
  return typeof value === "string" && ID.test(value);
}

/** 🐌️ Whether `value` is a slug: `^[a-z0-9]+(?:-[a-z0-9]+)*$` and at most 64 characters. */
export function isSlug(value: unknown): value is string {
  return typeof value === "string" && value.length <= 64 && SLUG.test(value);
}

/** ⏱️ Whether `value` is a timestamp: an integer from 0 to {@link MAX_TIMESTAMP}. */
export function isTimestamp(value: unknown): value is Timestamp {
  return Number.isInteger(value) && (value as number) >= 0 && (value as number) <= MAX_TIMESTAMP;
}

/** 🛃️ `id-invalid` unless every id a command carries has its shape: the command, learner and run ids are ids, the quiz and task ids slugs. A challenge that is none of the four or an `at` that is no timestamp (an integer from 0 to {@link MAX_TIMESTAMP}) is refused the same way: the Rust twin cannot decode the former or a negative, fractional or too large `at` (the proctor answers `command-malformed` before any decision) and refuses an `at` beyond {@link MAX_TIMESTAMP} as `id-invalid`, so the deputy never accepts what the proctor refuses. */
export function commandRejection(command: Command): Rejection | undefined {
  const members = command as Readonly<Record<string, unknown>>;
  const ids = ["id", "learner", "run"].every((member) => !Object.hasOwn(members, member) || isId(members[member]));
  const slugs = ["quiz", "task"].every((member) => !Object.hasOwn(members, member) || isSlug(members[member]));
  const challenge = command.type !== "start-run" || (CHALLENGES as readonly unknown[]).includes(members.challenge);
  const at = (command.type !== "start-run" && command.type !== "open-task" && command.type !== "record-answer") || isTimestamp(members.at);
  return ids && slugs && challenge && at ? undefined : "id-invalid";
}

/** 🧐️ `id-invalid` for a query whose learner or run is no id or whose quiz is no slug, `handle-invalid` for a `handle` query outside the handle policy. */
export function queryRejection(query: Query): Rejection | undefined {
  const members = query as Readonly<Record<string, unknown>>;
  if (!["learner", "run"].every((member) => !Object.hasOwn(members, member) || isId(members[member]))) return "id-invalid";
  if (Object.hasOwn(members, "quiz") && !isSlug(members.quiz)) return "id-invalid";
  return query.type === "handle" && normalizeHandle(query.handle) === undefined ? "handle-invalid" : undefined;
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

/** 🎭️ An identity: a known kind, and for pseudonyms and names a handle in its normalized display form (`handle-invalid`). */
function identity(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, IDENTITY_KINDS)) return;
  const json = value.kind === "anonymous" ? object(value, path, report, ["kind"]) : object(value, path, report, ["kind", "handle"]);
  if (!json || !Object.hasOwn(json, "handle")) return;
  if (typeof json.handle !== "string") return report(at(path, "handle"), "type-invalid");
  if (normalizeHandle(json.handle)?.display !== json.handle) report(at(path, "handle"), "handle-invalid");
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

/** ✍️ A draft answer: structurally valid per the schema (kind, members, slugs; matching values and sorting guesses finite numbers, `type-invalid` otherwise), possibly partial, with at most {@link THINKING_LIMIT} entries per map or order (`too-many`) and no repeated item in an order (`duplicate-id`). */
function answer(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  if (!literal(value.kind, at(path, "kind"), report, TASK_KINDS)) return;
  const member = value.kind === "sorting" ? "order" : value.kind === "matching" ? "values" : "assignments";
  const json = object(value, path, report, ["kind", member], value.kind === "sorting" ? ["guesses"] : []);
  if (!json || !Object.hasOwn(json, member)) return;
  if (value.kind === "sorting") {
    const seen = new Set<unknown>();
    const order = array(json.order, at(path, "order"), report, 0, (entry, entryPath) => {
      slug(entry, entryPath, report);
      if (seen.has(entry)) report(entryPath, "duplicate-id");
      seen.add(entry);
    });
    if (order && order.length > THINKING_LIMIT) report(at(path, "order"), "too-many");
    if (Object.hasOwn(json, "guesses")) slugMap(json.guesses, at(path, "guesses"), report, (entry, entryPath) => number(entry, entryPath, report));
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

/** 🎰️ Whether `guess` is a number a learner may guess on `scale`: finite, and positive on a logarithmic scale. */
function guessFits(guess: unknown, scale: Scale): guess is number {
  return typeof guess === "number" && Number.isFinite(guess) && (scale !== "logarithmic" || guess > 0);
}

/** 🪜️ Whether the sorting `guesses` of a sheet task that hides the keys fit a valid `order`: absent, or an object naming only presented items with numbers that fit the scale, where the guessed items stand in non-decreasing guess order. */
function guessesFit(guesses: unknown, order: readonly string[], items: ReadonlySet<string>, scale: Scale): boolean {
  if (guesses === undefined) return true;
  if (!isObject(guesses)) return false;
  const entries = Object.entries(guesses);
  if (!entries.every(([item, guess]) => items.has(item) && guessFits(guess, scale))) return false;
  const guessed = order.filter((item) => Object.hasOwn(guesses, item)).map((item) => guesses[item] as number);
  return guessed.every((guess, index) => index === 0 || guessed[index - 1]! <= guess);
}

/** 🙈️ Whether a matching sheet task hides the keys: some dimension carries no cards. */
function cardless(sheetTask: SheetMatchingTask): boolean {
  return sheetTask.dimensions.some((dimension) => dimension.cards === undefined);
}

/** 🃏️ Whether the matching `assignments` fit a sheet task that shows the keys: per presented dimension, presented items each with a card index of the dimension that no other item holds. */
function assignmentsFit(assignments: unknown, sheetTask: SheetMatchingTask, items: ReadonlySet<string>): boolean {
  if (!isObject(assignments)) return false;
  const cards = new Map(sheetTask.dimensions.map((dimension) => [dimension.id, dimension.cards?.length ?? 0]));
  for (const [dimension, perItem] of Object.entries(assignments)) {
    const count = cards.get(dimension);
    if (count === undefined || !isObject(perItem)) return false;
    const used = new Set<number>();
    for (const [item, card] of Object.entries(perItem)) {
      if (!items.has(item) || typeof card !== "number" || !Number.isInteger(card) || card < 0 || card >= count || used.has(card)) return false;
      used.add(card);
    }
  }
  return true;
}

/** 🔮️ Whether the matching `guesses` fit a sheet task that hides the keys: absent, or per presented dimension, presented items each with a number that fits the dimension's scale. */
function matchingGuessesFit(guesses: unknown, sheetTask: SheetMatchingTask, items: ReadonlySet<string>): boolean {
  if (guesses === undefined) return true;
  if (!isObject(guesses)) return false;
  const scales = new Map(sheetTask.dimensions.map((dimension) => [dimension.id, dimension.quantity.scale]));
  return Object.entries(guesses).every(([dimension, perItem]) => {
    const scale = scales.get(dimension);
    return scale !== undefined && isObject(perItem) && Object.entries(perItem).every(([item, guess]) => items.has(item) && guessFits(guess, scale));
  });
}

/** 🚧️ Why an answer is rejected for a sheet task (`answer-invalid`), or `undefined` when it is valid; partial classification and matching answers are valid. Where the sheet task shows the keys a sorting carries no guesses and a matching its card assignments and no guesses; where it hides them a sorting's guesses must fit the order and a matching carries guesses and no assignments. */
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
      if (seen.size !== order.length || !order.every((item) => typeof item === "string" && items.has(item))) return invalid;
      const guesses = (answer as Json).guesses;
      if (sheetTask.keys !== undefined) return guesses === undefined ? undefined : invalid;
      return guessesFit(guesses, order as readonly string[], items, sheetTask.quantity.scale) ? undefined : invalid;
    }
    case "matching": {
      const { assignments, guesses } = answer as Json;
      if (cardless(sheetTask)) return assignments === undefined && matchingGuessesFit(guesses, sheetTask, items) ? undefined : invalid;
      return guesses === undefined && assignmentsFit(assignments, sheetTask, items) ? undefined : invalid;
    }
  }
}

/** 🧾️ Whether `perDimension` holds an entry for every presented item in `dimension`. */
function everyItem(perDimension: Readonly<Record<string, Readonly<Record<string, number>>>> | undefined, dimension: string, sheetTask: SheetTask): boolean {
  const perItem = perDimension && Object.hasOwn(perDimension, dimension) ? perDimension[dimension] : undefined;
  return isObject(perItem) && sheetTask.items.every((item) => Object.hasOwn(perItem, item.id));
}

/** 🏁️ Whether an answer completes its sheet task: every item classified; every item matched in every dimension, with a card where the keys show and a guess where they are hidden; a recorded sorting where the keys show, a guess for every item where they are hidden. */
export function answerComplete(sheetTask: SheetTask, answer?: Answer): boolean {
  if (!answer || answer.kind !== sheetTask.kind) return false;
  switch (answer.kind) {
    case "classification":
      return sheetTask.items.every((item) => Object.hasOwn(answer.assignments, item.id));
    case "sorting": {
      const guesses = answer.guesses;
      return sheetTask.kind === "sorting" && (sheetTask.keys !== undefined || (isObject(guesses) && sheetTask.items.every((item) => Object.hasOwn(guesses, item.id))));
    }
    case "matching":
      return sheetTask.kind === "matching" && sheetTask.dimensions.every((dimension) => everyItem(dimension.cards === undefined ? answer.guesses : answer.assignments, dimension.id, sheetTask));
  }
}
