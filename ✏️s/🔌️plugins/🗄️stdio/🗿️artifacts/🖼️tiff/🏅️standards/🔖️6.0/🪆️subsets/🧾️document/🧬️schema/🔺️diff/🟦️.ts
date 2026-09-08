/** 🔺️ TiffDiff schema facet — mirrors 🦀️.rs field-for-field. No full-replace slot:
 * `ifds` is an index-keyed removed/modified/added triple; within a modified IFD, `entries` is
 * a TAG-ID-keyed triple (`tag`, not array index — a `TiffTag` is a weak value, so
 * modified/added carry the whole new tag). */

import type { TiffByteOrder, TiffFieldType, TiffIfd, TiffValues } from '../📸️snapshot/🟦️.ts';

export interface TiffTagModified { tag: number; kind: TiffFieldType; values: TiffValues }
export interface TiffTagAdded { tag: number; kind: TiffFieldType; values: TiffValues }

/** Tag-id-keyed `entries` triple for one IFD. */
export interface TiffTagsDiff {
  removed?: number[];
  modified?: TiffTagModified[];
  added?: TiffTagAdded[];
}

/** One IFD's own delta: the recursive tag triple plus a whole-value slot for its raw strip payload. */
export interface TiffIfdDiff { entries: TiffTagsDiff; pixels?: number[] }
export interface TiffIfdModified { index: number; diff: TiffIfdDiff }
export interface TiffIfdAdded { index: number; ifd: TiffIfd }

/** Index-keyed `ifds` triple (TIFF's IFD chain is positional). */
export interface TiffIfdsDiff {
  removed?: number[];
  modified?: TiffIfdModified[];
  added?: TiffIfdAdded[];
}

/** 🔺️ Sparse diff for `stdio.tiff`. Every field present = changed to a value. No tri-state
 * fields at this level — `byteOrder`/`pixels` are always-present scalars, `ifds` is the only
 * collection. */
export interface TiffDiff {
  byteOrder?: TiffByteOrder;
  ifds?: TiffIfdsDiff;
  pixels?: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentDiffGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentDiffGuardRefusal(at, why);
};

type stdioTiff60DocumentDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentDiffGuardReject(at, "value is not an object");
export const stdioTiff60DocumentDiffGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentDiffGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentDiffGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentDiffGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentDiffGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentDiffGuardNumber(value, at, bounds) : stdioTiff60DocumentDiffGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffTagsDiff(value: unknown, at = "$"): TiffTagsDiff {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioTiff60DocumentDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0, "maximum": 65535})),
    modified: row["modified"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseTiffTagModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseTiffTagAdded(item, `${at}.added[${index}]`)),
  };
}

export function parseTiffIfdDiff(value: unknown, at = "$"): TiffIfdDiff {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    entries: row["entries"] === undefined ? undefined : parseTiffTagsDiff(row["entries"], `${at}.entries`),
    pixels: row["pixels"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => stdioTiff60DocumentDiffGuardInteger(item, `${at}.pixels[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}

export function parseTiffIfdModified(value: unknown, at = "$"): TiffIfdModified {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    index: stdioTiff60DocumentDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parseTiffIfdDiff(row["diff"], `${at}.diff`),
  };
}

export function parseTiffIfdsDiff(value: unknown, at = "$"): TiffIfdsDiff {
  const row = stdioTiff60DocumentDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioTiff60DocumentDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parseTiffIfdModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioTiff60DocumentDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parseTiffIfdAdded(item, `${at}.added[${index}]`)),
  };
}
