/** 📝️ Text representation for `stdio.html.inference`. */
export type HtmlInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioHtml5AnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioHtml5AnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioHtml5AnyInferenceTextGuardRefusal(at, why);
};

type stdioHtml5AnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioHtml5AnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioHtml5AnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioHtml5AnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioHtml5AnyInferenceTextGuardReject(at, "value is not an object");
export const stdioHtml5AnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioHtml5AnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioHtml5AnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioHtml5AnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioHtml5AnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioHtml5AnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioHtml5AnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioHtml5AnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioHtml5AnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioHtml5AnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioHtml5AnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioHtml5AnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioHtml5AnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioHtml5AnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioHtml5AnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioHtml5AnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioHtml5AnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioHtml5AnyInferenceTextGuardNumber(value, at, bounds) : stdioHtml5AnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioHtml5AnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioHtml5AnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioHtml5AnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioHtml5AnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseHtmlInferenceText(value: unknown, at = "$"): HtmlInferenceText {
  return stdioHtml5AnyInferenceTextGuardObject(value, `${at}`);
}
