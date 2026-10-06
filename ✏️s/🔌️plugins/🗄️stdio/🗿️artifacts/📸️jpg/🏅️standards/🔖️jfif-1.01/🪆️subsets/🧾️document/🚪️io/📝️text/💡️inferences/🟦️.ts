/** 📝️ Text representation for `stdio.jpg`. */
export type JpgInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJpgJfif101DocumentInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJpgJfif101DocumentInferenceTextGuardReject = (at: string, why: string): never => {
  throw new stdioJpgJfif101DocumentInferenceTextGuardRefusal(at, why);
};

type stdioJpgJfif101DocumentInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJpgJfif101DocumentInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJpgJfif101DocumentInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJpgJfif101DocumentInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJpgJfif101DocumentInferenceTextGuardReject(at, "value is not an object");
export const stdioJpgJfif101DocumentInferenceTextGuardArray = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJpgJfif101DocumentInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJpgJfif101DocumentInferenceTextGuardString = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJpgJfif101DocumentInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJpgJfif101DocumentInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJpgJfif101DocumentInferenceTextGuardReject(at, "value is not a boolean"));
export const stdioJpgJfif101DocumentInferenceTextGuardNumber = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJpgJfif101DocumentInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJpgJfif101DocumentInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJpgJfif101DocumentInferenceTextGuardInteger = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJpgJfif101DocumentInferenceTextGuardNumber(value, at, bounds) : stdioJpgJfif101DocumentInferenceTextGuardReject(at, "value is not an integer");
export const stdioJpgJfif101DocumentInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJpgJfif101DocumentInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJpgJfif101DocumentInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJpgJfif101DocumentInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJpgInferenceText(value: unknown, at = "$"): JpgInferenceText {
  return stdioJpgJfif101DocumentInferenceTextGuardObject(value, `${at}`);
}
