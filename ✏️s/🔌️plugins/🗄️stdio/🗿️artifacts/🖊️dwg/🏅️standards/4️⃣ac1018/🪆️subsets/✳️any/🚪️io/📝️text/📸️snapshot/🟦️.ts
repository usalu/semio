/** 📝️ Text representation for `stdio.dwg` (snapshot). */
export type DwgSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1018AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1018AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1018AnySnapshotTextGuardRefusal(at, why);
};

type stdioDwgAc1018AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1018AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1018AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1018AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1018AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioDwgAc1018AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1018AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1018AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1018AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1018AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1018AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioDwgAc1018AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1018AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1018AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1018AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1018AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1018AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1018AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1018AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1018AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1018AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1018AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1018AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1018AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1018AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1018AnySnapshotTextGuardNumber(value, at, bounds) : stdioDwgAc1018AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioDwgAc1018AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1018AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1018AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1018AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDwgSnapshotText(value: unknown, at = "$"): DwgSnapshotText {
  return stdioDwgAc1018AnySnapshotTextGuardObject(value, `${at}`);
}
