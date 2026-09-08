/** 📝️ Text representation for `s.stdio.semio.image.inference`. */
export type SemioImageInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ImageInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ImageInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ImageInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1ImageInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ImageInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ImageInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ImageInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ImageInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1ImageInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ImageInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ImageInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ImageInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ImageInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ImageInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ImageInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ImageInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ImageInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ImageInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ImageInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ImageInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ImageInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ImageInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ImageInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ImageInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ImageInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ImageInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ImageInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ImageInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ImageInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1ImageInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ImageInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ImageInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ImageInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ImageInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioImageInferenceText(value: unknown, at = "$"): SemioImageInferenceText {
  return stdioSemioV1ImageInferenceTextGuardObject(value, `${at}`);
}
