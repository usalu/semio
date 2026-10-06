/** 📝️ Text representation for `stdio.obj` (snapshot). */
export type ObjSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioObj30GeometrySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioObj30GeometrySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioObj30GeometrySnapshotTextGuardRefusal(at, why);
};

type stdioObj30GeometrySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioObj30GeometrySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioObj30GeometrySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioObj30GeometrySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioObj30GeometrySnapshotTextGuardReject(at, "value is not an object");
export const stdioObj30GeometrySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioObj30GeometrySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioObj30GeometrySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioObj30GeometrySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioObj30GeometrySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioObj30GeometrySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioObj30GeometrySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioObj30GeometrySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioObj30GeometrySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioObj30GeometrySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioObj30GeometrySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioObj30GeometrySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioObj30GeometrySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioObj30GeometrySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioObj30GeometrySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioObj30GeometrySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioObj30GeometrySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioObj30GeometrySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioObj30GeometrySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioObj30GeometrySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioObj30GeometrySnapshotTextGuardNumber(value, at, bounds) : stdioObj30GeometrySnapshotTextGuardReject(at, "value is not an integer");
export const stdioObj30GeometrySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioObj30GeometrySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioObj30GeometrySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioObj30GeometrySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseObjSnapshotText(value: unknown, at = "$"): ObjSnapshotText {
  return stdioObj30GeometrySnapshotTextGuardObject(value, `${at}`);
}
