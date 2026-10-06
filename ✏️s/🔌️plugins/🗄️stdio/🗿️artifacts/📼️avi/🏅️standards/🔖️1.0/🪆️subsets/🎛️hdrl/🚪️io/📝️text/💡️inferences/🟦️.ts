/** 📝️ Text representation for `s.stdio.avi.inference`. */
export type AviInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioAvi10HdrlInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioAvi10HdrlInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioAvi10HdrlInferenceTextGuardRefusal(at, why);
};

type stdioAvi10HdrlInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioAvi10HdrlInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioAvi10HdrlInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioAvi10HdrlInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioAvi10HdrlInferenceTextGuardReject(at, "value is not an object");
export const stdioAvi10HdrlInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioAvi10HdrlInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioAvi10HdrlInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioAvi10HdrlInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioAvi10HdrlInferenceTextGuardString = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioAvi10HdrlInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioAvi10HdrlInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioAvi10HdrlInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioAvi10HdrlInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioAvi10HdrlInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioAvi10HdrlInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioAvi10HdrlInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioAvi10HdrlInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioAvi10HdrlInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioAvi10HdrlInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioAvi10HdrlInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioAvi10HdrlInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioAvi10HdrlInferenceTextGuardNumber(value, at, bounds) : stdioAvi10HdrlInferenceTextGuardReject(at, "value is not an integer");
export const stdioAvi10HdrlInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioAvi10HdrlInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioAvi10HdrlInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioAvi10HdrlInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseAviInferenceText(value: unknown, at = "$"): AviInferenceText {
  return stdioAvi10HdrlInferenceTextGuardObject(value, `${at}`);
}
