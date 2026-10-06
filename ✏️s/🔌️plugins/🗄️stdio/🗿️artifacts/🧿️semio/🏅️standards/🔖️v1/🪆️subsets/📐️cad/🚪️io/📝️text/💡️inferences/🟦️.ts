/** 📝️ Text representation for `s.stdio.semio.cad.inference`. */
export type SemioCadInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1CadInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1CadInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1CadInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1CadInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1CadInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1CadInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1CadInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1CadInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1CadInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1CadInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1CadInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1CadInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1CadInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1CadInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1CadInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1CadInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1CadInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1CadInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1CadInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1CadInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1CadInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1CadInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1CadInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1CadInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1CadInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1CadInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1CadInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1CadInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1CadInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1CadInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1CadInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioCadInferenceText(value: unknown, at = "$"): SemioCadInferenceText {
  return stdioSemioV1CadInferenceTextGuardObject(value, `${at}`);
}
