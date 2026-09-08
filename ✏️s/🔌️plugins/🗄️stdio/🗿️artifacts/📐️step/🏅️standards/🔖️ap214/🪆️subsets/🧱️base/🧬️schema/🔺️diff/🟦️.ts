/** 🔺️ StepDiff schema — handcrafted sparse diff mirroring the Rust `StepDiff` shape 1:1. Three
 * HEADER records are whole-value replaced (weak, per the recipe); `entities` is an id-keyed
 * triple whose `args` sub-diff is a SEPARATE index-keyed triple (Part-21 argument lists are
 * positional). */

import type { StepComplexType, StepEntity, StepFileDescription, StepFileName, StepFileSchema, StepValue } from '../📸️snapshot/🟦️.ts';

/** 🔺️ One `args.modified[]`/`args.added[]` entry — `StepValue` is weak, so the "diff" IS the
 * whole new value. */
export interface StepArgModified {
  index: number;
  value: StepValue;
}
export interface StepArgAdded {
  index: number;
  value: StepValue;
}

/** 🔺️ Index-keyed collection triple for `StepEntity.args`. */
export interface StepArgsDiff {
  removed?: number[];
  modified?: StepArgModified[];
  added?: StepArgAdded[];
}

/** 🔺️ Sparse per-field diff for one `StepEntity`. `complex` is a weak value list, whole-vec
 * replaced. */
export interface StepEntityDiff {
  name?: string;
  args?: StepArgsDiff;
  complex?: StepComplexType[];
}

/** 📦️ One `entities.modified[]` entity — `id` is stable Part-21 instance-number identity. */
export interface StepEntityModified {
  id: number;
  diff: StepEntityDiff;
}

/** 📦️ One `entities.added[]` entity — `index` is the position in the FINAL sequence. */
export interface StepEntityAdded {
  index: number;
  entity: StepEntity;
}

/** 📦️ Sparse id-keyed `entities` triple. */
export interface StepEntitiesDiff {
  removed?: number[];
  modified?: StepEntityModified[];
  added?: StepEntityAdded[];
}

/** 🔺️ Diff for `stdio.step`. `schema` is an identity field and never appears here. */
export interface StepDiff {
  fileDescription?: StepFileDescription;
  fileName?: StepFileName;
  fileSchema?: StepFileSchema;
  entities?: StepEntitiesDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStepAp214BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStepAp214BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioStepAp214BaseDiffGuardRefusal(at, why);
};

type stdioStepAp214BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStepAp214BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStepAp214BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStepAp214BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStepAp214BaseDiffGuardReject(at, "value is not an object");
export const stdioStepAp214BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStepAp214BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStepAp214BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStepAp214BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStepAp214BaseDiffGuardString = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStepAp214BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStepAp214BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStepAp214BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStepAp214BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStepAp214BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStepAp214BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioStepAp214BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStepAp214BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStepAp214BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStepAp214BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStepAp214BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioStepAp214BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStepAp214BaseDiffGuardNumber(value, at, bounds) : stdioStepAp214BaseDiffGuardReject(at, "value is not an integer");
export const stdioStepAp214BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStepAp214BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStepAp214BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStepAp214BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStepDiff(value: unknown, at = "$"): StepDiff {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    fileDescription: row["fileDescription"] === undefined ? undefined : parseStepFileDescription(row["fileDescription"], `${at}.fileDescription`),
    fileName: row["fileName"] === undefined ? undefined : parseStepFileName(row["fileName"], `${at}.fileName`),
    fileSchema: row["fileSchema"] === undefined ? undefined : parseStepFileSchema(row["fileSchema"], `${at}.fileSchema`),
    entities: row["entities"] === undefined ? undefined : parseStepEntitiesDiff(row["entities"], `${at}.entities`),
  };
}

export type StepValue = StepValue;

export function parseStepValue(value: unknown, at = "$"): StepValue {
  return parseStepValue(value, `${at}`);
}

export type StepComplexType = StepComplexType;

export function parseStepComplexType(value: unknown, at = "$"): StepComplexType {
  return parseStepComplexType(value, `${at}`);
}

export type StepEntity = StepEntity;

export function parseStepEntity(value: unknown, at = "$"): StepEntity {
  return parseStepEntity(value, `${at}`);
}

export type StepFileDescription = StepFileDescription;

export function parseStepFileDescription(value: unknown, at = "$"): StepFileDescription {
  return parseStepFileDescription(value, `${at}`);
}

export type StepFileName = StepFileName;

export function parseStepFileName(value: unknown, at = "$"): StepFileName {
  return parseStepFileName(value, `${at}`);
}

export type StepFileSchema = StepFileSchema;

export function parseStepFileSchema(value: unknown, at = "$"): StepFileSchema {
  return parseStepFileSchema(value, `${at}`);
}

export function parseStepArgModified(value: unknown, at = "$"): StepArgModified {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    index: stdioStepAp214BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    value: parseStepValue(row["value"], `${at}.value`),
  };
}

export function parseStepArgAdded(value: unknown, at = "$"): StepArgAdded {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    index: stdioStepAp214BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    value: parseStepValue(row["value"], `${at}.value`),
  };
}

export function parseStepArgsDiff(value: unknown, at = "$"): StepArgsDiff {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioStepAp214BaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseStepArgModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseStepArgAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseStepEntityDiff(value: unknown, at = "$"): StepEntityDiff {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    name: row["name"] === undefined ? undefined : stdioStepAp214BaseDiffGuardString(row["name"], `${at}.name`),
    args: row["args"] === undefined ? undefined : parseStepArgsDiff(row["args"], `${at}.args`),
    complex: row["complex"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["complex"], `${at}.complex`).map((item, index) => parseStepComplexType(item, `${at}.complex[${index}]`)),
  };
}

export function parseStepEntityModified(value: unknown, at = "$"): StepEntityModified {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    id: stdioStepAp214BaseDiffGuardInteger(row["id"], `${at}.id`, {"minimum": 0}),
    diff: parseStepEntityDiff(row["diff"], `${at}.diff`),
  };
}

export function parseStepEntityAdded(value: unknown, at = "$"): StepEntityAdded {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    index: stdioStepAp214BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    entity: parseStepEntity(row["entity"], `${at}.entity`),
  };
}

export function parseStepEntitiesDiff(value: unknown, at = "$"): StepEntitiesDiff {
  const row = stdioStepAp214BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioStepAp214BaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseStepEntityModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioStepAp214BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseStepEntityAdded(item, `${at}.added[${index}]`)),
  };
}
