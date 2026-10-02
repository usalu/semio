/** ✅️ Structural and semantic validation of menageries, species and ensembles without any schema library, and the assembly of a menagerie from its ensemble.
 *
 * A finding carries a JSON pointer into the validated document and a stable kebab-case code. Findings are returned
 * deduplicated and sorted by path, then code, in Unicode code point order. The structure of `🧬️schema/🔣️.json` is
 * checked first (`type-invalid`, `required`, `property-unknown`, `value-invalid`, `slug-invalid`, `length-invalid`,
 * `items-too-few`); a value that fails its structure is not judged any further. Then the document rules follow:
 * `duplicate-id`, `unknown-reference`, `bone-order`, `key-order`, `loop-seam`, `ease-range`, `out-of-range`,
 * `self-bond`, `duplicate-bond`, `missing-gait-clip`, `float-hover`, `empty-cast`, `duplicate-scene`.
 *
 * @see ../../🧬️schema/🔣️.json — the structure checked here
 * @see ../../README.md — the table of codes, the pointer each one is reported at and the rule behind it
 * @see ./🦀️.rs — the Rust twin
 */
import { ACTIVITIES, CHANNELS, ENSEMBLE_SCHEMA, GAITS, LANGUAGES, MENAGERIE_SCHEMA, PAINTS, type Ensemble, type Menagerie, type Species } from "../../🧬️schema/🟦️.ts";

/** 🩺️ One finding: where (JSON pointer) and what (kebab-case code). */
export type Issue = { readonly path: string; readonly code: string };

type Json = Readonly<Record<string, unknown>>;
type Report = (path: string, code: string) => void;
type Check<T> = (value: unknown, path: string) => T;

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const COLOR = /^#[0-9a-f]{6}$/u;
const SLUG_MAX = 64;
const SEAM_SLACK = 1e-9;
const SPECIES_FIELDS = ["id", "name", "thing", "grounds", "size", "palette", "bones", "parts", "face", "clips", "repertoire", "locomotion", "temperament"] as const;
const DOCUMENT_FIELDS = ["schema", "id", "title", "species", "bonds", "casts"] as const;
const TRAITS = ["energy", "sociability", "curiosity"] as const;
const COLORS = ["body", "accent", "detail"] as const;
const SHAPES: Readonly<Record<string, { readonly plain: readonly string[]; readonly positive: readonly string[] }>> = {
  path: { plain: [], positive: [] },
  ellipse: { plain: ["cx", "cy"], positive: ["rx", "ry"] },
  rect: { plain: ["x", "y"], positive: ["width", "height"] },
  line: { plain: ["x1", "y1", "x2", "y2"], positive: [] },
};

/** 🧷️ A JSON pointer one step below `base`, escaping `~` and `/`. */
function at(base: string, key: string | number): string {
  return `${base}/${String(key).replaceAll("~", "~0").replaceAll("/", "~1")}`;
}

/** 🔠️ Code point order, identical to UTF-8 byte order; the order of findings. */
function compareCodePoints(left: string, right: string): number {
  const a = [...left];
  const b = [...right];
  for (let index = 0; index < Math.min(a.length, b.length); index++) {
    const difference = a[index]!.codePointAt(0)! - b[index]!.codePointAt(0)!;
    if (difference !== 0) return difference;
  }
  return a.length - b.length;
}

/** 🧹️ Runs `check` with a reporter and returns its findings deduplicated and sorted. */
function collect(check: (report: Report) => void): Issue[] {
  const seen = new Map<string, Issue>();
  check((path, code) => seen.set(`${path}\u0000${code}`, { path, code }));
  return [...seen.values()].sort((left, right) => compareCodePoints(left.path, right.path) || compareCodePoints(left.code, right.code));
}

