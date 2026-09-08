/** 📝️ Text representation for `block.block3d.snapshot`. */
export type Block3dSnapshotText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dSnapshotTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dSnapshotTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dSnapshotTextGuardRefusal(at, why);
};

type blockBlock3dSnapshotTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dSnapshotTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dSnapshotTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dSnapshotTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dSnapshotTextGuardReject(at, "value is not an object");
export const blockBlock3dSnapshotTextGuardArray = (value: unknown, at: string, bounds: blockBlock3dSnapshotTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dSnapshotTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dSnapshotTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dSnapshotTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dSnapshotTextGuardString = (value: unknown, at: string, bounds: blockBlock3dSnapshotTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dSnapshotTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dSnapshotTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dSnapshotTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dSnapshotTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dSnapshotTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dSnapshotTextGuardReject(at, "value is not a boolean"));
export const blockBlock3dSnapshotTextGuardNumber = (value: unknown, at: string, bounds: blockBlock3dSnapshotTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dSnapshotTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dSnapshotTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dSnapshotTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dSnapshotTextGuardInteger = (value: unknown, at: string, bounds: blockBlock3dSnapshotTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dSnapshotTextGuardNumber(value, at, bounds) : blockBlock3dSnapshotTextGuardReject(at, "value is not an integer");
export const blockBlock3dSnapshotTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dSnapshotTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dSnapshotTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dSnapshotTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dSnapshotText(value: unknown, at = "$"): Block3dSnapshotText {
  return blockBlock3dSnapshotTextGuardObject(value, `${at}`);
}
