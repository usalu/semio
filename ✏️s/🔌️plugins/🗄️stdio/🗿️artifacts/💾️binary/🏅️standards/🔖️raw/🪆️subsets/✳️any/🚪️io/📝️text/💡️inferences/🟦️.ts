/** 📝️ Text representation for `s.stdio.binary.inference`. */
export type BinaryInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBinaryRawAnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBinaryRawAnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioBinaryRawAnyInferenceTextGuardRefusal(at, why);
};

type stdioBinaryRawAnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBinaryRawAnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBinaryRawAnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBinaryRawAnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBinaryRawAnyInferenceTextGuardReject(at, "value is not an object");
export const stdioBinaryRawAnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBinaryRawAnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBinaryRawAnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBinaryRawAnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBinaryRawAnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBinaryRawAnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBinaryRawAnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBinaryRawAnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBinaryRawAnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBinaryRawAnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBinaryRawAnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioBinaryRawAnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBinaryRawAnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBinaryRawAnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBinaryRawAnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBinaryRawAnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioBinaryRawAnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBinaryRawAnyInferenceTextGuardNumber(value, at, bounds) : stdioBinaryRawAnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioBinaryRawAnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBinaryRawAnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBinaryRawAnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBinaryRawAnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBinaryInferenceText(value: unknown, at = "$"): BinaryInferenceText {
  return stdioBinaryRawAnyInferenceTextGuardObject(value, `${at}`);
}
