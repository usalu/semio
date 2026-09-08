/** 🧬️ PdfSnapshot (1.7) schema — logical object-graph model mirroring Rust 1:1.
 *  `objects` is the full semantic indirect-object graph; `pages` is the
 *  resolved, editable view; `trailer` is the trailer dictionary (same shape as a `Dict`). */

/** 🔗️ An indirect-object reference `N G R` -- also the `objects` collection's diff key. */
export interface ObjRef {
  num: number;
  gen: number;
}

/** 🧩 One `key`/`value` pair of a PDF dictionary (order-preserving array, not a map). */
export interface PdfDictEntry {
  key: string;
  value: PdfObject;
}

export interface PdfDecimal {
  negative: boolean;
  coefficient: string;
  scale: number;
}

export interface PdfPredictor {
  predictor: number;
  colors: number;
  bitsPerComponent: number;
  columns: number;
}

export type PdfStreamFilter =
  | { kind: 'flate'; predictor?: PdfPredictor }
  | { kind: 'asciiHex' }
  | { kind: 'ascii85' }
  | { kind: 'runLength' };

/** 🎯 Parsed PDF COS objects. Stream data is decoded; filters are typed logical concepts. */
export type PdfObject =
  | { kind: 'null' }
  | { kind: 'bool'; value: boolean }
  | { kind: 'int'; value: number }
  | { kind: 'real'; value: PdfDecimal }
  | { kind: 'str'; value: number[] }
  | { kind: 'name'; value: string }
  | { kind: 'array'; value: PdfObject[] }
  | { kind: 'dict'; value: PdfDictEntry[] }
  | { kind: 'ref'; value: ObjRef }
  | { kind: 'stream'; dict: PdfDictEntry[]; data: number[]; filters: PdfStreamFilter[] };

/** 🗄️ One `N G obj ... endobj` indirect object, keyed by `id`. */
export interface PdfIndirectObject {
  id: ObjRef;
  value: PdfObject;
}

/** 📄️ One resolved page -- inherited `/Resources`/`/MediaBox`/`/CropBox`/`/Rotate` already
 *  applied, text already extracted from its content stream(s). */
export interface PdfPage {
  mediaBox: [number, number, number, number];
  cropBox?: [number, number, number, number];
  rotate: number;
  text: string;
}

/** 📇️ Document `/Info` dictionary. */
export interface PdfInfo {
  title?: string;
  author?: string;
  subject?: string;
  keywords?: string;
  creator?: string;
  producer?: string;
}

/** 🧬️ `stdio.pdf` (1.7) persistent snapshot. */
export interface PdfSnapshot {
  /** @state artifact */ schema: string;
  /** @state artifact */ declaredVersion: string;
  /** @state artifact */ pages: PdfPage[];
  /** @state artifact */ info: PdfInfo;
  /** @state artifact */ objects: PdfIndirectObject[];
  /** @state artifact */ trailer: PdfDictEntry[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf17BaseSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf17BaseSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioPdf17BaseSnapshotGuardRefusal(at, why);
};

type stdioPdf17BaseSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf17BaseSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf17BaseSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf17BaseSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf17BaseSnapshotGuardReject(at, "value is not an object");
export const stdioPdf17BaseSnapshotGuardArray = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf17BaseSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf17BaseSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf17BaseSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf17BaseSnapshotGuardString = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf17BaseSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf17BaseSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf17BaseSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf17BaseSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf17BaseSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf17BaseSnapshotGuardReject(at, "value is not a boolean"));
export const stdioPdf17BaseSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf17BaseSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf17BaseSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf17BaseSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf17BaseSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf17BaseSnapshotGuardNumber(value, at, bounds) : stdioPdf17BaseSnapshotGuardReject(at, "value is not an integer");
export const stdioPdf17BaseSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf17BaseSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf17BaseSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf17BaseSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfSnapshot(value: unknown, at = "$"): PdfSnapshot {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    schema: stdioPdf17BaseSnapshotGuardString(row["schema"], `${at}.schema`),
    declaredVersion: stdioPdf17BaseSnapshotGuardString(row["declaredVersion"], `${at}.declaredVersion`),
    pages: stdioPdf17BaseSnapshotGuardArray(row["pages"], `${at}.pages`).map((item, index) => parsePdfPage(item, `${at}.pages[${index}]`)),
    info: parsePdfInfo(row["info"], `${at}.info`),
    objects: stdioPdf17BaseSnapshotGuardArray(row["objects"], `${at}.objects`).map((item, index) => parsePdfIndirectObject(item, `${at}.objects[${index}]`)),
    trailer: stdioPdf17BaseSnapshotGuardArray(row["trailer"], `${at}.trailer`).map((item, index) => parsePdfDictEntry(item, `${at}.trailer[${index}]`)),
  };
}

