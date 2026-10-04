/** ✅️ Structural and semantic validation of menageries, species and ensembles without any schema library, and the assembly of a menagerie from its ensemble.
 *
 * A finding carries a JSON pointer into the validated document and a stable kebab-case code. Findings are returned
 * deduplicated and sorted by path, then code, in Unicode code point order. The structure of `🧬️schema/🔣️.json` is
 * checked first (`type-invalid`, `required`, `property-unknown`, `value-invalid`, `slug-invalid`, `length-invalid`,
 * `items-too-few`); a value that fails its structure is not judged any further. Then the document rules follow:
 * `duplicate-id`, `unknown-reference`, `bone-order`, `key-order`, `loop-seam`, `ease-range`, `out-of-range`,
 * `self-bond`, `duplicate-bond`, `missing-gait-clip`, `float-hover`, `empty-cast`, `duplicate-scene`,
 * `duplicate-entry`, `lasts-then`, `missing-gear-clip`, `missing-activity-clip`, `floater-gear`.
 *
 * @see ../../🧬️schema/🔣️.json — the structure checked here
 * @see ../../README.md — the table of codes, the pointer each one is reported at and the rule behind it
 * @see ./🦀️.rs — the Rust twin
 */
import { ACTIVITIES, CHANNELS, CUES, DRIFTS, ENSEMBLE_SCHEMA, GAITS, GEARS, LANGUAGES, MENAGERIE_SCHEMA, MOODS, PAINTS, type Ensemble, type Menagerie, type Species } from "../../🧬️schema/🟦️.ts";

/** 🩺️ One finding: where (JSON pointer) and what (kebab-case code). */
export type Issue = { readonly path: string; readonly code: string };

type Json = Readonly<Record<string, unknown>>;
type Report = (path: string, code: string) => void;
type Check<T> = (value: unknown, path: string) => T;
type Kind = { readonly id: string; readonly states: ReadonlySet<string>; readonly tricks: ReadonlySet<string> };
type Kinds = ReadonlyMap<string, Kind>;

const SLUG = /^[a-z0-9]+(?:-[a-z0-9]+)*$/u;
const COLOR = /^#[0-9a-f]{6}$/u;
const SLUG_MAX = 64;
const SEAM_SLACK = 1e-9;
const PARTICLES_MAX = 32;
const SPECIES_FIELDS = ["id", "name", "thing", "grounds", "size", "palette", "bones", "parts", "face", "clips", "repertoire", "locomotion", "temperament", "states", "tricks", "purr", "emitters", "gear", "grip", "reach", "mood"] as const;
const DOCUMENT_FIELDS = ["schema", "id", "title", "species", "bonds", "casts", "chemistry"] as const;
const TRAITS = ["energy", "sociability", "curiosity"] as const;
const COLORS = ["body", "accent", "detail"] as const;
const PARTIES = ["when", "near"] as const;
const PLACEMENTS = ["above", "below", "beside", "any"] as const;
const ENCOUNTERS = ["greet", "cuddle", "squabble"] as const;
const PURSUITS = ["fidget", "walk", "hop", "sleep"] as const;
const GROUND_GEARS: readonly string[] = ["climb", "ladder", "grapple"];
const GEAR_ACTIVITIES: Readonly<Record<string, readonly string[]>> = { climb: ["climb", "mantle", "slide"], ladder: ["carry", "climb"], grapple: ["aim", "reel"], parachute: ["glide"] };
const OWED_ACTIVITIES = ["hang", "tumble", "purr", "dizzy", "shrug", "push"] as const;
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

/** 👯️ Reports `code` (`duplicate-id` unless told otherwise) at every entry that repeats an earlier one. */
function unique(ids: readonly (string | undefined)[], path: string, report: Report, key?: string, code = "duplicate-id"): void {
  const seen = new Set<string>();
  ids.forEach((id, index) => {
    if (id === undefined) return;
    if (seen.has(id)) report(key === undefined ? at(path, index) : at(at(path, index), key), code);
    seen.add(id);
  });
}

/** 🧮️ A whole number in `[low, high]`: a fraction is `type-invalid`, a whole number outside is `out-of-range`. */
function whole(value: unknown, path: string, report: Report, low: number, high: number): void {
  const valid = number(value, path, report);
  if (valid === undefined) return;
  if (!Number.isInteger(valid)) return report(path, "type-invalid");
  if (valid < low || valid > high) report(path, "out-of-range");
}

