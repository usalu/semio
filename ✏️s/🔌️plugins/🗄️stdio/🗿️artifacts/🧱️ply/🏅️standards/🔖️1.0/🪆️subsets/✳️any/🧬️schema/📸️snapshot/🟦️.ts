/** 🧬️ PlySnapshot schema — complete per PLY's generic element/property system. */

export type PlyScalarType = 'char' | 'uChar' | 'short' | 'uShort' | 'int' | 'uInt' | 'float' | 'double';

/** 🧩️ One `property` declaration inside an `element` block. */
export type PlyProperty =
  | { form: 'scalar'; name: string; kind: PlyScalarType }
  | { form: 'list'; name: string; countKind: PlyScalarType; valueKind: PlyScalarType };

/** 🔣️ One typed cell value (adjacently tagged: `kind` + `value`). */
export type PlyValue =
  | { kind: 'char'; value: number }
  | { kind: 'uChar'; value: number }
  | { kind: 'short'; value: number }
  | { kind: 'uShort'; value: number }
  | { kind: 'int'; value: number }
  | { kind: 'uInt'; value: number }
  | { kind: 'float'; value: number }
  | { kind: 'double'; value: number }
  | { kind: 'list'; value: PlyValue[] };

/** 📏 One element instance's data — one `PlyValue` per declared property, same order. */
export interface PlyRow {
  values: PlyValue[];
}

/** 🧱 One `element <name> <count>` block. */
export interface PlyElement {
  name: string;
  count: number;
  properties: PlyProperty[];
  rows: PlyRow[];
}

export type PlyFormat = 'ascii' | 'binaryLittleEndian' | 'binaryBigEndian';

/** 📸️ Persisted `stdio.ply` snapshot. */
export interface PlySnapshot {
  schema: string;
  format: PlyFormat;
  comments: string[];
  elements: PlyElement[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPly10AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPly10AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioPly10AnySnapshotGuardRefusal(at, why);
};

type stdioPly10AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPly10AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPly10AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPly10AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPly10AnySnapshotGuardReject(at, "value is not an object");
export const stdioPly10AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioPly10AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPly10AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPly10AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPly10AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPly10AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioPly10AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPly10AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPly10AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPly10AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPly10AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPly10AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPly10AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioPly10AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioPly10AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPly10AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPly10AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPly10AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPly10AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioPly10AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPly10AnySnapshotGuardNumber(value, at, bounds) : stdioPly10AnySnapshotGuardReject(at, "value is not an integer");
export const stdioPly10AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPly10AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPly10AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPly10AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlySnapshot(value: unknown, at = "$"): PlySnapshot {
  const row = stdioPly10AnySnapshotGuardObject(value, at);
  return {
    schema: stdioPly10AnySnapshotGuardString(row["schema"], `${at}.schema`),
    format: stdioPly10AnySnapshotGuardMember(row["format"], `${at}.format`, ["ascii", "binaryLittleEndian", "binaryBigEndian"] as const),
    comments: stdioPly10AnySnapshotGuardArray(row["comments"], `${at}.comments`).map((item, index) => stdioPly10AnySnapshotGuardString(item, `${at}.comments[${index}]`)),
    elements: stdioPly10AnySnapshotGuardArray(row["elements"], `${at}.elements`).map((item, index) => parsePlyElement(item, `${at}.elements[${index}]`)),
  };
}

export function parsePlyScalarType(value: unknown, at = "$"): PlyScalarType {
  return stdioPly10AnySnapshotGuardMember(value, `${at}`, ["char", "uChar", "short", "uShort", "int", "uInt", "float", "double"] as const);
}

export function parsePlyRow(value: unknown, at = "$"): PlyRow {
  const row = stdioPly10AnySnapshotGuardObject(value, at);
  return {
    values: stdioPly10AnySnapshotGuardArray(row["values"], `${at}.values`).map((item, index) => parsePlyValue(item, `${at}.values[${index}]`)),
  };
}

export function parsePlyElement(value: unknown, at = "$"): PlyElement {
  const row = stdioPly10AnySnapshotGuardObject(value, at);
  return {
    name: stdioPly10AnySnapshotGuardString(row["name"], `${at}.name`),
    count: stdioPly10AnySnapshotGuardInteger(row["count"], `${at}.count`, {"minimum": 0}),
    properties: stdioPly10AnySnapshotGuardArray(row["properties"], `${at}.properties`).map((item, index) => parsePlyProperty(item, `${at}.properties[${index}]`)),
    rows: stdioPly10AnySnapshotGuardArray(row["rows"], `${at}.rows`).map((item, index) => parsePlyRow(item, `${at}.rows[${index}]`)),
  };
}