export function parseObjRef(value: unknown, at = "$"): ObjRef {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    num: stdioPdf17BaseSnapshotGuardInteger(row["num"], `${at}.num`, {"minimum": 0}),
    gen: stdioPdf17BaseSnapshotGuardInteger(row["gen"], `${at}.gen`, {"minimum": 0}),
  };
}

export function parsePdfDictEntry(value: unknown, at = "$"): PdfDictEntry {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    key: stdioPdf17BaseSnapshotGuardString(row["key"], `${at}.key`),
    value: parsePdfObject(row["value"], `${at}.value`),
  };
}

export function parsePdfDecimal(value: unknown, at = "$"): PdfDecimal {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    negative: stdioPdf17BaseSnapshotGuardBoolean(row["negative"], `${at}.negative`),
    coefficient: stdioPdf17BaseSnapshotGuardString(row["coefficient"], `${at}.coefficient`, {"pattern": "^[0-9]+$"}),
    scale: stdioPdf17BaseSnapshotGuardInteger(row["scale"], `${at}.scale`, {"minimum": 0}),
  };
}

export function parsePdfPredictor(value: unknown, at = "$"): PdfPredictor {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    predictor: stdioPdf17BaseSnapshotGuardInteger(row["predictor"], `${at}.predictor`),
    colors: stdioPdf17BaseSnapshotGuardInteger(row["colors"], `${at}.colors`),
    bitsPerComponent: stdioPdf17BaseSnapshotGuardInteger(row["bitsPerComponent"], `${at}.bitsPerComponent`),
    columns: stdioPdf17BaseSnapshotGuardInteger(row["columns"], `${at}.columns`),
  };
}

export function parsePdfStreamFilter(value: unknown, at = "$"): PdfStreamFilter {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    kind: stdioPdf17BaseSnapshotGuardMember(row["kind"], `${at}.kind`, ["flate", "asciiHex", "ascii85", "runLength"] as const),
    predictor: row["predictor"] === undefined ? undefined : parsePdfPredictor(row["predictor"], `${at}.predictor`),
  };
}

export function parsePdfIndirectObject(value: unknown, at = "$"): PdfIndirectObject {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    id: parseObjRef(row["id"], `${at}.id`),
    value: parsePdfObject(row["value"], `${at}.value`),
  };
}

export function parsePdfPage(value: unknown, at = "$"): PdfPage {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    mediaBox: stdioPdf17BaseSnapshotGuardArray(row["mediaBox"], `${at}.mediaBox`, {"minItems": 4, "maxItems": 4}).map((item, index) => stdioPdf17BaseSnapshotGuardNumber(item, `${at}.mediaBox[${index}]`)),
    cropBox: row["cropBox"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardArray(row["cropBox"], `${at}.cropBox`, {"minItems": 4, "maxItems": 4}).map((item, index) => stdioPdf17BaseSnapshotGuardNumber(item, `${at}.cropBox[${index}]`)),
    rotate: stdioPdf17BaseSnapshotGuardInteger(row["rotate"], `${at}.rotate`),
    text: stdioPdf17BaseSnapshotGuardString(row["text"], `${at}.text`),
  };
}

export function parsePdfInfo(value: unknown, at = "$"): PdfInfo {
  const row = stdioPdf17BaseSnapshotGuardObject(value, at);
  return {
    title: row["title"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardString(row["title"], `${at}.title`),
    author: row["author"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardString(row["author"], `${at}.author`),
    subject: row["subject"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardString(row["subject"], `${at}.subject`),
    keywords: row["keywords"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardString(row["keywords"], `${at}.keywords`),
    creator: row["creator"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardString(row["creator"], `${at}.creator`),
    producer: row["producer"] === undefined ? undefined : stdioPdf17BaseSnapshotGuardString(row["producer"], `${at}.producer`),
  };
}
