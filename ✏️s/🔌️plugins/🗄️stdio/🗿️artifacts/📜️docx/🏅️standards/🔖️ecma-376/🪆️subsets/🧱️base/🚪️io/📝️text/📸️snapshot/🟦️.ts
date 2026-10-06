/** 📝️ Text representation for `stdio.docx` (snapshot). */
export type DocxSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDocxEcma376BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDocxEcma376BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioDocxEcma376BaseSnapshotTextGuardRefusal(at, why);
};

type stdioDocxEcma376BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDocxEcma376BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDocxEcma376BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDocxEcma376BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDocxEcma376BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioDocxEcma376BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioDocxEcma376BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDocxEcma376BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDocxEcma376BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioDocxEcma376BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDocxEcma376BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDocxEcma376BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDocxEcma376BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioDocxEcma376BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioDocxEcma376BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDocxEcma376BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDocxEcma376BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDocxEcma376BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioDocxEcma376BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDocxEcma376BaseSnapshotTextGuardNumber(value, at, bounds) : stdioDocxEcma376BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioDocxEcma376BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDocxEcma376BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDocxEcma376BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDocxEcma376BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDocxSnapshotText(value: unknown, at = "$"): DocxSnapshotText {
  return stdioDocxEcma376BaseSnapshotTextGuardObject(value, `${at}`);
}
