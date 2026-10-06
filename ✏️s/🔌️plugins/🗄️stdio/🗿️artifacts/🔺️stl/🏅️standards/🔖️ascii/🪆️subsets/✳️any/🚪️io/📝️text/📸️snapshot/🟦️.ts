/** 📝️ Text representation for `stdio.stl` (snapshot). */
export type StlSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioStlAsciiAnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioStlAsciiAnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioStlAsciiAnySnapshotTextGuardRefusal(at, why);
};

type stdioStlAsciiAnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioStlAsciiAnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioStlAsciiAnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioStlAsciiAnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioStlAsciiAnySnapshotTextGuardReject(at, "value is not an object");
export const stdioStlAsciiAnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioStlAsciiAnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioStlAsciiAnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioStlAsciiAnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioStlAsciiAnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioStlAsciiAnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioStlAsciiAnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioStlAsciiAnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioStlAsciiAnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioStlAsciiAnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioStlAsciiAnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioStlAsciiAnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioStlAsciiAnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioStlAsciiAnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioStlAsciiAnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioStlAsciiAnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioStlAsciiAnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioStlAsciiAnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioStlAsciiAnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioStlAsciiAnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioStlAsciiAnySnapshotTextGuardNumber(value, at, bounds) : stdioStlAsciiAnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioStlAsciiAnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioStlAsciiAnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioStlAsciiAnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioStlAsciiAnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseStlSnapshotText(value: unknown, at = "$"): StlSnapshotText {
  return stdioStlAsciiAnySnapshotTextGuardObject(value, `${at}`);
}