/** 🎨️ A colour as `#rrggbb` in lowercase digits; anything else a string can be is `out-of-range`. */
function color(value: unknown, path: string, report: Report): void {
  if (string(value, path, report) && !COLOR.test(value)) report(path, "out-of-range");
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

/** ✨️ The emitters of a species, each on a bone of the rig, with a whole count of 1…32, a positive life, a speed of zero or more and a spread in [0, 1]; hands the emitter ids on. */
function emitters(value: unknown, path: string, report: Report, boneIds: ReadonlySet<string>): ReadonlySet<string> {
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "bone", "x", "y", "shape", "fill", "stroke", "motion", "count", "life", "speed", "spread"], ["strokeWidth"]);
    if (!json) return undefined;
    given(json, "bone", entryPath, (field, fieldPath) => reference(field, fieldPath, report, boneIds));
    for (const key of ["x", "y"]) given(json, key, entryPath, (field, fieldPath) => number(field, fieldPath, report));
    given(json, "shape", entryPath, (field, fieldPath) => shape(field, fieldPath, report));
    for (const key of ["fill", "stroke"]) given(json, key, entryPath, (field, fieldPath) => literal(field, fieldPath, report, PAINTS));
    given(json, "strokeWidth", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
    given(json, "motion", entryPath, (field, fieldPath) => literal(field, fieldPath, report, DRIFTS));
    given(json, "count", entryPath, (field, fieldPath) => whole(field, fieldPath, report, 1, PARTICLES_MAX));
    given(json, "life", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
    given(json, "speed", entryPath, (field, fieldPath) => positive(field, fieldPath, report, true));
    given(json, "spread", entryPath, (field, fieldPath) => within(field, fieldPath, report, 0, 1));
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (!ids) return new Set();
  unique(ids, path, report, "id");
  return new Set(ids.filter((id) => id !== undefined));
}

/** 🔦️ The states of a species: at least one, each with an optional tint, overlay clip and emitter of the species; a state that lasts names the state it gives way to; hands the state ids on. */
function states(value: unknown, path: string, report: Report, clipIds: ReadonlySet<string>, emitterIds: ReadonlySet<string>): ReadonlySet<string> {
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "name"], ["tint", "clip", "emitter", "lasts", "then"]);
    if (!json) return undefined;
    given(json, "name", entryPath, (field, fieldPath) => text(field, fieldPath, report));
    given(json, "tint", entryPath, (field, fieldPath) => {
      const tint = object(field, fieldPath, report, [], COLORS);
      if (tint) for (const name of COLORS) given(tint, name, fieldPath, (member, memberPath) => color(member, memberPath, report));
    });
    given(json, "clip", entryPath, (field, fieldPath) => reference(field, fieldPath, report, clipIds));
    given(json, "emitter", entryPath, (field, fieldPath) => reference(field, fieldPath, report, emitterIds));
    given(json, "lasts", entryPath, (field, fieldPath) => positive(field, fieldPath, report));
    if (Object.hasOwn(json, "lasts") && !Object.hasOwn(json, "then")) report(at(entryPath, "then"), "lasts-then");
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (!ids) return new Set();
  if (ids.length < 1) report(path, "items-too-few");
  unique(ids, path, report, "id");
  const known = new Set(ids.filter((id) => id !== undefined));
  (value as unknown[]).forEach((entry, index) => {
    if (isObject(entry)) given(entry, "then", at(path, index), (field, fieldPath) => reference(field, fieldPath, report, known));
  });
  return known;
}

