/** 📝️ Text representation for `stdio.binary` (snapshot). */
export type BinarySnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBinaryRawAnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBinaryRawAnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioBinaryRawAnySnapshotTextGuardRefusal(at, why);
};

type stdioBinaryRawAnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBinaryRawAnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBinaryRawAnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBinaryRawAnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBinaryRawAnySnapshotTextGuardReject(at, "value is not an object");
export const stdioBinaryRawAnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBinaryRawAnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBinaryRawAnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBinaryRawAnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBinaryRawAnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBinaryRawAnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBinaryRawAnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBinaryRawAnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBinaryRawAnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBinaryRawAnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBinaryRawAnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioBinaryRawAnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBinaryRawAnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBinaryRawAnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBinaryRawAnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBinaryRawAnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioBinaryRawAnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBinaryRawAnySnapshotTextGuardNumber(value, at, bounds) : stdioBinaryRawAnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioBinaryRawAnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBinaryRawAnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBinaryRawAnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBinaryRawAnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBinarySnapshotText(value: unknown, at = "$"): BinarySnapshotText {
  return stdioBinaryRawAnySnapshotTextGuardObject(value, `${at}`);
}
