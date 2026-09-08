/** 🔺️ PlyDiff schema — handcrafted sparse diff mirroring the Rust `PlyDiff` shape 1:1. Two
 * collection levels nest: `elements` (name-keyed) → each modified element's `rows` (index-keyed). */

import type { PlyElement, PlyFormat, PlyProperty, PlyRow, PlyValue } from '../📸️snapshot/🟦️.ts';

/** 🔣️ One changed cell, keyed by the owning element's property NAME. */
export interface PlyRowFieldChange {
  name: string;
  value: PlyValue;
}

/** 🔺️ Sparse per-property patch for one row. */
export interface PlyRowDiff {
  fields?: PlyRowFieldChange[];
}

/** 📦️ One `rows.modified[]` entity — `index` is the row's position in BASE. */
export interface PlyRowModified {
  index: number;
  diff: PlyRowDiff;
}

/** 📦️ One `rows.added[]` entity — `index` is the row's position in the FINAL sequence. */
export interface PlyRowAdded {
  index: number;
  row: PlyRow;
}

/** 🔺️ Index-keyed removed/modified/added triple over one element's `rows`. */
export interface PlyRowsDiff {
  removed?: number[];
  modified?: PlyRowModified[];
  added?: PlyRowAdded[];
}

/** 🔺️ Sparse per-field patch for one element. `properties` is a weak value-list — whole-vec
 * replaced, never sub-diffed. */
export interface PlyElementDiff {
  properties?: PlyProperty[];
  rows?: PlyRowsDiff;
}

/** 📦️ One `elements.modified[]` entity — `name` is the element's identity. */
export interface PlyElementModified {
  name: string;
  diff: PlyElementDiff;
}

/** 📦️ One `elements.added[]` entity — `index` is the position in the FINAL sequence. */
export interface PlyElementAdded {
  index: number;
  element: PlyElement;
}

/** 🔺️ Sparse name-keyed `elements` triple. */
export interface PlyElementsDiff {
  removed?: string[];
  modified?: PlyElementModified[];
  added?: PlyElementAdded[];
}

/** 🔺️ Diff for `stdio.ply`. `schema` is an identity field and never appears here. */
export interface PlyDiff {
  format?: PlyFormat;
  comments?: string[];
  elements?: PlyElementsDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPly10AnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPly10AnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioPly10AnyDiffGuardRefusal(at, why);
};

type stdioPly10AnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPly10AnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPly10AnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPly10AnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPly10AnyDiffGuardReject(at, "value is not an object");
export const stdioPly10AnyDiffGuardArray = (value: unknown, at: string, bounds: stdioPly10AnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPly10AnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPly10AnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPly10AnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPly10AnyDiffGuardString = (value: unknown, at: string, bounds: stdioPly10AnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPly10AnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPly10AnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPly10AnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPly10AnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPly10AnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPly10AnyDiffGuardReject(at, "value is not a boolean"));
export const stdioPly10AnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioPly10AnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPly10AnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPly10AnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPly10AnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPly10AnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioPly10AnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPly10AnyDiffGuardNumber(value, at, bounds) : stdioPly10AnyDiffGuardReject(at, "value is not an integer");
export const stdioPly10AnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPly10AnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPly10AnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPly10AnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlyDiff(value: unknown, at = "$"): PlyDiff {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    format: row["format"] === undefined ? undefined : stdioPly10AnyDiffGuardMember(row["format"], `${at}.format`, ["ascii", "binaryLittleEndian", "binaryBigEndian"] as const),
    comments: row["comments"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["comments"], `${at}.comments`).map((item, index) => stdioPly10AnyDiffGuardString(item, `${at}.comments[${index}]`)),
    elements: row["elements"] === undefined ? undefined : parsePlyElementsDiff(row["elements"], `${at}.elements`),
  };
}

export function parsePlyRowFieldChange(value: unknown, at = "$"): PlyRowFieldChange {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    name: stdioPly10AnyDiffGuardString(row["name"], `${at}.name`),
    value: parsePlyValue(row["value"], `${at}.value`),
  };
}

export function parsePlyRowDiff(value: unknown, at = "$"): PlyRowDiff {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    fields: row["fields"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["fields"], `${at}.fields`).map((item, index) => parsePlyRowFieldChange(item, `${at}.fields[${index}]`)),
  };
}

export function parsePlyRowModified(value: unknown, at = "$"): PlyRowModified {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    index: stdioPly10AnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parsePlyRowDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePlyRowsDiff(value: unknown, at = "$"): PlyRowsDiff {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPly10AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePlyRowModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePlyRowAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePlyElementModified(value: unknown, at = "$"): PlyElementModified {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    name: stdioPly10AnyDiffGuardString(row["name"], `${at}.name`),
    diff: parsePlyElementDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePlyElementsDiff(value: unknown, at = "$"): PlyElementsDiff {
  const row = stdioPly10AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPly10AnyDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePlyElementModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPly10AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePlyElementAdded(item, `${at}.added[${index}]`)),
  };
}
