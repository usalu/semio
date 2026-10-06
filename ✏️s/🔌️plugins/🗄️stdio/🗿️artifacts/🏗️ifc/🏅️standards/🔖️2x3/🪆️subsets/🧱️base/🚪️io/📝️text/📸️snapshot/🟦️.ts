/** 📝️ Text representation for `stdio.ifc.2x3` (snapshot). */
export type Ifc2x3SnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc2x3BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc2x3BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioIfc2x3BaseSnapshotTextGuardRefusal(at, why);
};

type stdioIfc2x3BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc2x3BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc2x3BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc2x3BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc2x3BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioIfc2x3BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc2x3BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc2x3BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc2x3BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc2x3BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc2x3BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc2x3BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc2x3BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc2x3BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc2x3BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc2x3BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioIfc2x3BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc2x3BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc2x3BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc2x3BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc2x3BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioIfc2x3BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc2x3BaseSnapshotTextGuardNumber(value, at, bounds) : stdioIfc2x3BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioIfc2x3BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc2x3BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc2x3BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc2x3BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfc2x3SnapshotText(value: unknown, at = "$"): Ifc2x3SnapshotText {
  return stdioIfc2x3BaseSnapshotTextGuardObject(value, `${at}`);
}
