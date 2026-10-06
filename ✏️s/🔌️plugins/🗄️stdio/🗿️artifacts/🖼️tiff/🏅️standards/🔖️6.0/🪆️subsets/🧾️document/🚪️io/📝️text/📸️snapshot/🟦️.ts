/** 📝️ Text representation for `stdio.tiff` (snapshot). */
export type TiffSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioTiff60DocumentSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioTiff60DocumentSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioTiff60DocumentSnapshotTextGuardRefusal(at, why);
};

type stdioTiff60DocumentSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioTiff60DocumentSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioTiff60DocumentSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioTiff60DocumentSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioTiff60DocumentSnapshotTextGuardReject(at, "value is not an object");
export const stdioTiff60DocumentSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioTiff60DocumentSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioTiff60DocumentSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioTiff60DocumentSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioTiff60DocumentSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioTiff60DocumentSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioTiff60DocumentSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioTiff60DocumentSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioTiff60DocumentSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioTiff60DocumentSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioTiff60DocumentSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioTiff60DocumentSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioTiff60DocumentSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioTiff60DocumentSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioTiff60DocumentSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioTiff60DocumentSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioTiff60DocumentSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioTiff60DocumentSnapshotTextGuardNumber(value, at, bounds) : stdioTiff60DocumentSnapshotTextGuardReject(at, "value is not an integer");
export const stdioTiff60DocumentSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioTiff60DocumentSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioTiff60DocumentSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioTiff60DocumentSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseTiffSnapshotText(value: unknown, at = "$"): TiffSnapshotText {
  return stdioTiff60DocumentSnapshotTextGuardObject(value, `${at}`);
}
