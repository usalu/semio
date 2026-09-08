/** 📝️ Text representation for `stdio.bmp`. */
export type BmpInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBmpV3AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBmpV3AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioBmpV3AnyInferenceTextGuardRefusal(at, why);
};

type stdioBmpV3AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBmpV3AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBmpV3AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBmpV3AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBmpV3AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioBmpV3AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBmpV3AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBmpV3AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBmpV3AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBmpV3AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBmpV3AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBmpV3AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBmpV3AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBmpV3AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBmpV3AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBmpV3AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioBmpV3AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBmpV3AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBmpV3AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBmpV3AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBmpV3AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioBmpV3AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBmpV3AnyInferenceTextGuardNumber(value, at, bounds) : stdioBmpV3AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioBmpV3AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBmpV3AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBmpV3AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBmpV3AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBmpInferenceText(value: unknown, at = "$"): BmpInferenceText {
  return stdioBmpV3AnyInferenceTextGuardObject(value, `${at}`);
}
