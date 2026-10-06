/** 📝️ Text representation for `stdio.deflate` (snapshot). */
export type DeflateSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDeflateRfc1950AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDeflateRfc1950AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioDeflateRfc1950AnySnapshotTextGuardRefusal(at, why);
};

type stdioDeflateRfc1950AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDeflateRfc1950AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDeflateRfc1950AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDeflateRfc1950AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDeflateRfc1950AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioDeflateRfc1950AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDeflateRfc1950AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDeflateRfc1950AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDeflateRfc1950AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDeflateRfc1950AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDeflateRfc1950AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioDeflateRfc1950AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDeflateRfc1950AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDeflateRfc1950AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioDeflateRfc1950AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDeflateRfc1950AnySnapshotTextGuardNumber(value, at, bounds) : stdioDeflateRfc1950AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioDeflateRfc1950AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDeflateRfc1950AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDeflateRfc1950AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDeflateSnapshotText(value: unknown, at = "$"): DeflateSnapshotText {
  return stdioDeflateRfc1950AnySnapshotTextGuardObject(value, `${at}`);
}
