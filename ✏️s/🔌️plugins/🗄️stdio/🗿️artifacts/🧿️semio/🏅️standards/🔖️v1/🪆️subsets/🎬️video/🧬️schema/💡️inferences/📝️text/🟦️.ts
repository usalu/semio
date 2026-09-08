/** 📝️ Text representation for `s.stdio.semio.video.inference`. */
export type SemioVideoInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1VideoInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1VideoInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1VideoInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1VideoInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1VideoInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1VideoInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1VideoInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1VideoInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1VideoInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1VideoInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1VideoInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1VideoInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1VideoInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1VideoInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1VideoInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1VideoInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1VideoInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1VideoInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1VideoInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1VideoInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1VideoInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1VideoInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1VideoInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1VideoInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1VideoInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1VideoInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1VideoInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1VideoInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1VideoInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1VideoInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1VideoInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioVideoInferenceText(value: unknown, at = "$"): SemioVideoInferenceText {
  return stdioSemioV1VideoInferenceTextGuardObject(value, `${at}`);
}
