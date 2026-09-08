/** 🔺️ PdfDiff (1.7) — handcrafted sparse diff mirroring the Rust `PdfDiff` shape 1:1. `pages` is
 *  an index-keyed triple of flat `PdfPageDiff` patches; `objects` is an `ObjRef`-keyed (the
 *  `(id,gen)` pair) triple of recursive `PdfValueDiff` patches mirroring `PdfObject`'s shape
 *  (`Replace` on node-KIND change, direct field/collection diff when the kind is stable); `trailer`
 *  reuses `PdfDictDiff` verbatim (trailer is itself a Dict-shaped structure). No
 *  `snapshot?: PdfSnapshot` full-replace slot anywhere. */

import type { ObjRef, PdfDecimal, PdfDictEntry, PdfInfo, PdfObject, PdfPage, PdfStreamFilter } from '../📸️snapshot/🟦️.ts';

/** 📄️ Sparse per-field patch for one `PdfPage` (weak entity -- flat fields only). `cropBox` is
 *  tri-state: absent = unchanged, `null` = cleared, a box = set. */
export interface PdfPageDiff {
  mediaBox?: [number, number, number, number];
  cropBox?: [number, number, number, number] | null;
  rotate?: number;
  text?: string;
}

export interface PdfPageModified {
  index: number;
  diff: PdfPageDiff;
}
export interface PdfPageAdded {
  index: number;
  page: PdfPage;
}
/** 📦️ Index-keyed `pages` triple. */
export interface PdfPagesDiff {
  removed?: number[];
  modified?: PdfPageModified[];
  added?: PdfPageAdded[];
}

export interface PdfDictModified {
  key: string;
  diff: PdfValueDiff;
}
export interface PdfDictAdded {
  index: number;
  key: string;
  item: PdfObject;
}
/** 📦️ Name-keyed `Dict`/`Stream.dict`/`trailer` triple (reused verbatim for the top-level
 *  `PdfDiff.trailer` field -- same shape, same semantics). */
export interface PdfDictDiff {
  removed?: string[];
  modified?: PdfDictModified[];
  added?: PdfDictAdded[];
}

export interface PdfArrayModified {
  index: number;
  diff: PdfValueDiff;
}
export interface PdfArrayAdded {
  index: number;
  item: PdfObject;
}
/** 📦️ Index-keyed `Array` triple. */
export interface PdfArrayDiff {
  removed?: number[];
  modified?: PdfArrayModified[];
  added?: PdfArrayAdded[];
}

/** 🔺️ Recursive logical diff mirroring `PdfObject`'s shape. */
export type PdfValueDiff =
  | { kind: 'replace'; value: PdfObject }
  | { kind: 'bool'; value: boolean }
  | { kind: 'int'; value: number }
  | { kind: 'real'; value: PdfDecimal }
  | { kind: 'str'; value: number[] }
  | { kind: 'name'; value: string }
  | { kind: 'ref'; value: ObjRef }
  | { kind: 'array'; diff: PdfArrayDiff }
  | { kind: 'dict'; diff: PdfDictDiff }
  | { kind: 'stream'; dict?: PdfDictDiff; data?: number[]; filters?: PdfStreamFilter[] };

export interface PdfObjectModified {
  id: ObjRef;
  diff: PdfValueDiff;
}
export interface PdfObjectAdded {
  index: number;
  id: ObjRef;
  value: PdfObject;
}
/** 📦️ `(id,gen)`-keyed `objects` triple. */
export interface PdfObjectsDiff {
  removed?: ObjRef[];
  modified?: PdfObjectModified[];
  added?: PdfObjectAdded[];
}

/** 🧭️ One step of a `NodePath`-style address into ONE object's `PdfObject` tree (used by
 *  `setDictEntry`/`removeDictEntry` mutations). Only `path == []` can address a `Stream`'s dict
 *  (a raw Stream can only ever be an indirect object's OWN top-level value per ISO 32000-1). */
export type PdfPathSegment =
  | { kind: 'arrayIndex'; index: number }
  | { kind: 'dictKey'; key: string };

/** 🔺️ Diff for `stdio.pdf.1.7`. `schema` is an identity field and never appears here. `info` is
 *  a WEAK value struct (whole-value replaced, never sub-diffed). */
