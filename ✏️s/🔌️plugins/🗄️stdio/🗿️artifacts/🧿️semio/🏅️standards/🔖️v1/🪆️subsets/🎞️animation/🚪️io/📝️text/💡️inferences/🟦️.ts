/** 📝️ Text representation for `s.stdio.semio.animation.inference`. */
export type SemioAnimationInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1AnimationInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1AnimationInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1AnimationInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1AnimationInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1AnimationInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1AnimationInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1AnimationInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1AnimationInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1AnimationInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1AnimationInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1AnimationInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1AnimationInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1AnimationInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1AnimationInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1AnimationInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1AnimationInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1AnimationInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1AnimationInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1AnimationInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1AnimationInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1AnimationInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1AnimationInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1AnimationInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1AnimationInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1AnimationInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1AnimationInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1AnimationInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1AnimationInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1AnimationInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1AnimationInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1AnimationInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioAnimationInferenceText(value: unknown, at = "$"): SemioAnimationInferenceText {
  return stdioSemioV1AnimationInferenceTextGuardObject(value, `${at}`);
}