/** 🪄️ The tricks of a species: a clip, distinct cues, an optional emitter, the states a trick is on offer in and the state and mood it leaves; hands the trick ids on. */
function tricks(value: unknown, path: string, report: Report, clipIds: ReadonlySet<string>, emitterIds: ReadonlySet<string>, stateIds: ReadonlySet<string>): ReadonlySet<string> {
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "name", "clip", "cues"], ["emitter", "from", "to", "mood"]);
    if (!json) return undefined;
    given(json, "name", entryPath, (field, fieldPath) => text(field, fieldPath, report));
    given(json, "clip", entryPath, (field, fieldPath) => reference(field, fieldPath, report, clipIds));
    given(json, "cues", entryPath, (entries, entriesPath) => {
      const cues = array(entries, entriesPath, report, (field, fieldPath) => literal(field, fieldPath, report, CUES));
      if (cues) unique(cues, entriesPath, report, undefined, "duplicate-entry");
    });
    given(json, "emitter", entryPath, (field, fieldPath) => reference(field, fieldPath, report, emitterIds));
    given(json, "from", entryPath, (entries, entriesPath) => {
      const offered = array(entries, entriesPath, report, (field, fieldPath) => reference(field, fieldPath, report, stateIds));
      if (offered) unique(offered, entriesPath, report, undefined, "duplicate-entry");
    });
    given(json, "to", entryPath, (field, fieldPath) => reference(field, fieldPath, report, stateIds));
    given(json, "mood", entryPath, (field, fieldPath) => literal(field, fieldPath, report, MOODS));
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (!ids) return new Set();
  unique(ids, path, report, "id");
  return new Set(ids.filter((id) => id !== undefined));
}

/** 🧰️ The gear of a species: distinct entries, none that needs ground under its feet for a floater, and for each the clips of the activities it brings. */
function gear(value: unknown, path: string, report: Report, floats: boolean, played: ReadonlySet<string> | undefined): void {
  const owned = array(value, path, report, (field, fieldPath) => literal(field, fieldPath, report, GEARS));
  if (!owned) return;
  unique(owned, path, report, undefined, "duplicate-entry");
  owned.forEach((entry, index) => {
    if (entry === undefined) return;
    if (floats && GROUND_GEARS.includes(entry)) report(at(path, index), "floater-gear");
    if (played !== undefined && GEAR_ACTIVITIES[entry]!.some((activity) => !played.has(activity))) report(at(path, index), "missing-gear-clip");
  });
}

/** 📐️ The extent `side` of a size box when the document states it as a number above zero. */
function extent(json: Json, side: string): number | undefined {
  const size = json.size;
  const value = isObject(size) ? size[side] : undefined;
  return typeof value === "number" && value > 0 ? value : undefined;
}

/** 🧬️ A species; hands its id, its state ids and its trick ids on when the id is a slug. */
function species(value: unknown, path: string, report: Report): Kind | undefined {
  const json = object(value, path, report, SPECIES_FIELDS, ["$schema", "canopy"]);
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
    if (palette) for (const name of COLORS) given(palette, name, entryPath, (field, fieldPath) => color(field, fieldPath, report));
  });
  const boneIds = given(json, "bones", path, (entry, entryPath) => bones(entry, entryPath, report)) ?? new Set<string>();
  const partIds = given(json, "parts", path, (entry, entryPath) => parts(entry, entryPath, report, boneIds)) ?? new Set<string>();
  given(json, "face", path, (entry, entryPath) => face(entry, entryPath, report, boneIds, partIds));
  const clipIds = given(json, "clips", path, (entry, entryPath) => clips(entry, entryPath, report, boneIds)) ?? new Set<string>();
  const played = given(json, "repertoire", path, (entry, entryPath) => repertoire(entry, entryPath, report, clipIds));
  if (played !== undefined) for (const activity of OWED_ACTIVITIES) if (!played.has(activity)) report(at(at(path, "repertoire"), activity), "missing-activity-clip");
  given(json, "locomotion", path, (entry, entryPath) => locomotion(entry, entryPath, report, played));
  given(json, "temperament", path, (entry, entryPath) => {
    const temperament = object(entry, entryPath, report, TRAITS);
    if (temperament) for (const trait of TRAITS) given(temperament, trait, entryPath, (field, fieldPath) => within(field, fieldPath, report, 0, 1));
  });
  const emitterIds = given(json, "emitters", path, (entry, entryPath) => emitters(entry, entryPath, report, boneIds)) ?? new Set<string>();
  const stateIds = given(json, "states", path, (entry, entryPath) => states(entry, entryPath, report, clipIds, emitterIds)) ?? new Set<string>();
  const trickIds = given(json, "tricks", path, (entry, entryPath) => tricks(entry, entryPath, report, clipIds, emitterIds, stateIds)) ?? new Set<string>();
  given(json, "purr", path, (entry, entryPath) => {
    const purr = object(entry, entryPath, report, ["clip"], ["emitter"]);
    if (!purr) return;
    given(purr, "clip", entryPath, (field, fieldPath) => reference(field, fieldPath, report, clipIds));
    given(purr, "emitter", entryPath, (field, fieldPath) => reference(field, fieldPath, report, emitterIds));
  });
  given(json, "gear", path, (entry, entryPath) => gear(entry, entryPath, report, isObject(json.locomotion) && json.locomotion.gait === "float", played));
  for (const [key, side, orZero] of [["grip", "height", false], ["reach", "width", true]] as const) {
    given(json, key, path, (field, fieldPath) => {
      const valid = positive(field, fieldPath, report, orZero);
      const most = extent(json, side);
      if (valid !== undefined && most !== undefined && valid > most) report(fieldPath, "out-of-range");
    });
  }
  given(json, "canopy", path, (field, fieldPath) => shape(field, fieldPath, report));
  given(json, "mood", path, (field, fieldPath) => literal(field, fieldPath, report, MOODS));
  const id = given(json, "id", path, (field, fieldPath) => slug(field, fieldPath, report));
  return id === undefined ? undefined : { id, states: stateIds, tricks: trickIds };
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

