/** 📝️ Text representation for `s.stdio.dwg.inference` (ac1018). */
export type DwgInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1018AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1018AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1018AnyInferenceTextGuardRefusal(at, why);
};

type stdioDwgAc1018AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1018AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1018AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1018AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1018AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioDwgAc1018AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1018AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1018AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1018AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1018AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1018AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1018AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1018AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1018AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1018AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1018AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1018AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1018AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1018AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1018AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1018AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1018AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1018AnyInferenceTextGuardNumber(value, at, bounds) : stdioDwgAc1018AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioDwgAc1018AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1018AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1018AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1018AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export type DwgAc1018InferenceText = Readonly<Record<string, unknown>>;

export function parseDwgAc1018InferenceText(value: unknown, at = "$"): DwgAc1018InferenceText {
  return stdioDwgAc1018AnyInferenceTextGuardObject(value, `${at}`);
}