/** 🧱️ Whether `value` is a JSON object. */
function isObject(value: unknown): value is Json {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** 🏷️ Whether `value` is a slug of 1…64 characters. */
function isSlug(value: unknown): value is string {
  return typeof value === "string" && value.length <= SLUG_MAX && SLUG.test(value);
}

/** 📦️ An object with exactly the given required and optional properties, or `undefined` after reporting `type-invalid`. */
function object(value: unknown, path: string, report: Report, required: readonly string[], optional: readonly string[] = []): Json | undefined {
  if (!isObject(value)) return void report(path, "type-invalid");
  for (const key of required) if (!Object.hasOwn(value, key)) report(at(path, key), "required");
  for (const key of Object.keys(value)) if (!required.includes(key) && !optional.includes(key)) report(at(path, key), "property-unknown");
  return value;
}

/** 🔍️ Checks a property when the object carries it and hands its verdict on. */
function given<T>(json: Json, key: string, path: string, check: Check<T>): T | undefined {
  return Object.hasOwn(json, key) ? check(json[key], at(path, key)) : undefined;
}

/** 🔡️ A string of at least `min` code points. */
function string(value: unknown, path: string, report: Report, min = 0): value is string {
  if (typeof value !== "string") return (report(path, "type-invalid"), false);
  if ([...value].length < min) return (report(path, "length-invalid"), false);
  return true;
}

/** 🔖️ A slug; hands it on when valid. */
function slug(value: unknown, path: string, report: Report): string | undefined {
  if (typeof value !== "string") return void report(path, "type-invalid");
  if (!isSlug(value)) return void report(path, "slug-invalid");
  return value;
}

/** 🔢️ A finite number; hands it on when valid. */
function number(value: unknown, path: string, report: Report): number | undefined {
  if (typeof value !== "number" || !Number.isFinite(value)) return void report(path, "type-invalid");
  return value;
}

/** 📏️ A number above zero (`orZero`: zero or above); hands it on when valid. */
function positive(value: unknown, path: string, report: Report, orZero = false): number | undefined {
  const valid = number(value, path, report);
  if (valid === undefined) return undefined;
  if (orZero ? valid < 0 : valid <= 0) return void report(path, "out-of-range");
  return valid;
}

/** 🎚️ A number in `[low, high]`. */
function within(value: unknown, path: string, report: Report, low: number, high: number): void {
  const valid = number(value, path, report);
  if (valid !== undefined && (valid < low || valid > high)) report(path, "out-of-range");
}

/** 📌️ One of the allowed literal values; hands it on when valid. */
function literal(value: unknown, path: string, report: Report, allowed: readonly string[]): string | undefined {
  if (typeof value !== "string" || !allowed.includes(value)) return void report(path, "value-invalid");
  return value;
}

/** 🗄️ An array whose entries are each checked by `each`; hands the verdicts on in list order. */
function array<T>(value: unknown, path: string, report: Report, each: Check<T>): T[] | undefined {
  if (!Array.isArray(value)) return void report(path, "type-invalid");
  return value.map((entry, index) => each(entry, at(path, index)));
}

/** 🌍️ A text in every language. */
function text(value: unknown, path: string, report: Report): void {
  const json = object(value, path, report, LANGUAGES);
  if (json) for (const language of LANGUAGES) given(json, language, path, (entry, entryPath) => string(entry, entryPath, report, 1));
}

/** 👯️ Reports `duplicate-id` at every entry that repeats an earlier one. */
function unique(ids: readonly (string | undefined)[], path: string, report: Report, key?: string): void {
  const seen = new Set<string>();
  ids.forEach((id, index) => {
    if (id === undefined) return;
    if (seen.has(id)) report(key === undefined ? at(path, index) : at(at(path, index), key), "duplicate-id");
    seen.add(id);
  });
}

/** 🔗️ A slug that must be one of `known`; reports `unknown-reference` otherwise. */
function reference(value: unknown, path: string, report: Report, known: ReadonlySet<string> | undefined): string | undefined {
  const valid = slug(value, path, report);
  if (valid !== undefined && known !== undefined && !known.has(valid)) report(path, "unknown-reference");
  return valid;
}

/** 🦴️ The bones of a rig: parents first, exactly the first without a parent; hands the bone ids on. */
function bones(value: unknown, path: string, report: Report): ReadonlySet<string> {
  const parents: (unknown | undefined)[] = [];
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "x", "y"], ["parent", "rotation"]);
    parents.push(json && Object.hasOwn(json, "parent") ? json.parent : undefined);
    if (!json) return undefined;
    for (const key of ["x", "y", "rotation"]) given(json, key, entryPath, (field, fieldPath) => number(field, fieldPath, report));
    given(json, "parent", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (!ids) return new Set();
  if (ids.length < 1) report(path, "items-too-few");
  unique(ids, path, report, "id");
  parents.forEach((parent, index) => {
    if (parent === undefined) return index > 0 && isObject((value as unknown[])[index]) ? report(at(path, index), "bone-order") : undefined;
    if (!isSlug(parent)) return;
    const first = ids.indexOf(parent);
    if (first < 0) report(at(at(path, index), "parent"), "unknown-reference");
    if (index === 0 || first >= index) report(at(at(path, index), "parent"), "bone-order");
  });
  return new Set(ids.filter((id) => id !== undefined));
}

