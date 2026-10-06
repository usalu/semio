/** 📝️ Text representation for `s.stdio.wav.inference`. */
export type WavInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioWavRiffpcmAnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioWavRiffpcmAnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioWavRiffpcmAnyInferenceTextGuardRefusal(at, why);
};

type stdioWavRiffpcmAnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioWavRiffpcmAnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioWavRiffpcmAnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioWavRiffpcmAnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioWavRiffpcmAnyInferenceTextGuardReject(at, "value is not an object");
export const stdioWavRiffpcmAnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioWavRiffpcmAnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioWavRiffpcmAnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioWavRiffpcmAnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioWavRiffpcmAnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioWavRiffpcmAnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioWavRiffpcmAnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioWavRiffpcmAnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioWavRiffpcmAnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioWavRiffpcmAnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioWavRiffpcmAnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioWavRiffpcmAnyInferenceTextGuardNumber(value, at, bounds) : stdioWavRiffpcmAnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioWavRiffpcmAnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioWavRiffpcmAnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioWavRiffpcmAnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioWavRiffpcmAnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseWavInferenceText(value: unknown, at = "$"): WavInferenceText {
  return stdioWavRiffpcmAnyInferenceTextGuardObject(value, `${at}`);
}
