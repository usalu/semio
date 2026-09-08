/** 📝️ Text representation for `s.stdio.deflate.inference`. */
export type DeflateInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDeflateRfc1950AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDeflateRfc1950AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioDeflateRfc1950AnyInferenceTextGuardRefusal(at, why);
};

type stdioDeflateRfc1950AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDeflateRfc1950AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDeflateRfc1950AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDeflateRfc1950AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDeflateRfc1950AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioDeflateRfc1950AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDeflateRfc1950AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDeflateRfc1950AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDeflateRfc1950AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDeflateRfc1950AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDeflateRfc1950AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioDeflateRfc1950AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDeflateRfc1950AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDeflateRfc1950AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDeflateRfc1950AnyInferenceTextGuardNumber(value, at, bounds) : stdioDeflateRfc1950AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioDeflateRfc1950AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDeflateRfc1950AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDeflateRfc1950AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDeflateInferenceText(value: unknown, at = "$"): DeflateInferenceText {
  return stdioDeflateRfc1950AnyInferenceTextGuardObject(value, `${at}`);
}
