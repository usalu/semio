/** 🧬️ TiffSnapshot schema facet — mirrors 🦀️.rs field-for-field. Complete TIFF 6.0
 * semantic model: `byteOrder` + index-keyed `ifds`, each holding tag-id-keyed typed tag/type/
 * value entries — TIFF's own generic model ("unknown tags" are just tags this codec doesn't
 * specially interpret, but whose typed value is still stored losslessly via this same
 * triple), plus decoded `pixels`. */

export type TiffByteOrder = 'littleEndian' | 'bigEndian';

/** 🏷️ TIFF6 §2 Table 2 — the 12 real IFD entry field types. */
export type TiffFieldType =
  | 'byte' | 'ascii' | 'short' | 'long' | 'rational' | 'sByte'
  | 'undefined' | 'sShort' | 'sLong' | 'sRational' | 'float' | 'double';

/** 📦️ Typed union over every TIFF 6.0 field type's decoded value — adjacently tagged
 * (`kind`/`value`), mirroring the Rust enum's `#[serde(tag = "kind", content = "value")]`. */
export type TiffValues =
  | { kind: 'byte'; value: number[] }
  | { kind: 'ascii'; value: string }
  | { kind: 'short'; value: number[] }
  | { kind: 'long'; value: number[] }
  | { kind: 'rational'; value: [number, number][] }
  | { kind: 'sByte'; value: number[] }
  | { kind: 'undefined'; value: number[] }
  | { kind: 'sShort'; value: number[] }
  | { kind: 'sLong'; value: number[] }
  | { kind: 'sRational'; value: [number, number][] }
  | { kind: 'float'; value: number[] }
  | { kind: 'double'; value: number[] };

/** 🏷️ One IFD entry — a weak value (whole-value replaced in diffs: `kind`/`values` move
 * together atomically). */
export interface TiffTag {
  tag: number;
  kind: TiffFieldType;
  values: TiffValues;
}

/** 🗂️ One Image File Directory — tag-id-keyed `entries` (TIFF requires ascending-tag-order). */
export interface TiffIfd {
  entries: TiffTag[];
  /** This directory's own raster as RAW STRIP BYTES; empty for IFD 0 (whose raster is the
   * snapshot's own canonical RGBA `pixels`) and for a metadata-only directory. */
  pixels: number[];
}

/** 📸️ Complete `stdio.tiff` 6.0 semantic snapshot. `schema` is an identity field, never
 * diffed. `pixels` is the decoded raster payload (canonical 8-bit RGBA, decoded from IFD 0
 * only — see the Rust engine's doc for the full completeness accounting). */
export interface TiffSnapshot {
  schema: string;
  byteOrder: TiffByteOrder;
  ifds: TiffIfd[];
  pixels: number[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentSnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentSnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentSnapshotGuardRefusal(at, why);
};

type stdioTiff60DocumentSnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentSnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentSnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentSnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentSnapshotGuardReject(at, "value is not an object");
export const stdioTiff60DocumentSnapshotGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentSnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentSnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentSnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentSnapshotGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentSnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentSnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentSnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentSnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentSnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentSnapshotGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentSnapshotGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentSnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentSnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentSnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentSnapshotGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentSnapshotGuardNumber(value, at, bounds) : stdioTiff60DocumentSnapshotGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentSnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentSnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentSnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentSnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffSnapshot(value: unknown, at = "$"): TiffSnapshot {
  const row = stdioTiff60DocumentSnapshotGuardObject(value, at);
  return {
    schema: stdioTiff60DocumentSnapshotGuardString(row["schema"], `${at}.schema`),
    byteOrder: parseTiffByteOrder(row["byteOrder"], `${at}.byteOrder`),
    ifds: stdioTiff60DocumentSnapshotGuardArray(row["ifds"], `${at}.ifds`).map((item, index) => parseTiffIfd(item, `${at}.ifds[${index}]`)),
    pixels: stdioTiff60DocumentSnapshotGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => stdioTiff60DocumentSnapshotGuardInteger(item, `${at}.pixels[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}

export function parseTiffByteOrder(value: unknown, at = "$"): TiffByteOrder {
  return stdioTiff60DocumentSnapshotGuardMember(value, `${at}`, ["littleEndian", "bigEndian"] as const);
}

export function parseTiffFieldType(value: unknown, at = "$"): TiffFieldType {
  return stdioTiff60DocumentSnapshotGuardMember(value, `${at}`, ["byte", "ascii", "short", "long", "rational", "sByte", "undefined", "sShort", "sLong", "sRational", "float", "double"] as const);
}

export function parseTiffTag(value: unknown, at = "$"): TiffTag {
  const row = stdioTiff60DocumentSnapshotGuardObject(value, at);
  return {
    tag: stdioTiff60DocumentSnapshotGuardInteger(row["tag"], `${at}.tag`, {"minimum": 0, "maximum": 65535}),
    kind: parseTiffFieldType(row["kind"], `${at}.kind`),
    values: parseTiffValues(row["values"], `${at}.values`),
  };
}

export function parseTiffIfd(value: unknown, at = "$"): TiffIfd {
  const row = stdioTiff60DocumentSnapshotGuardObject(value, at);
  return {
    entries: stdioTiff60DocumentSnapshotGuardArray(row["entries"], `${at}.entries`).map((item, index) => parseTiffTag(item, `${at}.entries[${index}]`)),
    pixels: stdioTiff60DocumentSnapshotGuardArray(row["pixels"], `${at}.pixels`).map((item, index) => stdioTiff60DocumentSnapshotGuardInteger(item, `${at}.pixels[${index}]`, {"minimum": 0, "maximum": 255})),
  };
}
