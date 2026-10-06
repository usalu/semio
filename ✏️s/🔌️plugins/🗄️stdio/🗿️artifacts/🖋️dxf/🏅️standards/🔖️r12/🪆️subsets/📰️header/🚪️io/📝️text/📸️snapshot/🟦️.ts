/** 📝️ Text representation for `stdio.dxf` (snapshot). */
export type DxfSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioDxfR12HeaderSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioDxfR12HeaderSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioDxfR12HeaderSnapshotTextGuardRefusal(at, why);
};

type stdioDxfR12HeaderSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioDxfR12HeaderSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioDxfR12HeaderSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioDxfR12HeaderSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioDxfR12HeaderSnapshotTextGuardReject(at, "value is not an object");
export const stdioDxfR12HeaderSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioDxfR12HeaderSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioDxfR12HeaderSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioDxfR12HeaderSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioDxfR12HeaderSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioDxfR12HeaderSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioDxfR12HeaderSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioDxfR12HeaderSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioDxfR12HeaderSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioDxfR12HeaderSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioDxfR12HeaderSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioDxfR12HeaderSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioDxfR12HeaderSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioDxfR12HeaderSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioDxfR12HeaderSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioDxfR12HeaderSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioDxfR12HeaderSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioDxfR12HeaderSnapshotTextGuardNumber(value, at, bounds) : stdioDxfR12HeaderSnapshotTextGuardReject(at, "value is not an integer");
export const stdioDxfR12HeaderSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioDxfR12HeaderSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioDxfR12HeaderSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioDxfR12HeaderSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseDxfSnapshotText(value: unknown, at = "$"): DxfSnapshotText {
  return stdioDxfR12HeaderSnapshotTextGuardObject(value, `${at}`);
}
