/** 🧬️ CsvSnapshot schema facet — mirrors 🦀️.rs field-for-field. */

/** 🔤 One RFC 4180 field value plus whether the source quoted it. */
export interface CsvField {
  value: string;
  quoted: boolean;
}

/** 📄 One RFC 4180 record (row) — index-keyed within `CsvSnapshot.records`. */
export interface CsvRecord {
  fields: CsvField[];
}

/** 📸️ Persisted `stdio.csv` snapshot. `records[0]` is the header row when `hasHeader`. */
export interface CsvSnapshot {
  schema: string;
  hasHeader: boolean;
  records: CsvRecord[];
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioCsvRfc4180AnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioCsvRfc4180AnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioCsvRfc4180AnySnapshotGuardRefusal(at, why);
};

type stdioCsvRfc4180AnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioCsvRfc4180AnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioCsvRfc4180AnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioCsvRfc4180AnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioCsvRfc4180AnySnapshotGuardReject(at, "value is not an object");
export const stdioCsvRfc4180AnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioCsvRfc4180AnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioCsvRfc4180AnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioCsvRfc4180AnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioCsvRfc4180AnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioCsvRfc4180AnySnapshotGuardString = (value: unknown, at: string, bounds: stdioCsvRfc4180AnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioCsvRfc4180AnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioCsvRfc4180AnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioCsvRfc4180AnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioCsvRfc4180AnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioCsvRfc4180AnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioCsvRfc4180AnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioCsvRfc4180AnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioCsvRfc4180AnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioCsvRfc4180AnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioCsvRfc4180AnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioCsvRfc4180AnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioCsvRfc4180AnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioCsvRfc4180AnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioCsvRfc4180AnySnapshotGuardNumber(value, at, bounds) : stdioCsvRfc4180AnySnapshotGuardReject(at, "value is not an integer");
export const stdioCsvRfc4180AnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioCsvRfc4180AnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioCsvRfc4180AnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioCsvRfc4180AnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCsvSnapshot(value: unknown, at = "$"): CsvSnapshot {
  const row = stdioCsvRfc4180AnySnapshotGuardObject(value, at);
  return {
    schema: stdioCsvRfc4180AnySnapshotGuardString(row["schema"], `${at}.schema`),
    hasHeader: stdioCsvRfc4180AnySnapshotGuardBoolean(row["hasHeader"], `${at}.hasHeader`),
    records: stdioCsvRfc4180AnySnapshotGuardArray(row["records"], `${at}.records`).map((item, index) => parseCsvRecord(item, `${at}.records[${index}]`)),
  };
}

export function parseCsvField(value: unknown, at = "$"): CsvField {
  const row = stdioCsvRfc4180AnySnapshotGuardObject(value, at);
  return {
    value: stdioCsvRfc4180AnySnapshotGuardString(row["value"], `${at}.value`),
    quoted: stdioCsvRfc4180AnySnapshotGuardBoolean(row["quoted"], `${at}.quoted`),
  };
}

export function parseCsvRecord(value: unknown, at = "$"): CsvRecord {
  const row = stdioCsvRfc4180AnySnapshotGuardObject(value, at);
  return {
    fields: stdioCsvRfc4180AnySnapshotGuardArray(row["fields"], `${at}.fields`).map((item, index) => parseCsvField(item, `${at}.fields[${index}]`)),
  };
}