/** 🔷️ The geometry of a part: the variant `kind` names, with positive extents. */
function shape(value: unknown, path: string, report: Report): void {
  if (!isObject(value)) return report(path, "type-invalid");
  if (!Object.hasOwn(value, "kind")) return report(at(path, "kind"), "required");
  const kind = literal(value.kind, at(path, "kind"), report, Object.keys(SHAPES));
  if (kind === undefined) return;
  const fields = SHAPES[kind]!;
  object(value, path, report, ["kind", ...(kind === "path" ? ["d"] : []), ...fields.plain, ...fields.positive], kind === "rect" ? ["radius"] : []);
  if (kind === "path") given(value, "d", path, (field, fieldPath) => string(field, fieldPath, report, 1));
  for (const key of fields.plain) given(value, key, path, (field, fieldPath) => number(field, fieldPath, report));
  for (const key of fields.positive) given(value, key, path, (field, fieldPath) => positive(field, fieldPath, report));
  if (kind === "rect") given(value, "radius", path, (field, fieldPath) => positive(field, fieldPath, report, true));
}

/** 🧩️ The parts of a species, each on a bone of the rig; hands the part ids on. */
function parts(value: unknown, path: string, report: Report, boneIds: ReadonlySet<string>): ReadonlySet<string> {
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "bone", "shape", "fill", "stroke"], ["strokeWidth"]);
    if (!json) return undefined;
    given(json, "bone", entryPath, (field, fieldPath) => reference(field, fieldPath, report, boneIds));
    given(json, "shape", entryPath, (field, fieldPath) => shape(field, fieldPath, report));
    for (const key of ["fill", "stroke"]) given(json, key, entryPath, (field, fieldPath) => literal(field, fieldPath, report, PAINTS));
    given(json, "strokeWidth", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (!ids) return new Set();
  unique(ids, path, report, "id");
  return new Set(ids.filter((id) => id !== undefined));
}

