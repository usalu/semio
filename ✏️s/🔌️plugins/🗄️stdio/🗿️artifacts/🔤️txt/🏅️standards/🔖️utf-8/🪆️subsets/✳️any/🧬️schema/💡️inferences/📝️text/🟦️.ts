/** 📝️ Text representation for `stdio.txt.inference`. */
export type TxtInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTxtUtf8AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTxtUtf8AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioTxtUtf8AnyInferenceTextGuardRefusal(at, why);
};

type stdioTxtUtf8AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTxtUtf8AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTxtUtf8AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTxtUtf8AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTxtUtf8AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioTxtUtf8AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTxtUtf8AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTxtUtf8AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTxtUtf8AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTxtUtf8AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTxtUtf8AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTxtUtf8AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTxtUtf8AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTxtUtf8AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTxtUtf8AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTxtUtf8AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioTxtUtf8AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTxtUtf8AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTxtUtf8AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTxtUtf8AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTxtUtf8AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioTxtUtf8AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTxtUtf8AnyInferenceTextGuardNumber(value, at, bounds) : stdioTxtUtf8AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioTxtUtf8AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTxtUtf8AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTxtUtf8AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTxtUtf8AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTxtInferenceText(value: unknown, at = "$"): TxtInferenceText {
  return stdioTxtUtf8AnyInferenceTextGuardObject(value, `${at}`);
}
