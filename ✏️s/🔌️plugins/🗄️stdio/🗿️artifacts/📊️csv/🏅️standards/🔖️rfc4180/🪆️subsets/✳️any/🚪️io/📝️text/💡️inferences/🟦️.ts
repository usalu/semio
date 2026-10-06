/** 📝️ Text representation for `stdio.csv.inference`. */
export type CsvInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioCsvRfc4180AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioCsvRfc4180AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioCsvRfc4180AnyInferenceTextGuardRefusal(at, why);
};

type stdioCsvRfc4180AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioCsvRfc4180AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioCsvRfc4180AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioCsvRfc4180AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioCsvRfc4180AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioCsvRfc4180AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioCsvRfc4180AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioCsvRfc4180AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioCsvRfc4180AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioCsvRfc4180AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioCsvRfc4180AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioCsvRfc4180AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioCsvRfc4180AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioCsvRfc4180AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioCsvRfc4180AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioCsvRfc4180AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioCsvRfc4180AnyInferenceTextGuardNumber(value, at, bounds) : stdioCsvRfc4180AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioCsvRfc4180AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioCsvRfc4180AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioCsvRfc4180AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioCsvRfc4180AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseCsvInferenceText(value: unknown, at = "$"): CsvInferenceText {
  return stdioCsvRfc4180AnyInferenceTextGuardObject(value, `${at}`);
}