/** 🙂️ The face: eyes whose pupil is smaller than their white, an optional mouth, and the part it is drawn above. */
function face(value: unknown, path: string, report: Report, boneIds: ReadonlySet<string>, partIds: ReadonlySet<string>): void {
  const json = object(value, path, report, ["eyes"], ["mouth", "above"]);
  if (!json) return;
  given(json, "eyes", path, (eyes, eyesPath) => {
    const ids = array(eyes, eyesPath, report, (entry, entryPath) => {
      const eye = object(entry, entryPath, report, ["id", "bone", "x", "y", "radius", "pupil"]);
      if (!eye) return undefined;
      given(eye, "bone", entryPath, (field, fieldPath) => reference(field, fieldPath, report, boneIds));
      for (const key of ["x", "y"]) given(eye, key, entryPath, (field, fieldPath) => number(field, fieldPath, report));
      const radius = given(eye, "radius", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
      const pupil = given(eye, "pupil", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
      if (radius !== undefined && pupil !== undefined && pupil >= radius) report(at(entryPath, "pupil"), "out-of-range");
      return given(eye, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
    });
    if (ids) unique(ids, eyesPath, report, "id");
  });
  given(json, "mouth", path, (entry, entryPath) => {
    const mouth = object(entry, entryPath, report, ["bone", "x", "y", "width"]);
    if (!mouth) return;
    given(mouth, "bone", entryPath, (field, fieldPath) => reference(field, fieldPath, report, boneIds));
    for (const key of ["x", "y"]) given(mouth, key, entryPath, (field, fieldPath) => number(field, fieldPath, report));
    given(mouth, "width", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
  });
  given(json, "above", path, (field, fieldPath) => reference(field, fieldPath, report, partIds));
}

/** 🔑️ One key: its phase, its value and an optional easing whose abscissas lie in [0, 1]; hands phase and value on. */
function keyframe(value: unknown, path: string, report: Report): { readonly at?: number; readonly value?: number } {
  const json = object(value, path, report, ["at", "value"], ["ease"]);
  if (!json) return {};
  given(json, "ease", path, (ease, easePath) => {
    const entries = array(ease, easePath, report, (field, fieldPath) => number(field, fieldPath, report));
    if (!entries) return;
    if (entries.length !== 4) return report(easePath, "length-invalid");
    for (const index of [0, 2]) if (entries[index] !== undefined && (entries[index]! < 0 || entries[index]! > 1)) report(at(easePath, index), "ease-range");
  });
  return { at: given(json, "at", path, (field, fieldPath) => number(field, fieldPath, report)), value: given(json, "value", path, (field, fieldPath) => number(field, fieldPath, report)) };
}

/** 🛤️ One track: at least two keys in strictly ascending phase from 0 to 1 and, in a looping clip, a seamless end — the same value at both ends, or on the rotation channel a difference of a whole number of turns, judged within 1e-9 of a turn (authored decimals such as −359.8 and −719.8 are a turn apart, and the quotient of their difference is not an exact integer). */
function track(value: unknown, path: string, report: Report, boneIds: ReadonlySet<string>, loop: boolean): void {
  const json = object(value, path, report, ["bone", "channel", "keys"]);
  if (!json) return;
  given(json, "bone", path, (field, fieldPath) => reference(field, fieldPath, report, boneIds));
  const channel = given(json, "channel", path, (field, fieldPath) => literal(field, fieldPath, report, CHANNELS));
  given(json, "keys", path, (entries, keysPath) => {
    const keys = array(entries, keysPath, report, (entry, entryPath) => keyframe(entry, entryPath, report));
    if (!keys) return;
    keys.forEach((entry, index) => {
      if (entry.at !== undefined && (entry.at < 0 || entry.at > 1)) report(at(at(keysPath, index), "at"), "key-order");
    });
    if (keys.length < 2) return report(keysPath, "key-order");
    const last = keys.length - 1;
    keys.forEach((entry, index) => {
      if (entry.at === undefined) return;
      const before = index > 0 ? keys[index - 1]!.at : undefined;
      if ((index === 0 && entry.at !== 0) || (index === last && entry.at !== 1) || (before !== undefined && entry.at <= before)) report(at(at(keysPath, index), "at"), "key-order");
    });
    const from = keys[0]!.value;
    const to = keys[last]!.value;
    if (!loop || from === undefined || to === undefined || from === to) return;
    const revolutions = (to - from) / 360;
    const apart = revolutions - Math.floor(revolutions + 0.5);
    if (channel !== "rotation" || apart > SEAM_SLACK || apart < 0 - SEAM_SLACK) report(at(at(keysPath, last), "value"), "loop-seam");
  });
}

/** 🎞️ The clips of a species; hands the clip ids on. */
function clips(value: unknown, path: string, report: Report, boneIds: ReadonlySet<string>): ReadonlySet<string> {
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "seconds", "loop", "tracks"]);
    if (!json) return undefined;
    given(json, "seconds", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
    given(json, "loop", entryPath, (field, fieldPath) => (typeof field === "boolean" ? undefined : report(fieldPath, "type-invalid")));
    given(json, "tracks", entryPath, (tracks, tracksPath) => array(tracks, tracksPath, report, (member, memberPath) => track(member, memberPath, report, boneIds, json.loop === true)));
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (!ids) return new Set();
  unique(ids, path, report, "id");
  return new Set(ids.filter((id) => id !== undefined));
}

/** 🗃️ The repertoire: clip ids per activity; hands the activities that have at least one clip on. */
function repertoire(value: unknown, path: string, report: Report, clipIds: ReadonlySet<string>): ReadonlySet<string> | undefined {
  const json = object(value, path, report, [], ACTIVITIES);
  if (!json) return undefined;
  const played = new Set<string>();
  for (const activity of ACTIVITIES) {
    const listed = given(json, activity, path, (entries, entriesPath) => array(entries, entriesPath, report, (field, fieldPath) => reference(field, fieldPath, report, clipIds)));
    if (listed !== undefined && listed.length > 0) played.add(activity);
  }
  return played;
}

/** 🏃️ The locomotion: a positive speed, a hover exactly when floating, and a clip for the gait of walkers and hoppers. */
function locomotion(value: unknown, path: string, report: Report, played: ReadonlySet<string> | undefined): void {
  const json = object(value, path, report, ["gait", "speed"], ["hover"]);
  if (!json) return;
  given(json, "speed", path, (field, fieldPath) => positive(field, fieldPath, report));
  given(json, "hover", path, (field, fieldPath) => positive(field, fieldPath, report));
  const gait = given(json, "gait", path, (field, fieldPath) => literal(field, fieldPath, report, GAITS));
  if (gait === undefined) return;
  if ((gait === "float") !== Object.hasOwn(json, "hover")) report(at(path, "hover"), "float-hover");
  if (gait !== "float" && played !== undefined && !played.has(gait)) report(at(path, "gait"), "missing-gait-clip");
}

/** 🧬️ A species; hands its id on when that is a slug. */
function species(value: unknown, path: string, report: Report): string | undefined {
  const json = object(value, path, report, SPECIES_FIELDS, ["$schema"]);
  if (!json) return undefined;
  given(json, "$schema", path, (field, fieldPath) => string(field, fieldPath, report));
  for (const name of ["name", "thing"]) given(json, name, path, (field, fieldPath) => text(field, fieldPath, report));
  given(json, "grounds", path, (entries, entriesPath) => array(entries, entriesPath, report, (field, fieldPath) => string(field, fieldPath, report, 1)));
  given(json, "size", path, (entry, entryPath) => {
    const size = object(entry, entryPath, report, ["width", "height"]);
    if (size) for (const side of ["width", "height"]) given(size, side, entryPath, (field, fieldPath) => positive(field, fieldPath, report));
  });
  given(json, "palette", path, (entry, entryPath) => {
    const palette = object(entry, entryPath, report, COLORS);
    if (palette) for (const color of COLORS) given(palette, color, entryPath, (field, fieldPath) => (string(field, fieldPath, report) && !COLOR.test(field) ? report(fieldPath, "out-of-range") : undefined));
  });
  const boneIds = given(json, "bones", path, (entry, entryPath) => bones(entry, entryPath, report)) ?? new Set<string>();
  const partIds = given(json, "parts", path, (entry, entryPath) => parts(entry, entryPath, report, boneIds)) ?? new Set<string>();
  given(json, "face", path, (entry, entryPath) => face(entry, entryPath, report, boneIds, partIds));
  const clipIds = given(json, "clips", path, (entry, entryPath) => clips(entry, entryPath, report, boneIds)) ?? new Set<string>();
  const played = given(json, "repertoire", path, (entry, entryPath) => repertoire(entry, entryPath, report, clipIds));
  given(json, "locomotion", path, (entry, entryPath) => locomotion(entry, entryPath, report, played));
  given(json, "temperament", path, (entry, entryPath) => {
    const temperament = object(entry, entryPath, report, TRAITS);
    if (temperament) for (const trait of TRAITS) given(temperament, trait, entryPath, (field, fieldPath) => within(field, fieldPath, report, 0, 1));
  });
  return given(json, "id", path, (field, fieldPath) => slug(field, fieldPath, report));
}

/** 🤝️ The bonds: two different species each, every unordered pair at most once, an affinity in [−1, 1]. */
function bonds(value: unknown, path: string, report: Report, speciesIds: ReadonlySet<string> | undefined): void {
  const pairs = new Set<string>();
  array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["between", "affinity"]);
    if (!json) return;
    given(json, "affinity", entryPath, (field, fieldPath) => within(field, fieldPath, report, -1, 1));
    given(json, "between", entryPath, (between, betweenPath) => {
      const ends = array(between, betweenPath, report, (field, fieldPath) => reference(field, fieldPath, report, speciesIds));
      if (!ends) return;
      if (ends.length !== 2) return report(betweenPath, "length-invalid");
      const [left, right] = ends;
      if (left === undefined || right === undefined) return;
      if (left === right) return report(betweenPath, "self-bond");
      const pair = compareCodePoints(left, right) < 0 ? `${left} ${right}` : `${right} ${left}`;
      if (pairs.has(pair)) report(betweenPath, "duplicate-bond");
      pairs.add(pair);
    });
  });
}

