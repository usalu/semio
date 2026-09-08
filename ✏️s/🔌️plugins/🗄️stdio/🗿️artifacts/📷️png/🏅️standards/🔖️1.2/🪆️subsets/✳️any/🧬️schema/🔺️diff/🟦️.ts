/** 🔺️ PngDiff schema facet — mirrors 🦀️.rs field-for-field. No full-replace slot:
 * IHDR/tRNS/ancillary fields are sparse top-level patches (tri-state `T | null` for the
 * genuinely optional ones — `null` means "removed"); `plte`/`textChunks`/`chunkOrder`/
 * `unknownChunks` are index-keyed removed/modified/added triples. */

import type {
  PngBackground, PngChromaticities, PngChunk, PngChunkMarker, PngColorType, PngPhysicalDims,
  PngRgb, PngSrgbIntent, PngTextChunk, PngTextKind, PngTimestamp, PngTransparency,
} from '../📸️snapshot/🟦️.ts';

export interface PngPlteEntryModified { index: number; rgb: PngRgb }
export interface PngPlteEntryAdded { index: number; rgb: PngRgb }

/** Nested inside `PngDiff.plte`'s outer tri-state: absent = unchanged; `null` = the PLTE
 * chunk was removed entirely; present = the palette is present (new or entry-changed). */
export interface PngPlteDiff {
  removed?: number[];
  modified?: PngPlteEntryModified[];
  added?: PngPlteEntryAdded[];
}

/** Sparse per-field patch for one `PngTextChunk`. */
export interface PngTextChunkDiff {
  keyword?: string;
  value?: string;
  compressed?: boolean;
  kind?: PngTextKind;
  languageTag?: string;
  translatedKeyword?: string;
}

export interface PngTextChunkModified { index: number; diff: PngTextChunkDiff }
export interface PngTextChunkAdded { index: number; chunk: PngTextChunk }

export interface PngTextChunksDiff {
  removed?: number[];
  modified?: PngTextChunkModified[];
  added?: PngTextChunkAdded[];
}

export interface PngUnknownChunkModified { index: number; chunk: PngChunk }
export interface PngUnknownChunkAdded { index: number; chunk: PngChunk }

export interface PngUnknownChunksDiff {
  removed?: number[];
  modified?: PngUnknownChunkModified[];
  added?: PngUnknownChunkAdded[];
}

export interface PngChunkOrderModified { index: number; marker: PngChunkMarker }
export interface PngChunkOrderAdded { index: number; marker: PngChunkMarker }

export interface PngChunkOrderDiff {
  removed?: number[];
  modified?: PngChunkOrderModified[];
  added?: PngChunkOrderAdded[];
}

/** 🔺️ Sparse diff for `stdio.png`. Every field present = changed to a value; tri-state
 * fields (`trns`/`gama`/`chrm`/`srgb`/`phys`/`time`/`bkgd`/`plte`) use `null` for "cleared". */
export interface PngDiff {
  width?: number;
  height?: number;
  bitDepth?: number;
  colorType?: PngColorType;
  interlace?: boolean;
  plte?: PngPlteDiff | null;
  trns?: PngTransparency | null;
  gama?: number | null;
  chrm?: PngChromaticities | null;
  srgb?: PngSrgbIntent | null;
  phys?: PngPhysicalDims | null;
  time?: PngTimestamp | null;
  bkgd?: PngBackground | null;
  textChunks?: PngTextChunksDiff;
  pixels?: number[];
  chunkOrder?: PngChunkOrderDiff;
  unknownChunks?: PngUnknownChunksDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPng12AnyDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPng12AnyDiffGuardReject = (at: string, why: string): never => {
  throw new stdioPng12AnyDiffGuardRefusal(at, why);
};

type stdioPng12AnyDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPng12AnyDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPng12AnyDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPng12AnyDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPng12AnyDiffGuardReject(at, "value is not an object");
export const stdioPng12AnyDiffGuardArray = (value: unknown, at: string, bounds: stdioPng12AnyDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPng12AnyDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPng12AnyDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPng12AnyDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPng12AnyDiffGuardString = (value: unknown, at: string, bounds: stdioPng12AnyDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPng12AnyDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPng12AnyDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPng12AnyDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPng12AnyDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPng12AnyDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPng12AnyDiffGuardReject(at, "value is not a boolean"));
export const stdioPng12AnyDiffGuardNumber = (value: unknown, at: string, bounds: stdioPng12AnyDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPng12AnyDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPng12AnyDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPng12AnyDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPng12AnyDiffGuardInteger = (value: unknown, at: string, bounds: stdioPng12AnyDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPng12AnyDiffGuardNumber(value, at, bounds) : stdioPng12AnyDiffGuardReject(at, "value is not an integer");
export const stdioPng12AnyDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPng12AnyDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPng12AnyDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPng12AnyDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePngPlteDiff(value: unknown, at = "$"): PngPlteDiff {
  const row = stdioPng12AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPng12AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePngPlteEntryModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePngPlteEntryAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePngTextChunkModified(value: unknown, at = "$"): PngTextChunkModified {
  const row = stdioPng12AnyDiffGuardObject(value, at);
  return {
    index: stdioPng12AnyDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parsePngTextChunkDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePngTextChunksDiff(value: unknown, at = "$"): PngTextChunksDiff {
  const row = stdioPng12AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPng12AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePngTextChunkModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePngTextChunkAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePngUnknownChunksDiff(value: unknown, at = "$"): PngUnknownChunksDiff {
  const row = stdioPng12AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPng12AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePngUnknownChunkModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePngUnknownChunkAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePngChunkOrderDiff(value: unknown, at = "$"): PngChunkOrderDiff {
  const row = stdioPng12AnyDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPng12AnyDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePngChunkOrderModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPng12AnyDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePngChunkOrderAdded(item, `${at}.added[${index}]`)),
  };
}
