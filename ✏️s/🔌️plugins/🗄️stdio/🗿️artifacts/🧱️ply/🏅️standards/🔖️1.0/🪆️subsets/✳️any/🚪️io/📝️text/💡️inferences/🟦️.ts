/** 📝️ Text representation for `s.stdio.ply.inference`. */
export type PlyInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPly10AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPly10AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioPly10AnyInferenceTextGuardRefusal(at, why);
};

type stdioPly10AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPly10AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPly10AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPly10AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPly10AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioPly10AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioPly10AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPly10AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPly10AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPly10AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPly10AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioPly10AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPly10AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPly10AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPly10AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPly10AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPly10AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPly10AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioPly10AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioPly10AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPly10AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPly10AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPly10AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPly10AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioPly10AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPly10AnyInferenceTextGuardNumber(value, at, bounds) : stdioPly10AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioPly10AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPly10AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPly10AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPly10AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePlyInferenceText(value: unknown, at = "$"): PlyInferenceText {
  return stdioPly10AnyInferenceTextGuardObject(value, `${at}`);
}
