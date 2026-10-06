/** 📝️ Text representation for `stdio.md.inference`. */
export type MdInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioMdCommonmarkAnyInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioMdCommonmarkAnyInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioMdCommonmarkAnyInferenceTextGuardRefusal(at, why);
};

type stdioMdCommonmarkAnyInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioMdCommonmarkAnyInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioMdCommonmarkAnyInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioMdCommonmarkAnyInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioMdCommonmarkAnyInferenceTextGuardReject(at, "value is not an object");
export const stdioMdCommonmarkAnyInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioMdCommonmarkAnyInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioMdCommonmarkAnyInferenceTextGuardString = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioMdCommonmarkAnyInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioMdCommonmarkAnyInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioMdCommonmarkAnyInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioMdCommonmarkAnyInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioMdCommonmarkAnyInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioMdCommonmarkAnyInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioMdCommonmarkAnyInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioMdCommonmarkAnyInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioMdCommonmarkAnyInferenceTextGuardNumber(value, at, bounds) : stdioMdCommonmarkAnyInferenceTextGuardReject(at, "value is not an integer");
export const stdioMdCommonmarkAnyInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioMdCommonmarkAnyInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioMdCommonmarkAnyInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioMdCommonmarkAnyInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseMdInferenceText(value: unknown, at = "$"): MdInferenceText {
  return stdioMdCommonmarkAnyInferenceTextGuardObject(value, `${at}`);
}
