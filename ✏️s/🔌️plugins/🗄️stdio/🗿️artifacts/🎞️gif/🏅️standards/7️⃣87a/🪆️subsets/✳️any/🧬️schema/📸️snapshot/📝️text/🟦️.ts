/** 📝️ Text representation for `stdio.gif` (snapshot). */
export type GifSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioGif87aAnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioGif87aAnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioGif87aAnySnapshotTextGuardRefusal(at, why);
};

type stdioGif87aAnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioGif87aAnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioGif87aAnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioGif87aAnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioGif87aAnySnapshotTextGuardReject(at, "value is not an object");
export const stdioGif87aAnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioGif87aAnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioGif87aAnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioGif87aAnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioGif87aAnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioGif87aAnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioGif87aAnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioGif87aAnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioGif87aAnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioGif87aAnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioGif87aAnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioGif87aAnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioGif87aAnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioGif87aAnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioGif87aAnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioGif87aAnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioGif87aAnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioGif87aAnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioGif87aAnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioGif87aAnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioGif87aAnySnapshotTextGuardNumber(value, at, bounds) : stdioGif87aAnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioGif87aAnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioGif87aAnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioGif87aAnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioGif87aAnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseGifSnapshotText(value: unknown, at = "$"): GifSnapshotText {
  return stdioGif87aAnySnapshotTextGuardObject(value, `${at}`);
}
