/** 📝️ Text representation for `stdio.json.inference`. */
export type JsonInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJsonRfc8259BaseInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJsonRfc8259BaseInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioJsonRfc8259BaseInferenceTextGuardRefusal(at, why);
};

type stdioJsonRfc8259BaseInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJsonRfc8259BaseInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJsonRfc8259BaseInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJsonRfc8259BaseInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJsonRfc8259BaseInferenceTextGuardReject(at, "value is not an object");
export const stdioJsonRfc8259BaseInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJsonRfc8259BaseInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJsonRfc8259BaseInferenceTextGuardString = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJsonRfc8259BaseInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJsonRfc8259BaseInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJsonRfc8259BaseInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioJsonRfc8259BaseInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJsonRfc8259BaseInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJsonRfc8259BaseInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJsonRfc8259BaseInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJsonRfc8259BaseInferenceTextGuardNumber(value, at, bounds) : stdioJsonRfc8259BaseInferenceTextGuardReject(at, "value is not an integer");
export const stdioJsonRfc8259BaseInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJsonRfc8259BaseInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJsonRfc8259BaseInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJsonRfc8259BaseInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJsonInferenceText(value: unknown, at = "$"): JsonInferenceText {
  return stdioJsonRfc8259BaseInferenceTextGuardObject(value, `${at}`);
}
