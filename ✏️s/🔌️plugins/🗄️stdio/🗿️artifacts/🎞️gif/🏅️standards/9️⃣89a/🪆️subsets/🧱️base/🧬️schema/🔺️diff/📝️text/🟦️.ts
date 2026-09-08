/** 📝️ Text representation for `stdio.gif` (diff). */
export type GifDiffText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif89aBaseDiffTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif89aBaseDiffTextGuardReject = (at: string, why: string): never => {
  throw new stdioGif89aBaseDiffTextGuardRefusal(at, why);
};

type stdioGif89aBaseDiffTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif89aBaseDiffTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif89aBaseDiffTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif89aBaseDiffTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif89aBaseDiffTextGuardReject(at, "value is not an object");
export const stdioGif89aBaseDiffTextGuardArray = (value: unknown, at: string, bounds: stdioGif89aBaseDiffTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif89aBaseDiffTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif89aBaseDiffTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif89aBaseDiffTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif89aBaseDiffTextGuardString = (value: unknown, at: string, bounds: stdioGif89aBaseDiffTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif89aBaseDiffTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif89aBaseDiffTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif89aBaseDiffTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif89aBaseDiffTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif89aBaseDiffTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif89aBaseDiffTextGuardReject(at, "value is not a boolean"));
export const stdioGif89aBaseDiffTextGuardNumber = (value: unknown, at: string, bounds: stdioGif89aBaseDiffTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif89aBaseDiffTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif89aBaseDiffTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif89aBaseDiffTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif89aBaseDiffTextGuardInteger = (value: unknown, at: string, bounds: stdioGif89aBaseDiffTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif89aBaseDiffTextGuardNumber(value, at, bounds) : stdioGif89aBaseDiffTextGuardReject(at, "value is not an integer");
export const stdioGif89aBaseDiffTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif89aBaseDiffTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif89aBaseDiffTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif89aBaseDiffTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifDiffText(value: unknown, at = "$"): GifDiffText {
  return stdioGif89aBaseDiffTextGuardObject(value, `${at}`);
}
