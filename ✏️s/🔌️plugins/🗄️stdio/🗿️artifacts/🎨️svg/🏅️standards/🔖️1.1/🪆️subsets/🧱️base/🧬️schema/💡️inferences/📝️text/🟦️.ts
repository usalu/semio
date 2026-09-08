/** 📝️ Text representation for `stdio.svg`. */
export type SvgInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioSvg11BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioSvg11BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioSvg11BaseInferenceTextGuardRefusal(at, why);
};

type stdioSvg11BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioSvg11BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioSvg11BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioSvg11BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioSvg11BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioSvg11BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioSvg11BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioSvg11BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioSvg11BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioSvg11BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioSvg11BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioSvg11BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioSvg11BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioSvg11BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioSvg11BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioSvg11BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioSvg11BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioSvg11BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioSvg11BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioSvg11BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioSvg11BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioSvg11BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioSvg11BaseInferenceTextGuardNumber(value, at, bounds) : stdioSvg11BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioSvg11BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioSvg11BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioSvg11BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioSvg11BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSvgInferenceText(value: unknown, at = "$"): SvgInferenceText {
  return stdioSvg11BaseInferenceTextGuardObject(value, `${at}`);
}
