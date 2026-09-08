/** 💾️ Binary representation for `stdio.ifc.2x3` (snapshot). */
export type Ifc2x3SnapshotBinary = Uint8Array;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc2x3BaseSnapshotBinaryGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc2x3BaseSnapshotBinaryGuardReject = (at: string, why: string): never => {
  throw new stdioIfc2x3BaseSnapshotBinaryGuardRefusal(at, why);
};

type stdioIfc2x3BaseSnapshotBinaryGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc2x3BaseSnapshotBinaryGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc2x3BaseSnapshotBinaryGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc2x3BaseSnapshotBinaryGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc2x3BaseSnapshotBinaryGuardReject(at, "value is not an object");
export const stdioIfc2x3BaseSnapshotBinaryGuardArray = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotBinaryGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc2x3BaseSnapshotBinaryGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc2x3BaseSnapshotBinaryGuardString = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotBinaryGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc2x3BaseSnapshotBinaryGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc2x3BaseSnapshotBinaryGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc2x3BaseSnapshotBinaryGuardReject(at, "value is not a boolean"));
export const stdioIfc2x3BaseSnapshotBinaryGuardNumber = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotBinaryGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc2x3BaseSnapshotBinaryGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc2x3BaseSnapshotBinaryGuardInteger = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotBinaryGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc2x3BaseSnapshotBinaryGuardNumber(value, at, bounds) : stdioIfc2x3BaseSnapshotBinaryGuardReject(at, "value is not an integer");
export const stdioIfc2x3BaseSnapshotBinaryGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc2x3BaseSnapshotBinaryGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc2x3BaseSnapshotBinaryGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfc2x3SnapshotBinary(value: unknown, at = "$"): Ifc2x3SnapshotBinary {
  return stdioIfc2x3BaseSnapshotBinaryGuardObject(value, `${at}`);
}