/** 🔎️ One side of a reaction: optionally a species of the menagerie, a state and a trick of that species (of some species of the menagerie when it names none), a positive time held, a mood and an activity; hands on the species it names when the menagerie knows it, `anyone` when it names none. */
function trait(value: unknown, path: string, report: Report, kinds: Kinds | undefined, anyone: Kind | undefined): Kind | undefined {
  const json = object(value, path, report, [], ["species", "state", "held", "mood", "activity", "trick"]);
  if (!json) return undefined;
  const named = given(json, "species", path, (field, fieldPath) => reference(field, fieldPath, report, kinds && new Set(kinds.keys())));
  const kind = !Object.hasOwn(json, "species") ? anyone : named === undefined ? undefined : kinds?.get(named);
  given(json, "state", path, (field, fieldPath) => reference(field, fieldPath, report, kind?.states));
  given(json, "trick", path, (field, fieldPath) => reference(field, fieldPath, report, kind?.tricks));
  given(json, "held", path, (field, fieldPath) => positive(field, fieldPath, report));
  given(json, "mood", path, (field, fieldPath) => literal(field, fieldPath, report, MOODS));
  given(json, "activity", path, (field, fieldPath) => literal(field, fieldPath, report, ACTIVITIES));
  return kind;
}

/** 🫂️ The bounds of the affinity a reaction asks of its pair: two numbers in [−1, 1], the low one first (an empty range is `out-of-range`). */
function bounds(value: unknown, path: string, report: Report): void {
  const ends = array(value, path, report, (field, fieldPath) => {
    within(field, fieldPath, report, -1, 1);
    return typeof field === "number" ? field : undefined;
  });
  if (!ends) return;
  if (ends.length !== 2) return report(path, "length-invalid");
  const [low, high] = ends;
  if (low !== undefined && high !== undefined && low > high) report(path, "out-of-range");
}

