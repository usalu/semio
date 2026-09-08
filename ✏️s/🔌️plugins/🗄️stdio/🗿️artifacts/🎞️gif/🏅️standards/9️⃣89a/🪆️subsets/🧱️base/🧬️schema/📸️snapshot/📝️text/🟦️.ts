/** 📝️ Text representation for `stdio.gif` (snapshot). */
export type GifSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif89aBaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif89aBaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioGif89aBaseSnapshotTextGuardRefusal(at, why);
};

type stdioGif89aBaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif89aBaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif89aBaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif89aBaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif89aBaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioGif89aBaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioGif89aBaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif89aBaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif89aBaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif89aBaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif89aBaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioGif89aBaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif89aBaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif89aBaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif89aBaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif89aBaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif89aBaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif89aBaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioGif89aBaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioGif89aBaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif89aBaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif89aBaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif89aBaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif89aBaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioGif89aBaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif89aBaseSnapshotTextGuardNumber(value, at, bounds) : stdioGif89aBaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioGif89aBaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif89aBaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif89aBaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif89aBaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifSnapshotText(value: unknown, at = "$"): GifSnapshotText {
  return stdioGif89aBaseSnapshotTextGuardObject(value, `${at}`);
}
