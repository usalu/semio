/** 🟰️ Projection comparison
 *
 * Canonicalization, normalization and the structural diff of two projections under a comparison profile.
 */

//#region 🔌️Adapters
import { type ComparisonProfile } from "../../../../../🔨️modules/🧪️test/🔌️adapter/🟦️.ts";
import { type ComparisonProfileSpec, CORE_COMPARISON_PROFILES } from "../🗂️taxonomy/🟦️.ts";
import { digest } from "../🔢️digests/🟦️.ts";
//#endregion 🔌️Adapters

//#region ⚖️Comparison
/** ⚖️ One field-level difference between an oracle projection and a subject projection. */
export type ComparisonDiff = Readonly<{ path: string; oracle: unknown; subject: unknown; reason: string }>;

/** ⚖️ The verdict of one comparison profile over two projections. */
export type ComparisonVerdict = Readonly<{ profile: ComparisonProfile; equal: boolean; diffs: readonly ComparisonDiff[] }>;

export function isPlainObject(value: unknown): value is Record<string, unknown> {
  return typeof value === "object" && value !== null && !Array.isArray(value);
}

/** ⚖️ Sorts object keys recursively so serialization order is never mistaken for a semantic difference. */
export function canonicalize(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonicalize);
  if (isPlainObject(value))
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, canonicalize(value[key])]),
    );
  return value;
}

function sortDeep(value: unknown): unknown {
  if (Array.isArray(value)) return [...value.map(sortDeep)].sort((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)));
  if (isPlainObject(value))
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .map((key) => [key, sortDeep(value[key])]),
    );
  return value;
}

function stripKeys(value: unknown, keys: ReadonlySet<string>): unknown {
  if (keys.size === 0) return value;
  if (Array.isArray(value)) return value.map((entry) => stripKeys(entry, keys));
  if (isPlainObject(value))
    return Object.fromEntries(
      Object.entries(value)
        .filter(([key]) => !keys.has(key))
        .map(([key, entry]) => [key, stripKeys(entry, keys)]),
    );
  return value;
}

function roundFloats(value: unknown, tolerance: number): unknown {
  if (tolerance <= 0) return value;
  if (typeof value === "number") return Math.round(value / tolerance) * tolerance;
  if (Array.isArray(value)) return value.map((entry) => roundFloats(entry, tolerance));
  if (isPlainObject(value)) return Object.fromEntries(Object.entries(value).map(([key, entry]) => [key, roundFloats(entry, tolerance)]));
  return value;
}

function normalizeText(value: unknown): string {
  return String(value)
    .replace(/\r\n/g, "\n")
    .replace(/[ \t]+$/gm, "")
    .replace(/\n+$/, "")
    .normalize("NFC");
}

function diffValues(path: string, oracle: unknown, subject: unknown, out: ComparisonDiff[], tolerance: number): void {
  if (typeof oracle === "number" && typeof subject === "number") {
    if (Math.abs(oracle - subject) > tolerance) out.push({ path, oracle, subject, reason: `numbers differ by more than ${tolerance}` });
    return;
  }
  if (Array.isArray(oracle) && Array.isArray(subject)) {
    if (oracle.length !== subject.length) out.push({ path, oracle: oracle.length, subject: subject.length, reason: "array length differs" });
    for (let i = 0; i < Math.max(oracle.length, subject.length); i += 1) diffValues(`${path}[${i}]`, oracle[i], subject[i], out, tolerance);
    return;
  }
  if (isPlainObject(oracle) && isPlainObject(subject)) {
    for (const key of new Set([...Object.keys(oracle), ...Object.keys(subject)])) {
      if (!(key in oracle)) out.push({ path: `${path}.${key}`, oracle: undefined, subject: subject[key], reason: "present only in subject" });
      else if (!(key in subject)) out.push({ path: `${path}.${key}`, oracle: oracle[key], subject: undefined, reason: "present only in oracle" });
      else diffValues(`${path}.${key}`, oracle[key], subject[key], out, tolerance);
    }
    return;
  }
  if (JSON.stringify(oracle) !== JSON.stringify(subject)) out.push({ path, oracle, subject, reason: "values differ" });
}

/** ⚖️ Applies a profile's declared policy to one projection, producing its canonical form. */
function projectUnder(spec: ComparisonProfileSpec, value: unknown): unknown {
  const stripped = stripKeys(value, new Set(spec.ignoreKeys ?? []));
  const rounded = roundFloats(stripped, spec.tolerance ?? 0);
  return spec.arrays === "set" ? sortDeep(rounded) : canonicalize(rounded);
}

/**
 * ⚖️ Applies one comparison profile. The MECHANISM lives here; the per-format policy is data, so a
 * new format needs a manifest entry rather than a new branch. Comparison never lives in an adapter,
 * so two implementations cannot quietly agree on different notions of "the same result".
 */
export function compareProjections(profile: ComparisonProfile, oracle: unknown, subject: unknown, profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): ComparisonVerdict {
  const spec = profiles.get(profile);
  if (spec === undefined) return { profile, equal: false, diffs: [{ path: "$", oracle: profile, subject: [...profiles.keys()].sort(), reason: `unknown comparison profile ${JSON.stringify(profile)} — register it in an owner's declared oracle contribution manifest` }] };
  const diffs: ComparisonDiff[] = [];
  if (spec.bytes === true) {
    const left = typeof oracle === "string" ? oracle : JSON.stringify(oracle);
    const right = typeof subject === "string" ? subject : JSON.stringify(subject);
    if (left !== right) diffs.push({ path: "$", oracle: digest(left), subject: digest(right), reason: "byte-exact comparison failed" });
    return { profile, equal: diffs.length === 0, diffs };
  }
  if (spec.text === "utf8") {
    if (normalizeText(oracle) !== normalizeText(subject)) diffs.push({ path: "$", oracle: normalizeText(oracle), subject: normalizeText(subject), reason: "normalized text differs" });
    return { profile, equal: diffs.length === 0, diffs };
  }
  diffValues("$", projectUnder(spec, oracle), projectUnder(spec, subject), diffs, spec.tolerance ?? 0);
  return { profile, equal: diffs.length === 0, diffs };
}

/** ⚖️ Digest of a projection under a profile's canonical form — the value results carry as `projectionHash`. */
export function projectionHash(profile: ComparisonProfile, projection: unknown, profiles: ReadonlyMap<string, ComparisonProfileSpec> = coreProfileTable()): string {
  const spec = profiles.get(profile);
  if (spec === undefined) return digest(JSON.stringify(canonicalize(projection)));
  if (spec.bytes === true) return digest(typeof projection === "string" ? projection : JSON.stringify(projection));
  if (spec.text === "utf8") return digest(normalizeText(projection));
  return digest(JSON.stringify(projectUnder(spec, projection)));
}

let coreProfileTableCache: ReadonlyMap<string, ComparisonProfileSpec> | null = null;

/** ⚖️ The framework's own domain-neutral profile table, for callers with no contributions loaded. */
export function coreProfileTable(): ReadonlyMap<string, ComparisonProfileSpec> {
  coreProfileTableCache ??= new Map(CORE_COMPARISON_PROFILES.map((spec) => [spec.id, spec]));
  return coreProfileTableCache;
}
//#endregion ⚖️Comparison
