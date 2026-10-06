/** 📝️ Text representation for `stdio.xlsx` (snapshot). */
export type XlsxSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioXlsxEcma376BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioXlsxEcma376BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioXlsxEcma376BaseSnapshotTextGuardRefusal(at, why);
};

type stdioXlsxEcma376BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioXlsxEcma376BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioXlsxEcma376BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioXlsxEcma376BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioXlsxEcma376BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioXlsxEcma376BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioXlsxEcma376BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioXlsxEcma376BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioXlsxEcma376BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioXlsxEcma376BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioXlsxEcma376BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioXlsxEcma376BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioXlsxEcma376BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioXlsxEcma376BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioXlsxEcma376BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioXlsxEcma376BaseSnapshotTextGuardNumber(value, at, bounds) : stdioXlsxEcma376BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioXlsxEcma376BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioXlsxEcma376BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioXlsxEcma376BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseXlsxSnapshotText(value: unknown, at = "$"): XlsxSnapshotText {
  return stdioXlsxEcma376BaseSnapshotTextGuardObject(value, `${at}`);
}
