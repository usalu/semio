/** 📝️ Text representation for `stdio.zip` (snapshot). */
export type ZipSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioZip20BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioZip20BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioZip20BaseSnapshotTextGuardRefusal(at, why);
};

type stdioZip20BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioZip20BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioZip20BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioZip20BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioZip20BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioZip20BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioZip20BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioZip20BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioZip20BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioZip20BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioZip20BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioZip20BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioZip20BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioZip20BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioZip20BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioZip20BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioZip20BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioZip20BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioZip20BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioZip20BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioZip20BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioZip20BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioZip20BaseSnapshotTextGuardNumber(value, at, bounds) : stdioZip20BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioZip20BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioZip20BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioZip20BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioZip20BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseZipSnapshotText(value: unknown, at = "$"): ZipSnapshotText {
  return stdioZip20BaseSnapshotTextGuardObject(value, `${at}`);
}
