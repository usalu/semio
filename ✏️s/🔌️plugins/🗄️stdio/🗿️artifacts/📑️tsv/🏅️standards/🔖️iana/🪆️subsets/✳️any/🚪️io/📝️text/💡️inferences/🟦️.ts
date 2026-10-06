/** 📝️ Text representation for `stdio.tsv.inference`. */
export type TsvInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTsvIanaAnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTsvIanaAnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioTsvIanaAnyInferenceTextGuardRefusal(at, why);
};

type stdioTsvIanaAnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTsvIanaAnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTsvIanaAnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTsvIanaAnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTsvIanaAnyInferenceTextGuardReject(at, "value is not an object");
export const stdioTsvIanaAnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTsvIanaAnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTsvIanaAnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTsvIanaAnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTsvIanaAnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTsvIanaAnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTsvIanaAnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTsvIanaAnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTsvIanaAnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTsvIanaAnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTsvIanaAnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioTsvIanaAnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTsvIanaAnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTsvIanaAnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTsvIanaAnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTsvIanaAnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioTsvIanaAnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTsvIanaAnyInferenceTextGuardNumber(value, at, bounds) : stdioTsvIanaAnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioTsvIanaAnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTsvIanaAnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTsvIanaAnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTsvIanaAnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTsvInferenceText(value: unknown, at = "$"): TsvInferenceText {
  return stdioTsvIanaAnyInferenceTextGuardObject(value, `${at}`);
}
