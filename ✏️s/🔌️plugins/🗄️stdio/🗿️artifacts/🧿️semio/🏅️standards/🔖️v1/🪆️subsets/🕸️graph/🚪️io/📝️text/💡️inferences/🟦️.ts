/** 📝️ Text representation for `s.stdio.semio.graph.inference`. */
export type SemioGraphInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1GraphInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1GraphInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1GraphInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1GraphInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1GraphInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1GraphInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1GraphInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1GraphInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1GraphInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1GraphInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1GraphInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1GraphInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1GraphInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1GraphInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1GraphInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1GraphInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1GraphInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1GraphInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1GraphInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1GraphInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1GraphInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1GraphInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1GraphInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1GraphInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1GraphInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1GraphInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1GraphInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1GraphInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1GraphInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1GraphInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1GraphInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioGraphInferenceText(value: unknown, at = "$"): SemioGraphInferenceText {
  return stdioSemioV1GraphInferenceTextGuardObject(value, `${at}`);
}
