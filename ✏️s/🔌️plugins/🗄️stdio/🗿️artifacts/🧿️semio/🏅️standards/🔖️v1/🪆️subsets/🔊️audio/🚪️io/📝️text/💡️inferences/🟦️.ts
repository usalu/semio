/** 📝️ Text representation for `s.stdio.semio.audio.inference`. */
export type SemioAudioInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AudioInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AudioInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AudioInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1AudioInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AudioInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AudioInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AudioInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AudioInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1AudioInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AudioInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AudioInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AudioInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AudioInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AudioInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AudioInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AudioInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AudioInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AudioInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AudioInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AudioInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AudioInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AudioInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AudioInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AudioInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AudioInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AudioInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1AudioInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1AudioInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AudioInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AudioInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AudioInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAudioInferenceText(value: unknown, at = "$"): SemioAudioInferenceText {
  return stdioSemioV1AudioInferenceTextGuardObject(value, `${at}`);
}
