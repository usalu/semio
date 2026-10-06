/** 📝️ Text representation for `stdio.json` (diff). */
export type JsonDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJsonRfc8259BaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJsonRfc8259BaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioJsonRfc8259BaseDiffTextGuardRefusal(at, why);
};

type stdioJsonRfc8259BaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJsonRfc8259BaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJsonRfc8259BaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJsonRfc8259BaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJsonRfc8259BaseDiffTextGuardReject(at, "value is not an object");
export const stdioJsonRfc8259BaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJsonRfc8259BaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJsonRfc8259BaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJsonRfc8259BaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJsonRfc8259BaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJsonRfc8259BaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJsonRfc8259BaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJsonRfc8259BaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJsonRfc8259BaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJsonRfc8259BaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJsonRfc8259BaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioJsonRfc8259BaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJsonRfc8259BaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJsonRfc8259BaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJsonRfc8259BaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJsonRfc8259BaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioJsonRfc8259BaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJsonRfc8259BaseDiffTextGuardNumber(value, at, bounds) : stdioJsonRfc8259BaseDiffTextGuardReject(at, "value is not an integer");
export const stdioJsonRfc8259BaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJsonRfc8259BaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJsonRfc8259BaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJsonRfc8259BaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJsonDiffText(value: unknown, at = "$"): JsonDiffText {
  return stdioJsonRfc8259BaseDiffTextGuardObject(value, `${at}`);
}
