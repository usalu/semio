/** 📝️ Text representation for `stdio.pdf` (snapshot). */
export type PdfSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioPdf17BaseSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioPdf17BaseSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioPdf17BaseSnapshotTextGuardRefusal(at, why);
};

type stdioPdf17BaseSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioPdf17BaseSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioPdf17BaseSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioPdf17BaseSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioPdf17BaseSnapshotTextGuardReject(at, "value is not an object");
export const stdioPdf17BaseSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioPdf17BaseSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioPdf17BaseSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioPdf17BaseSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioPdf17BaseSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioPdf17BaseSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioPdf17BaseSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioPdf17BaseSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioPdf17BaseSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioPdf17BaseSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioPdf17BaseSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioPdf17BaseSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioPdf17BaseSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioPdf17BaseSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioPdf17BaseSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioPdf17BaseSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioPdf17BaseSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioPdf17BaseSnapshotTextGuardNumber(value, at, bounds) : stdioPdf17BaseSnapshotTextGuardReject(at, "value is not an integer");
export const stdioPdf17BaseSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioPdf17BaseSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioPdf17BaseSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioPdf17BaseSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parsePdfSnapshotText(value: unknown, at = "$"): PdfSnapshotText {
  return stdioPdf17BaseSnapshotTextGuardObject(value, `${at}`);
}
