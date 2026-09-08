/** 📝️ Text representation for `stdio.deflate` (diff). */
export type DeflateDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDeflateRfc1950AnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDeflateRfc1950AnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioDeflateRfc1950AnyDiffTextGuardRefusal(at, why);
};

type stdioDeflateRfc1950AnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDeflateRfc1950AnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDeflateRfc1950AnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDeflateRfc1950AnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDeflateRfc1950AnyDiffTextGuardReject(at, "value is not an object");
export const stdioDeflateRfc1950AnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDeflateRfc1950AnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDeflateRfc1950AnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDeflateRfc1950AnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDeflateRfc1950AnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDeflateRfc1950AnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioDeflateRfc1950AnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDeflateRfc1950AnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDeflateRfc1950AnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDeflateRfc1950AnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDeflateRfc1950AnyDiffTextGuardNumber(value, at, bounds) : stdioDeflateRfc1950AnyDiffTextGuardReject(at, "value is not an integer");
export const stdioDeflateRfc1950AnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDeflateRfc1950AnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDeflateRfc1950AnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDeflateRfc1950AnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDeflateDiffText(value: unknown, at = "$"): DeflateDiffText {
  return stdioDeflateRfc1950AnyDiffTextGuardObject(value, `${at}`);
}
