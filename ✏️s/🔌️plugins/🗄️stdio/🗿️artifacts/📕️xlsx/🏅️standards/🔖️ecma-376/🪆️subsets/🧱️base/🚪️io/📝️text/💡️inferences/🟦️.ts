/** 📝️ Text representation for `stdio.xlsx.inference`. */
export type XlsxInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXlsxEcma376BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXlsxEcma376BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioXlsxEcma376BaseInferenceTextGuardRefusal(at, why);
};

type stdioXlsxEcma376BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXlsxEcma376BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXlsxEcma376BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXlsxEcma376BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXlsxEcma376BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioXlsxEcma376BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXlsxEcma376BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXlsxEcma376BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXlsxEcma376BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXlsxEcma376BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXlsxEcma376BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioXlsxEcma376BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXlsxEcma376BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXlsxEcma376BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXlsxEcma376BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXlsxEcma376BaseInferenceTextGuardNumber(value, at, bounds) : stdioXlsxEcma376BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioXlsxEcma376BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXlsxEcma376BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXlsxEcma376BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXlsxEcma376BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXlsxInferenceText(value: unknown, at = "$"): XlsxInferenceText {
  return stdioXlsxEcma376BaseInferenceTextGuardObject(value, `${at}`);
}
