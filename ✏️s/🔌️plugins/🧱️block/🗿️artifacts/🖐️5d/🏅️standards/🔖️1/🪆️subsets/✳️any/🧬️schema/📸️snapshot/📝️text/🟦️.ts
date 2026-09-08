/** 📝️ Text representation for `block.block5d.snapshot`. */
export type Block5dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dSnapshotTextGuardRefusal(at, why);
};

type blockBlock5dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dSnapshotTextGuardReject(at, "value is not an object");
export const blockBlock5dSnapshotTextGuardArray = (value: unknown, at: string, bounds: blockBlock5dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dSnapshotTextGuardString = (value: unknown, at: string, bounds: blockBlock5dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dSnapshotTextGuardReject(at, "value is not a boolean"));
export const blockBlock5dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: blockBlock5dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: blockBlock5dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dSnapshotTextGuardNumber(value, at, bounds) : blockBlock5dSnapshotTextGuardReject(at, "value is not an integer");
export const blockBlock5dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dSnapshotText(value: unknown, at = "$"): Block5dSnapshotText {
  return blockBlock5dSnapshotTextGuardObject(value, `${at}`);
}