export interface PdfDiff {
  /** @state artifact */ declaredVersion?: string;
  /** @state artifact */ info?: PdfInfo;
  /** @state artifact */ pages?: PdfPagesDiff;
  /** @state artifact */ objects?: PdfObjectsDiff;
  /** @state artifact */ trailer?: PdfDictDiff;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf17BaseDiffGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf17BaseDiffGuardReject = (at: string, why: string): never => {
  throw new stdioPdf17BaseDiffGuardRefusal(at, why);
};

type stdioPdf17BaseDiffGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf17BaseDiffGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf17BaseDiffGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf17BaseDiffGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf17BaseDiffGuardReject(at, "value is not an object");
export const stdioPdf17BaseDiffGuardArray = (value: unknown, at: string, bounds: stdioPdf17BaseDiffGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf17BaseDiffGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf17BaseDiffGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf17BaseDiffGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf17BaseDiffGuardString = (value: unknown, at: string, bounds: stdioPdf17BaseDiffGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf17BaseDiffGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf17BaseDiffGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf17BaseDiffGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf17BaseDiffGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf17BaseDiffGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf17BaseDiffGuardReject(at, "value is not a boolean"));
export const stdioPdf17BaseDiffGuardNumber = (value: unknown, at: string, bounds: stdioPdf17BaseDiffGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf17BaseDiffGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf17BaseDiffGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf17BaseDiffGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf17BaseDiffGuardInteger = (value: unknown, at: string, bounds: stdioPdf17BaseDiffGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf17BaseDiffGuardNumber(value, at, bounds) : stdioPdf17BaseDiffGuardReject(at, "value is not an integer");
export const stdioPdf17BaseDiffGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf17BaseDiffGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf17BaseDiffGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf17BaseDiffGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export interface ObjRef {
  readonly num: number;
  readonly gen: number;
}

export function parseObjRef(value: unknown, at = "$"): ObjRef {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    num: stdioPdf17BaseDiffGuardInteger(row["num"], `${at}.num`),
    gen: stdioPdf17BaseDiffGuardInteger(row["gen"], `${at}.gen`),
  };
}

export interface PdfStreamFilter {
  readonly kind: "flate" | "asciiHex" | "ascii85" | "runLength";
  readonly predictor?: Readonly<Record<string, unknown>>;
}

export function parsePdfStreamFilter(value: unknown, at = "$"): PdfStreamFilter {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    kind: stdioPdf17BaseDiffGuardMember(row["kind"], `${at}.kind`, ["flate", "asciiHex", "ascii85", "runLength"] as const),
    predictor: row["predictor"] === undefined ? undefined : stdioPdf17BaseDiffGuardObject(row["predictor"], `${at}.predictor`),
  };
}

export interface PdfPage {
  readonly mediaBox?: readonly number[];
  readonly cropBox?: readonly number[];
  readonly rotate?: number;
  readonly text?: string;
}

export function parsePdfPage(value: unknown, at = "$"): PdfPage {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    mediaBox: row["mediaBox"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["mediaBox"], `${at}.mediaBox`).map((item, index) => stdioPdf17BaseDiffGuardNumber(item, `${at}.mediaBox[${index}]`)),
    cropBox: row["cropBox"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["cropBox"], `${at}.cropBox`).map((item, index) => stdioPdf17BaseDiffGuardNumber(item, `${at}.cropBox[${index}]`)),
    rotate: row["rotate"] === undefined ? undefined : stdioPdf17BaseDiffGuardInteger(row["rotate"], `${at}.rotate`),
    text: row["text"] === undefined ? undefined : stdioPdf17BaseDiffGuardString(row["text"], `${at}.text`),
  };
}

export function parsePdfPageModified(value: unknown, at = "$"): PdfPageModified {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    index: stdioPdf17BaseDiffGuardInteger(row["index"], `${at}.index`),
    diff: parsePdfPageDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePdfPageAdded(value: unknown, at = "$"): PdfPageAdded {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    index: stdioPdf17BaseDiffGuardInteger(row["index"], `${at}.index`),
    page: parsePdfPage(row["page"], `${at}.page`),
  };
}

export function parsePdfPagesDiff(value: unknown, at = "$"): PdfPagesDiff {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPdf17BaseDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePdfPageModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePdfPageAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePdfDictModified(value: unknown, at = "$"): PdfDictModified {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    key: stdioPdf17BaseDiffGuardString(row["key"], `${at}.key`),
    diff: parsePdfValueDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePdfDictDiff(value: unknown, at = "$"): PdfDictDiff {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPdf17BaseDiffGuardString(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePdfDictModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePdfDictAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePdfArrayModified(value: unknown, at = "$"): PdfArrayModified {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    index: stdioPdf17BaseDiffGuardInteger(row["index"], `${at}.index`),
    diff: parsePdfValueDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePdfArrayDiff(value: unknown, at = "$"): PdfArrayDiff {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => stdioPdf17BaseDiffGuardInteger(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePdfArrayModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePdfArrayAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePdfObjectModified(value: unknown, at = "$"): PdfObjectModified {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    id: parseObjRef(row["id"], `${at}.id`),
    diff: parsePdfValueDiff(row["diff"], `${at}.diff`),
  };
}

export function parsePdfObjectsDiff(value: unknown, at = "$"): PdfObjectsDiff {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    removed: row["removed"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["removed"], `${at}.removed`).map((item, index) => parseObjRef(item, `${at}.removed[${index}]`)),
    modified: row["modified"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["modified"], `${at}.modified`).map((item, index) => parsePdfObjectModified(item, `${at}.modified[${index}]`)),
    added: row["added"] === undefined ? undefined : stdioPdf17BaseDiffGuardArray(row["added"], `${at}.added`).map((item, index) => parsePdfObjectAdded(item, `${at}.added[${index}]`)),
  };
}

export function parsePdfPathSegment(value: unknown, at = "$"): PdfPathSegment {
  const row = stdioPdf17BaseDiffGuardObject(value, at);
  return {
    kind: stdioPdf17BaseDiffGuardMember(row["kind"], `${at}.kind`, ["arrayIndex", "dictKey"] as const),
    index: row["index"] === undefined ? undefined : stdioPdf17BaseDiffGuardInteger(row["index"], `${at}.index`),
    key: row["key"] === undefined ? undefined : stdioPdf17BaseDiffGuardString(row["key"], `${at}.key`),
  };
}
