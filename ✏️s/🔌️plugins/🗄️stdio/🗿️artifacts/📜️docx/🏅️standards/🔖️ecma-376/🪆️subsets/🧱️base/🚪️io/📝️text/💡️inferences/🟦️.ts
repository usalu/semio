/** 📝️ Text representation for `stdio.docx.inference`. */
export type DocxInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDocxEcma376BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDocxEcma376BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioDocxEcma376BaseInferenceTextGuardRefusal(at, why);
};

type stdioDocxEcma376BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDocxEcma376BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDocxEcma376BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDocxEcma376BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDocxEcma376BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioDocxEcma376BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDocxEcma376BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDocxEcma376BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDocxEcma376BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDocxEcma376BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDocxEcma376BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDocxEcma376BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDocxEcma376BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDocxEcma376BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDocxEcma376BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDocxEcma376BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioDocxEcma376BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDocxEcma376BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDocxEcma376BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDocxEcma376BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDocxEcma376BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioDocxEcma376BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDocxEcma376BaseInferenceTextGuardNumber(value, at, bounds) : stdioDocxEcma376BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioDocxEcma376BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDocxEcma376BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDocxEcma376BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDocxEcma376BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDocxInferenceText(value: unknown, at = "$"): DocxInferenceText {
  return stdioDocxEcma376BaseInferenceTextGuardObject(value, `${at}`);
}
