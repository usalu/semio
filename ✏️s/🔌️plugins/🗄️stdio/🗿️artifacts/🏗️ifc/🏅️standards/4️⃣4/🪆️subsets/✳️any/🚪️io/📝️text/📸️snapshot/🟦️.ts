/** 📝️ Text representation for `stdio.ifc` (snapshot). */
export type IfcSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioIfc4AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioIfc4AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioIfc4AnySnapshotTextGuardRefusal(at, why);
};

type stdioIfc4AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioIfc4AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioIfc4AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioIfc4AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioIfc4AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioIfc4AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioIfc4AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioIfc4AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioIfc4AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioIfc4AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioIfc4AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioIfc4AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioIfc4AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioIfc4AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioIfc4AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioIfc4AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioIfc4AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioIfc4AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioIfc4AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioIfc4AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioIfc4AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioIfc4AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioIfc4AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioIfc4AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioIfc4AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioIfc4AnySnapshotTextGuardNumber(value, at, bounds) : stdioIfc4AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioIfc4AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioIfc4AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioIfc4AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioIfc4AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseIfcSnapshotText(value: unknown, at = "$"): IfcSnapshotText {
  return stdioIfc4AnySnapshotTextGuardObject(value, `${at}`);
}
