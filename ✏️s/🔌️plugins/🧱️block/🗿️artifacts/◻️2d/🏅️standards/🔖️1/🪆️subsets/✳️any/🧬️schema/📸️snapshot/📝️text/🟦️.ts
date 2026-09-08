/** 📝️ Text representation for `block.block2d.snapshot`. */
export type Block2dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dSnapshotTextGuardRefusal(at, why);
};

type blockBlock2dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dSnapshotTextGuardReject(at, "value is not an object");
export const blockBlock2dSnapshotTextGuardArray = (value: unknown, at: string, bounds: blockBlock2dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dSnapshotTextGuardString = (value: unknown, at: string, bounds: blockBlock2dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dSnapshotTextGuardReject(at, "value is not a boolean"));
export const blockBlock2dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: blockBlock2dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: blockBlock2dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dSnapshotTextGuardNumber(value, at, bounds) : blockBlock2dSnapshotTextGuardReject(at, "value is not an integer");
export const blockBlock2dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dSnapshotText(value: unknown, at = "$"): Block2dSnapshotText {
  return blockBlock2dSnapshotTextGuardObject(value, `${at}`);
}
