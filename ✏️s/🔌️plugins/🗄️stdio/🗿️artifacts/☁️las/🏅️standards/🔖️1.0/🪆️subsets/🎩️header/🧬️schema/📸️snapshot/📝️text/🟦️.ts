/** 📝️ Text representation for `stdio.las` (snapshot). */
export type LasSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioLas10HeaderSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioLas10HeaderSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioLas10HeaderSnapshotTextGuardRefusal(at, why);
};

type stdioLas10HeaderSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioLas10HeaderSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioLas10HeaderSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioLas10HeaderSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioLas10HeaderSnapshotTextGuardReject(at, "value is not an object");
export const stdioLas10HeaderSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioLas10HeaderSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioLas10HeaderSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioLas10HeaderSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioLas10HeaderSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioLas10HeaderSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioLas10HeaderSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioLas10HeaderSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioLas10HeaderSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioLas10HeaderSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioLas10HeaderSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioLas10HeaderSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioLas10HeaderSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioLas10HeaderSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioLas10HeaderSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioLas10HeaderSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioLas10HeaderSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioLas10HeaderSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioLas10HeaderSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioLas10HeaderSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioLas10HeaderSnapshotTextGuardNumber(value, at, bounds) : stdioLas10HeaderSnapshotTextGuardReject(at, "value is not an integer");
export const stdioLas10HeaderSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioLas10HeaderSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioLas10HeaderSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioLas10HeaderSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseLasSnapshotText(value: unknown, at = "$"): LasSnapshotText {
  return stdioLas10HeaderSnapshotTextGuardObject(value, `${at}`);
}
