/** 🔺️ PdfDiff (1.4) — a handcrafted sparse diff over the document's page tree, mirroring the Rust
 *  `PdfDiff` shape 1:1. `removed`/`modified` indices address the BASE state (removals applied
 *  descending); `added` indices address the FINAL state (insertions applied ascending). There is
 *  no `snapshot?: PdfSnapshot` full-replace slot; every change stays page-addressed. */
import type { PageDoc } from '../📸️snapshot/🟦️.ts';

export interface PdfPageDiff {
  width?: number;
  height?: number;
  text?: string;
}
export interface PdfPageModified {
  index: number;
  diff: PdfPageDiff;
}
export interface PdfPageAdded {
  index: number;
  page: PageDoc;
}
export interface PdfPagesDiff {
  removed?: number[];
  modified?: PdfPageModified[];
  added?: PdfPageAdded[];
}
export interface PdfDiff {
  pages?: PdfPagesDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf14BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf14BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioPdf14BaseDiffGuardRefusal(at, why);
};

type stdioPdf14BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf14BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf14BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf14BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf14BaseDiffGuardReject(at, "value is not an object");
export const stdioPdf14BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioPdf14BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf14BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf14BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf14BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf14BaseDiffGuardString = (value: unknown, at: string, bounds: stdioPdf14BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf14BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf14BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf14BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf14BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf14BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf14BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioPdf14BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioPdf14BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf14BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf14BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf14BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf14BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioPdf14BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf14BaseDiffGuardNumber(value, at, bounds) : stdioPdf14BaseDiffGuardReject(at, "value is not an integer");
export const stdioPdf14BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf14BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf14BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf14BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfDiff(value: unknown, at = "$"): PdfDiff {
  const row = stdioPdf14BaseDiffGuardObject(value, at);
  return {
    pages: row["pages"] === undefined ? undefined : parsePdfPagesDiff(row["pages"], `${at}.pages`),
  };
}

export function parsePdfPageDiff(value: unknown, at = "$"): PdfPageDiff {
  const row = stdioPdf14BaseDiffGuardObject(value, at);
  return {
    width: row["width"] === undefined ? undefined : stdioPdf14BaseDiffGuardNumber(row["width"], `${at}.width`),
    height: row["height"] === undefined ? undefined : stdioPdf14BaseDiffGuardNumber(row["height"], `${at}.height`),
    text: row["text"] === undefined ? undefined : stdioPdf14BaseDiffGuardString(row["text"], `${at}.text`),
  };
}

export function parsePdfPageModified(value: unknown, at = "$"): PdfPageModified {
  const row = stdioPdf14BaseDiffGuardObject(value, at);
  return {
    index: stdioPdf14BaseDiffGuardInteger(row["index"], `${at}.index`, {"minimum": 0}),
    diff: parsePdfPageDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePdfPagesDiff(value: unknown, at = "$"): PdfPagesDiff {
  const row = stdioPdf14BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPdf14BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPdf14BaseDiffGuardInteger(item, `${at}.removed[${index}]`, {"minimum": 0})),
    modified: row["modified"] === undefined ? undefined : stdioPdf14BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePdfPageModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPdf14BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePdfPageAdded(item, `${at}.added[${index}]`)),
  };
}
