/** 📝️ Text representation for `stdio.dwg` (snapshot). */
export type DwgSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDwgAc1024AnySnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDwgAc1024AnySnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioDwgAc1024AnySnapshotTextGuardRefusal(at, why);
};

type stdioDwgAc1024AnySnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDwgAc1024AnySnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDwgAc1024AnySnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDwgAc1024AnySnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDwgAc1024AnySnapshotTextGuardReject(at, "value is not an object");
export const stdioDwgAc1024AnySnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioDwgAc1024AnySnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDwgAc1024AnySnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDwgAc1024AnySnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDwgAc1024AnySnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDwgAc1024AnySnapshotTextGuardString = (value: unknown, at: string, bounds: stdioDwgAc1024AnySnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDwgAc1024AnySnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDwgAc1024AnySnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDwgAc1024AnySnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDwgAc1024AnySnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDwgAc1024AnySnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDwgAc1024AnySnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioDwgAc1024AnySnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioDwgAc1024AnySnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDwgAc1024AnySnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDwgAc1024AnySnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDwgAc1024AnySnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDwgAc1024AnySnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioDwgAc1024AnySnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDwgAc1024AnySnapshotTextGuardNumber(value, at, bounds) : stdioDwgAc1024AnySnapshotTextGuardReject(at, "value is not an integer");
export const stdioDwgAc1024AnySnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDwgAc1024AnySnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDwgAc1024AnySnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDwgAc1024AnySnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDwgSnapshotText(value: unknown, at = "$"): DwgSnapshotText {
  return stdioDwgAc1024AnySnapshotTextGuardObject(value, `${at}`);
}
