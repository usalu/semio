/** 📝️ Text representation for `stdio.txt` (diff). */
export type TxtDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTxtUtf8AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTxtUtf8AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioTxtUtf8AnyDiffTextGuardRefusal(at, why);
};

type stdioTxtUtf8AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTxtUtf8AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTxtUtf8AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTxtUtf8AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTxtUtf8AnyDiffTextGuardReject(at, "value is not an object");
export const stdioTxtUtf8AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioTxtUtf8AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTxtUtf8AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTxtUtf8AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTxtUtf8AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTxtUtf8AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioTxtUtf8AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTxtUtf8AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTxtUtf8AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTxtUtf8AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTxtUtf8AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTxtUtf8AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTxtUtf8AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioTxtUtf8AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioTxtUtf8AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTxtUtf8AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTxtUtf8AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTxtUtf8AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTxtUtf8AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioTxtUtf8AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTxtUtf8AnyDiffTextGuardNumber(value, at, bounds) : stdioTxtUtf8AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioTxtUtf8AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTxtUtf8AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTxtUtf8AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTxtUtf8AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTxtDiffText(value: unknown, at = "$"): TxtDiffText {
  return stdioTxtUtf8AnyDiffTextGuardObject(value, `${at}`);
}
