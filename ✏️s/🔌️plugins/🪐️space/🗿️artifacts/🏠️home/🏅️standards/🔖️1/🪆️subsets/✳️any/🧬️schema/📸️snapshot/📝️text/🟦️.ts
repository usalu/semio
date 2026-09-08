/** 📝️ Text representation for `space.home.snapshot`. */
export type SHomeSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class spaceHomeSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const spaceHomeSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new spaceHomeSnapshotTextGuardRefusal(at, why);
};

type spaceHomeSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type spaceHomeSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type spaceHomeSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const spaceHomeSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : spaceHomeSnapshotTextGuardReject(at, "value is not an object");
export const spaceHomeSnapshotTextGuardArray = (value: unknown, at: string, bounds: spaceHomeSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return spaceHomeSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) spaceHomeSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) spaceHomeSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const spaceHomeSnapshotTextGuardString = (value: unknown, at: string, bounds: spaceHomeSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return spaceHomeSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) spaceHomeSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) spaceHomeSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) spaceHomeSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const spaceHomeSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : spaceHomeSnapshotTextGuardReject(at, "value is not a boolean"));
export const spaceHomeSnapshotTextGuardNumber = (value: unknown, at: string, bounds: spaceHomeSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return spaceHomeSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) spaceHomeSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) spaceHomeSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const spaceHomeSnapshotTextGuardInteger = (value: unknown, at: string, bounds: spaceHomeSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? spaceHomeSnapshotTextGuardNumber(value, at, bounds) : spaceHomeSnapshotTextGuardReject(at, "value is not an integer");
export const spaceHomeSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : spaceHomeSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const spaceHomeSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : spaceHomeSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseSHomeSnapshotText(value: unknown, at = "$"): SHomeSnapshotText {
  return spaceHomeSnapshotTextGuardObject(value, `${at}`);
}
