/** 📝️ Text representation for `s.stdio.mp4.inference`. */
export type Mp4InferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMp4IsobmffAnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMp4IsobmffAnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioMp4IsobmffAnyInferenceTextGuardRefusal(at, why);
};

type stdioMp4IsobmffAnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMp4IsobmffAnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMp4IsobmffAnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMp4IsobmffAnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMp4IsobmffAnyInferenceTextGuardReject(at, "value is not an object");
export const stdioMp4IsobmffAnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMp4IsobmffAnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMp4IsobmffAnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMp4IsobmffAnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMp4IsobmffAnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMp4IsobmffAnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioMp4IsobmffAnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMp4IsobmffAnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMp4IsobmffAnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMp4IsobmffAnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioMp4IsobmffAnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMp4IsobmffAnyInferenceTextGuardNumber(value, at, bounds) : stdioMp4IsobmffAnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioMp4IsobmffAnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMp4IsobmffAnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMp4IsobmffAnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMp4IsobmffAnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMp4InferenceText(value: unknown, at = "$"): Mp4InferenceText {
  return stdioMp4IsobmffAnyInferenceTextGuardObject(value, `${at}`);
}
