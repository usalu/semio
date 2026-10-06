/** 📝️ Text representation for `stdio.pdf.inference`. */
export type PdfInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf14BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf14BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioPdf14BaseInferenceTextGuardRefusal(at, why);
};

type stdioPdf14BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf14BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf14BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf14BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf14BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioPdf14BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf14BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf14BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf14BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf14BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf14BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf14BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf14BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf14BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf14BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf14BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioPdf14BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf14BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf14BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf14BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf14BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioPdf14BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf14BaseInferenceTextGuardNumber(value, at, bounds) : stdioPdf14BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioPdf14BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf14BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf14BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf14BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfInferenceText(value: unknown, at = "$"): PdfInferenceText {
  return stdioPdf14BaseInferenceTextGuardObject(value, `${at}`);
}
