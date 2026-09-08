/** 🧬️ TsvSnapshot schema facet — mirrors 🦀️.rs field-for-field. IANA TSV has no
 * quoting/escaping; `records` is a raw row grid with no header/data structural distinction. */

export type TsvLineEnding = 'lf' | 'crlf';

export interface TsvSnapshot {
  schema: string;
  records: string[][];
  trailingNewline: boolean;
  lineEnding: TsvLineEnding;
}

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnySnapshotGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnySnapshotGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnySnapshotGuardRefusal(at, why);
};

type stdioTsvIanaAnySnapshotGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnySnapshotGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnySnapshotGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnySnapshotGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnySnapshotGuardReject(at, "value is not an object");
export const stdioTsvIanaAnySnapshotGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnySnapshotGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnySnapshotGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnySnapshotGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnySnapshotGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnySnapshotGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnySnapshotGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnySnapshotGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnySnapshotGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnySnapshotGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnySnapshotGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnySnapshotGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnySnapshotGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnySnapshotGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnySnapshotGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnySnapshotGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnySnapshotGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnySnapshotGuardNumber(value, at, bounds) : stdioTsvIanaAnySnapshotGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnySnapshotGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnySnapshotGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnySnapshotGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnySnapshotGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvSnapshot(value: unknown, at = "$"): TsvSnapshot {
  const row = stdioTsvIanaAnySnapshotGuardObject(value, at);
  return {
    schema: stdioTsvIanaAnySnapshotGuardString(row["schema"], `${at}.schema`),
    records: stdioTsvIanaAnySnapshotGuardArray(row["records"], `${at}.records`).map((item, index) => stdioTsvIanaAnySnapshotGuardArray(item, `${at}.records[${index}]`).map((item, index) => stdioTsvIanaAnySnapshotGuardString(item, `${at}.records[${index}][${index}]`))),
    trailingNewline: stdioTsvIanaAnySnapshotGuardBoolean(row["trailingNewline"], `${at}.trailingNewline`),
    lineEnding: stdioTsvIanaAnySnapshotGuardMember(row["lineEnding"], `${at}.lineEnding`, ["lf", "crlf"] as const),
  };
}