/** 🎟️ The casts: unique scenes, each with at least one core species. */
function casts(value: unknown, path: string, report: Report, speciesIds: ReadonlySet<string> | undefined): void {
  const scenes = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["scene", "core", "rotation"]);
    if (!json) return undefined;
    for (const list of ["core", "rotation"]) {
      const members = given(json, list, entryPath, (entries, entriesPath) => array(entries, entriesPath, report, (field, fieldPath) => reference(field, fieldPath, report, speciesIds)));
      if (list === "core" && members !== undefined && members.length < 1) report(at(entryPath, list), "empty-cast");
    }
    return given(json, "scene", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  const seen = new Set<string>();
  scenes?.forEach((scene, index) => {
    if (scene === undefined) return;
    if (seen.has(scene)) report(at(at(path, index), "scene"), "duplicate-scene");
    seen.add(scene);
  });
}

/** 📇️ The head every document shares: its schema identifier, id and title. */
function head(json: Json, report: Report, schema: string): void {
  given(json, "$schema", "", (field, fieldPath) => string(field, fieldPath, report));
  given(json, "schema", "", (field, fieldPath) => literal(field, fieldPath, report, [schema]));
  given(json, "id", "", (field, fieldPath) => slug(field, fieldPath, report));
  given(json, "title", "", (field, fieldPath) => text(field, fieldPath, report));
}