/** ⚗️ The chemistry: unique reaction ids, sides that name species of the menagerie (or nobody: anyone) with states and tricks of those species, a third side `unless` like them, the bounds of an affinity, positive distances and periods, a chance in [0, 1] and at least one effect, each on one side with a state or trick of that side's species, an amount in [0, 1], a rapport step in [−1, 1] and an activity a pet starts by itself. */
function chemistry(value: unknown, path: string, report: Report, kinds: Kinds | undefined): void {
  const anyone: Kind | undefined = kinds && { id: "", states: new Set([...kinds.values()].flatMap((kind) => [...kind.states])), tricks: new Set([...kinds.values()].flatMap((kind) => [...kind.tricks])) };
  const ids = array(value, path, report, (entry, entryPath) => {
    const json = object(entry, entryPath, report, ["id", "when", "near", "within", "every", "then"], ["where", "unless", "affinity", "chance"]);
    if (!json) return undefined;
    const sides: Record<string, Kind | undefined> = {};
    for (const party of PARTIES) sides[party] = given(json, party, entryPath, (field, fieldPath) => trait(field, fieldPath, report, kinds, anyone));
    given(json, "unless", entryPath, (field, fieldPath) => trait(field, fieldPath, report, kinds, anyone));
    given(json, "affinity", entryPath, (field, fieldPath) => bounds(field, fieldPath, report));
    for (const key of ["within", "every"]) given(json, key, entryPath, (field, fieldPath) => positive(field, fieldPath, report));
    given(json, "where", entryPath, (field, fieldPath) => literal(field, fieldPath, report, PLACEMENTS));
    given(json, "chance", entryPath, (field, fieldPath) => within(field, fieldPath, report, 0, 1));
    given(json, "then", entryPath, (entries, entriesPath) => {
      const effects = array(entries, entriesPath, report, (member, memberPath) => {
        const effect = object(member, memberPath, report, ["on"], ["state", "mood", "amount", "rapport", "encounter", "trick", "activity"]);
        if (!effect) return;
        const party = given(effect, "on", memberPath, (field, fieldPath) => literal(field, fieldPath, report, PARTIES));
        const kind = party === undefined ? undefined : sides[party];
        given(effect, "state", memberPath, (field, fieldPath) => reference(field, fieldPath, report, kind?.states));
        given(effect, "trick", memberPath, (field, fieldPath) => reference(field, fieldPath, report, kind?.tricks));
        given(effect, "mood", memberPath, (field, fieldPath) => literal(field, fieldPath, report, MOODS));
        given(effect, "amount", memberPath, (field, fieldPath) => within(field, fieldPath, report, 0, 1));
        given(effect, "rapport", memberPath, (field, fieldPath) => within(field, fieldPath, report, -1, 1));
        given(effect, "encounter", memberPath, (field, fieldPath) => literal(field, fieldPath, report, ENCOUNTERS));
        given(effect, "activity", memberPath, (field, fieldPath) => literal(field, fieldPath, report, PURSUITS));
      });
      if (effects && effects.length < 1) report(entriesPath, "items-too-few");
    });
    return given(json, "id", entryPath, (field, fieldPath) => slug(field, fieldPath, report));
  });
  if (ids) unique(ids, path, report, "id");
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

/** 🎪️ The findings of a menagerie document: its species, their unique ids, and bonds, casts and reactions that name them. */
export function menagerieIssues(document: unknown): Issue[] {
  return collect((report) => {
    const json = object(document, "", report, DOCUMENT_FIELDS, ["$schema"]);
    if (!json) return;
    head(json, report, MENAGERIE_SCHEMA);
    const found = given(json, "species", "", (entries, entriesPath) => array(entries, entriesPath, report, (entry, entryPath) => species(entry, entryPath, report)));
    if (found) unique(found.map((kind) => kind?.id), "/species", report, "id");
    const kinds = found ? new Map([...found].reverse().flatMap((kind) => (kind === undefined ? [] : [[kind.id, kind] as const]))) : undefined;
    const known = kinds && new Set(kinds.keys());
    given(json, "bonds", "", (entries, entriesPath) => bonds(entries, entriesPath, report, known));
    given(json, "casts", "", (entries, entriesPath) => casts(entries, entriesPath, report, known));
    given(json, "chemistry", "", (entries, entriesPath) => chemistry(entries, entriesPath, report, kinds));
  });
}

/** 🗂️ The findings of an ensemble document: unique species paths, bonds, casts and reactions (whose species, states and tricks resolve only in the assembled menagerie). */
export function ensembleIssues(document: unknown): Issue[] {
  return collect((report) => {
    const json = object(document, "", report, DOCUMENT_FIELDS, ["$schema"]);
    if (!json) return;
    head(json, report, ENSEMBLE_SCHEMA);
    const paths = given(json, "species", "", (entries, entriesPath) => array(entries, entriesPath, report, (field, fieldPath) => (string(field, fieldPath, report, 1) ? field : undefined)));
    if (paths) unique(paths, "/species", report);
    given(json, "bonds", "", (entries, entriesPath) => bonds(entries, entriesPath, report, undefined));
    given(json, "casts", "", (entries, entriesPath) => casts(entries, entriesPath, report, undefined));
    given(json, "chemistry", "", (entries, entriesPath) => chemistry(entries, entriesPath, report, undefined));
  });
}

/** 🧺️ The menagerie an ensemble describes, given its species documents in the order of the ensemble's paths; a species' `$schema` hint stays behind. */
export function assembleMenagerie(ensemble: Ensemble, species: readonly Species[]): Menagerie {
  const members = species.map((member) => Object.fromEntries(Object.entries(member).filter(([name]) => name !== "$schema")) as Species);
  return { schema: MENAGERIE_SCHEMA, id: ensemble.id, title: ensemble.title, species: members, bonds: ensemble.bonds, casts: ensemble.casts, chemistry: ensemble.chemistry };
}
