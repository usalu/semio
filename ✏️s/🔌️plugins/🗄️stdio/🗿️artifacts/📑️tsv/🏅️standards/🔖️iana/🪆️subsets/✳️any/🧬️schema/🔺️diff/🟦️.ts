/** 🔺️ TsvDiff schema facet — mirrors 🦀️.rs field-for-field. `records` is an
 * index-keyed removed/modified/added triple with a sparse positional per-column patch. */
import type { TsvLineEnding } from '../📸️snapshot/🟦️.ts';

/** Positional per-column patch list; `null` at a position means that column is unchanged. */
export interface TsvRowDiff {
  fields?: (string | null)[];
}

export interface TsvRowModified { index: number; diff: TsvRowDiff; }
export interface TsvRowAdded { index: number; row: string[]; }
export interface TsvRowsDiff { removed?: number[]; modified?: TsvRowModified[]; added?: TsvRowAdded[]; }

export interface TsvDiff {
  trailingNewline?: boolean;
  lineEnding?: TsvLineEnding;
  records?: TsvRowsDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnyDiffGuardRefusal(at, why);
};

type stdioTsvIanaAnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnyDiffGuardReject(at, "value is not an object");
export const stdioTsvIanaAnyDiffGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnyDiffGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnyDiffGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnyDiffGuardNumber(value, at, bounds) : stdioTsvIanaAnyDiffGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvDiff(value: unknown, at = "$"): TsvDiff {
  const row = stdioTsvIanaAnyDiffGuardObject(value, at);
  return {
    trailingNewline: row["trailingNewline"] === undefined ? undefined : stdioTsvIanaAnyDiffGuardBoolean(row["trailingNewline"], `${at}.trailingNewline`),
    lineEnding: row["lineEnding"] === undefined ? undefined : stdioTsvIanaAnyDiffGuardMember(row["lineEnding"], `${at}.lineEnding`, ["lf", "crlf"] as const),
    records: row["records"] === undefined ? undefined : parseTsvRowsDiff(row["records"], `${at}.records`),
  };
}

export function parseTsvRowModified(value: unknown, at = "$"): TsvRowModified {
  const row = stdioTsvIanaAnyDiffGuardObject(value, at);
  return {
    index: stdioTsvIanaAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseTsvRowDiff(row["diff"], `${at}.diff`),
  };
}

export function parseTsvRowAdded(value: unknown, at = "$"): TsvRowAdded {
  const row = stdioTsvIanaAnyDiffGuardObject(value, at);
  return {
    index: stdioTsvIanaAnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    row: stdioTsvIanaAnyDiffGuardArray(row["row"], `${at}.row`).map((item, index) => stdioTsvIanaAnyDiffGuardString(item, `${at}.row[${index}]`)),
  };
}

export function parseTsvRowsDiff(value: unknown, at = "$"): TsvRowsDiff {
  const row = stdioTsvIanaAnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioTsvIanaAnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioTsvIanaAnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioTsvIanaAnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseTsvRowModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioTsvIanaAnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseTsvRowAdded(item, `${at}.added[${index}]`)),
  };
}
