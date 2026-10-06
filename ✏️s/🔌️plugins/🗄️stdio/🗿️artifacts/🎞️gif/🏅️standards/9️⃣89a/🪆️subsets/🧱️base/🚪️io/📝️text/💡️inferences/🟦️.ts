/** 📝️ Text representation for `stdio.gif.89a`. */
export type Gif89aInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif89aBaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif89aBaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioGif89aBaseInferenceTextGuardRefusal(at, why);
};

type stdioGif89aBaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif89aBaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif89aBaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif89aBaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif89aBaseInferenceTextGuardReject(at, "value is not an object");
export const stdioGif89aBaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif89aBaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif89aBaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif89aBaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif89aBaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif89aBaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif89aBaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif89aBaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif89aBaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif89aBaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif89aBaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioGif89aBaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif89aBaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif89aBaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif89aBaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif89aBaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioGif89aBaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif89aBaseInferenceTextGuardNumber(value, at, bounds) : stdioGif89aBaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioGif89aBaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif89aBaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif89aBaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif89aBaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGif89aInferenceText(value: unknown, at = "$"): Gif89aInferenceText {
  return stdioGif89aBaseInferenceTextGuardObject(value, `${at}`);
}
