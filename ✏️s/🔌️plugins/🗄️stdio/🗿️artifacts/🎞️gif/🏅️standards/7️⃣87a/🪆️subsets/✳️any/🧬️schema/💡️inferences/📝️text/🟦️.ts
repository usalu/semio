/** 📝️ Text representation for `stdio.gif`. */
export type Gif87aInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif87aAnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif87aAnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioGif87aAnyInferenceTextGuardRefusal(at, why);
};

type stdioGif87aAnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif87aAnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif87aAnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif87aAnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif87aAnyInferenceTextGuardReject(at, "value is not an object");
export const stdioGif87aAnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif87aAnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif87aAnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif87aAnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif87aAnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif87aAnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif87aAnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif87aAnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif87aAnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif87aAnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif87aAnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioGif87aAnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif87aAnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif87aAnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif87aAnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif87aAnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioGif87aAnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif87aAnyInferenceTextGuardNumber(value, at, bounds) : stdioGif87aAnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioGif87aAnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif87aAnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif87aAnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif87aAnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGif87aInferenceText(value: unknown, at = "$"): Gif87aInferenceText {
  return stdioGif87aAnyInferenceTextGuardObject(value, `${at}`);
}
