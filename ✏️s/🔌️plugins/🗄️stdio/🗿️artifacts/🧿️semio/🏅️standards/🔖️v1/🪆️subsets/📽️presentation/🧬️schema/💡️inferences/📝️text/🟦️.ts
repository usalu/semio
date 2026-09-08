/** 📝️ Text representation for `s.stdio.semio.presentation.inference`. */
export type SemioPresentationInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1PresentationInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1PresentationInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1PresentationInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1PresentationInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1PresentationInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1PresentationInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1PresentationInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1PresentationInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1PresentationInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1PresentationInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1PresentationInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1PresentationInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1PresentationInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1PresentationInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1PresentationInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1PresentationInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1PresentationInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1PresentationInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1PresentationInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1PresentationInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1PresentationInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1PresentationInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1PresentationInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1PresentationInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1PresentationInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1PresentationInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1PresentationInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1PresentationInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1PresentationInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1PresentationInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1PresentationInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioPresentationInferenceText(value: unknown, at = "$"): SemioPresentationInferenceText {
  return stdioSemioV1PresentationInferenceTextGuardObject(value, `${at}`);
}