/** 🧪️ The findings of one species document; none when it is valid. */
export function speciesIssues(document: unknown): Issue[] {
  return collect((report) => void species(document, "", report));
}

/** 🎪️ The findings of a menagerie document: its species, their unique ids, and bonds and casts that name them. */
export function menagerieIssues(document: unknown): Issue[] {
  return collect((report) => {
    const json = object(document, "", report, DOCUMENT_FIELDS, ["$schema"]);
    if (!json) return;
    head(json, report, MENAGERIE_SCHEMA);
    const ids = given(json, "species", "", (entries, entriesPath) => array(entries, entriesPath, report, (entry, entryPath) => species(entry, entryPath, report)));
    if (ids) unique(ids, "/species", report, "id");
    const known = ids ? new Set(ids.filter((id) => id !== undefined)) : undefined;
    given(json, "bonds", "", (entries, entriesPath) => bonds(entries, entriesPath, report, known));
    given(json, "casts", "", (entries, entriesPath) => casts(entries, entriesPath, report, known));
  });
}

/** 🗂️ The findings of an ensemble document: unique species paths, bonds and casts (whose species ids resolve only in the assembled menagerie). */
export function ensembleIssues(document: unknown): Issue[] {
  return collect((report) => {
    const json = object(document, "", report, DOCUMENT_FIELDS, ["$schema"]);
    if (!json) return;
    head(json, report, ENSEMBLE_SCHEMA);
    const paths = given(json, "species", "", (entries, entriesPath) => array(entries, entriesPath, report, (field, fieldPath) => (string(field, fieldPath, report, 1) ? field : undefined)));
    if (paths) unique(paths, "/species", report);
    given(json, "bonds", "", (entries, entriesPath) => bonds(entries, entriesPath, report, undefined));
    given(json, "casts", "", (entries, entriesPath) => casts(entries, entriesPath, report, undefined));
  });
}

/** 🧺️ The menagerie an ensemble describes, given its species documents in the order of the ensemble's paths; a species' `$schema` hint stays behind. */
export function assembleMenagerie(ensemble: Ensemble, species: readonly Species[]): Menagerie {
  const members = species.map((member) => Object.fromEntries(Object.entries(member).filter(([name]) => name !== "$schema")) as Species);
  return { schema: MENAGERIE_SCHEMA, id: ensemble.id, title: ensemble.title, species: members, bonds: ensemble.bonds, casts: ensemble.casts };
}
