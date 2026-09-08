/** 📝️ Text representation for `stdio.bcf` (snapshot). */
export type BcfSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class stdioBcf21MarkupSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const stdioBcf21MarkupSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new stdioBcf21MarkupSnapshotTextGuardRefusal(at, why);
};

type stdioBcf21MarkupSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type stdioBcf21MarkupSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type stdioBcf21MarkupSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const stdioBcf21MarkupSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : stdioBcf21MarkupSnapshotTextGuardReject(at, "value is not an object");
export const stdioBcf21MarkupSnapshotTextGuardArray = (value: unknown, at: string, bounds: stdioBcf21MarkupSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return stdioBcf21MarkupSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) stdioBcf21MarkupSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) stdioBcf21MarkupSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const stdioBcf21MarkupSnapshotTextGuardString = (value: unknown, at: string, bounds: stdioBcf21MarkupSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return stdioBcf21MarkupSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) stdioBcf21MarkupSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) stdioBcf21MarkupSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) stdioBcf21MarkupSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const stdioBcf21MarkupSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : stdioBcf21MarkupSnapshotTextGuardReject(at, "value is not a boolean"));
export const stdioBcf21MarkupSnapshotTextGuardNumber = (value: unknown, at: string, bounds: stdioBcf21MarkupSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return stdioBcf21MarkupSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) stdioBcf21MarkupSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) stdioBcf21MarkupSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const stdioBcf21MarkupSnapshotTextGuardInteger = (value: unknown, at: string, bounds: stdioBcf21MarkupSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? stdioBcf21MarkupSnapshotTextGuardNumber(value, at, bounds) : stdioBcf21MarkupSnapshotTextGuardReject(at, "value is not an integer");
export const stdioBcf21MarkupSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : stdioBcf21MarkupSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const stdioBcf21MarkupSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : stdioBcf21MarkupSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBcfSnapshotText(value: unknown, at = "$"): BcfSnapshotText {
  return stdioBcf21MarkupSnapshotTextGuardObject(value, `${at}`);
}
