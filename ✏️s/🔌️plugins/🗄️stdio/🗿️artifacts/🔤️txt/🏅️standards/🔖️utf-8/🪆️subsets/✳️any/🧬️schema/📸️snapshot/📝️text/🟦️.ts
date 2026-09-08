/** 📝️ Text representation for `stdio.txt` (snapshot). */
export type TxtSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTxtUtf8AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTxtUtf8AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioTxtUtf8AnySnapshotTextGuardRefusal(at, why);
};

type stdioTxtUtf8AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTxtUtf8AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTxtUtf8AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTxtUtf8AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTxtUtf8AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioTxtUtf8AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTxtUtf8AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTxtUtf8AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTxtUtf8AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTxtUtf8AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTxtUtf8AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTxtUtf8AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTxtUtf8AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTxtUtf8AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTxtUtf8AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTxtUtf8AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioTxtUtf8AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTxtUtf8AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTxtUtf8AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTxtUtf8AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTxtUtf8AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioTxtUtf8AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTxtUtf8AnySnapshotTextGuardNumber(value, at, bounds) : stdioTxtUtf8AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioTxtUtf8AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTxtUtf8AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTxtUtf8AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTxtUtf8AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTxtSnapshotText(value: unknown, at = "$"): TxtSnapshotText {
  return stdioTxtUtf8AnySnapshotTextGuardObject(value, `${at}`);
}
