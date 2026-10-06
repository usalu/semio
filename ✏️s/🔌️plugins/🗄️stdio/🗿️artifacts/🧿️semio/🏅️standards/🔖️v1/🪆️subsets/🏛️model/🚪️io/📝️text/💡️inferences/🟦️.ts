/** 📝️ Text representation for `s.stdio.semio.model.inference`. */
export type SemioModelInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ModelInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ModelInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ModelInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1ModelInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ModelInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ModelInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ModelInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ModelInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1ModelInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ModelInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ModelInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ModelInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ModelInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ModelInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ModelInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ModelInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ModelInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ModelInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ModelInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ModelInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ModelInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ModelInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ModelInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ModelInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ModelInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ModelInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1ModelInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ModelInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ModelInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ModelInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ModelInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioModelInferenceText(value: unknown, at = "$"): SemioModelInferenceText {
  return stdioSemioV1ModelInferenceTextGuardObject(value, `${at}`);
}
