/** 📝️ Text representation for `s.stdio.semio.document.inference`. */
export type SemioDocumentInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1DocumentInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1DocumentInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1DocumentInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1DocumentInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1DocumentInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1DocumentInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1DocumentInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1DocumentInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1DocumentInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1DocumentInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1DocumentInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1DocumentInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1DocumentInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1DocumentInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1DocumentInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1DocumentInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1DocumentInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1DocumentInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1DocumentInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1DocumentInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1DocumentInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1DocumentInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1DocumentInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1DocumentInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1DocumentInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1DocumentInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1DocumentInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1DocumentInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1DocumentInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1DocumentInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1DocumentInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioDocumentInferenceText(value: unknown, at = "$"): SemioDocumentInferenceText {
  return stdioSemioV1DocumentInferenceTextGuardObject(value, `${at}`);
}
