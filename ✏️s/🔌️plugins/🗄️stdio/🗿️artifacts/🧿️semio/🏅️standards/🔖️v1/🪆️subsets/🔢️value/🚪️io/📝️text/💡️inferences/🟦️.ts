/** 📝️ Text representation for `s.stdio.semio.value.inference`. */
export type SemioValueInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSemioV1ValueInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSemioV1ValueInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSemioV1ValueInferenceTextGuardRefusal(at, why);
};

type stdioSemioV1ValueInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSemioV1ValueInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSemioV1ValueInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSemioV1ValueInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSemioV1ValueInferenceTextGuardReject(at, "value is not an object");
export const stdioSemioV1ValueInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSemioV1ValueInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSemioV1ValueInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSemioV1ValueInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSemioV1ValueInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSemioV1ValueInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSemioV1ValueInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSemioV1ValueInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSemioV1ValueInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSemioV1ValueInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSemioV1ValueInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSemioV1ValueInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSemioV1ValueInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSemioV1ValueInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSemioV1ValueInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSemioV1ValueInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSemioV1ValueInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSemioV1ValueInferenceTextGuardNumber(value, at, bounds) : stdioSemioV1ValueInferenceTextGuardReject(at, "value is not an integer");
export const stdioSemioV1ValueInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSemioV1ValueInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSemioV1ValueInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSemioV1ValueInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSemioValueInferenceText(value: unknown, at = "$"): SemioValueInferenceText {
  return stdioSemioV1ValueInferenceTextGuardObject(value, `${at}`);
}
