/** 📝️ Text representation for `s.stdio.semio.object.inference`. */
export type SemioObjectInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ObjectInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ObjectInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ObjectInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1ObjectInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ObjectInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ObjectInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ObjectInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ObjectInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1ObjectInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ObjectInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ObjectInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ObjectInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ObjectInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ObjectInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ObjectInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ObjectInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ObjectInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ObjectInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ObjectInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ObjectInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ObjectInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ObjectInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ObjectInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ObjectInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ObjectInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ObjectInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1ObjectInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ObjectInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ObjectInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ObjectInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ObjectInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioObjectInferenceText(value: unknown, at = "$"): SemioObjectInferenceText {
  return stdioSemioV1ObjectInferenceTextGuardObject(value, `${at}`);
}
