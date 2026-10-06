/** 📝️ Text representation for `stdio.png`. */
export type PngInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPng12AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPng12AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioPng12AnyInferenceTextGuardRefusal(at, why);
};

type stdioPng12AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPng12AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPng12AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPng12AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPng12AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioPng12AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioPng12AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPng12AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPng12AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPng12AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPng12AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioPng12AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPng12AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPng12AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPng12AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPng12AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPng12AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPng12AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioPng12AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioPng12AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPng12AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPng12AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPng12AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPng12AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioPng12AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPng12AnyInferenceTextGuardNumber(value, at, bounds) : stdioPng12AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioPng12AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPng12AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPng12AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPng12AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePngInferenceText(value: unknown, at = "$"): PngInferenceText {
  return stdioPng12AnyInferenceTextGuardObject(value, `${at}`);
}
