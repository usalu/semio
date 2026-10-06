/** 📝️ Text representation for `stdio.gif` (diff). */
export type GifDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif87aAnyDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif87aAnyDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioGif87aAnyDiffTextGuardRefusal(at, why);
};

type stdioGif87aAnyDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif87aAnyDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif87aAnyDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif87aAnyDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif87aAnyDiffTextGuardReject(at, "value is not an object");
export const stdioGif87aAnyDiffTextGuardArray = (value: unknown, at: string, bounds: stdioGif87aAnyDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif87aAnyDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif87aAnyDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif87aAnyDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif87aAnyDiffTextGuardString = (value: unknown, at: string, bounds: stdioGif87aAnyDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif87aAnyDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif87aAnyDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif87aAnyDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif87aAnyDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif87aAnyDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif87aAnyDiffTextGuardReject(at, "value is not a boolean"));
export const stdioGif87aAnyDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioGif87aAnyDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif87aAnyDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif87aAnyDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif87aAnyDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif87aAnyDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioGif87aAnyDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif87aAnyDiffTextGuardNumber(value, at, bounds) : stdioGif87aAnyDiffTextGuardReject(at, "value is not an integer");
export const stdioGif87aAnyDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif87aAnyDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif87aAnyDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif87aAnyDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifDiffText(value: unknown, at = "$"): GifDiffText {
  return stdioGif87aAnyDiffTextGuardObject(value, `${at}`);
}
