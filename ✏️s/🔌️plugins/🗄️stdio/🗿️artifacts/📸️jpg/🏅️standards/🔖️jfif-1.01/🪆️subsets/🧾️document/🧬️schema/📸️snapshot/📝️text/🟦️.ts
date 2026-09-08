/** 📝️ Text representation for `stdio.jpg` (snapshot). */
export type JpgSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioJpgJfif101DocumentSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioJpgJfif101DocumentSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioJpgJfif101DocumentSnapshotTextGuardRefusal(at, why);
};

type stdioJpgJfif101DocumentSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioJpgJfif101DocumentSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioJpgJfif101DocumentSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioJpgJfif101DocumentSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioJpgJfif101DocumentSnapshotTextGuardReject(at, "value is not an object");
export const stdioJpgJfif101DocumentSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioJpgJfif101DocumentSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioJpgJfif101DocumentSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioJpgJfif101DocumentSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioJpgJfif101DocumentSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioJpgJfif101DocumentSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioJpgJfif101DocumentSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioJpgJfif101DocumentSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioJpgJfif101DocumentSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioJpgJfif101DocumentSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioJpgJfif101DocumentSnapshotTextGuardNumber(value, at, bounds) : stdioJpgJfif101DocumentSnapshotTextGuardReject(at, "value is not an integer");
export const stdioJpgJfif101DocumentSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioJpgJfif101DocumentSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioJpgJfif101DocumentSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseJpgSnapshotText(value: unknown, at = "$"): JpgSnapshotText {
  return stdioJpgJfif101DocumentSnapshotTextGuardObject(value, `${at}`);
}
