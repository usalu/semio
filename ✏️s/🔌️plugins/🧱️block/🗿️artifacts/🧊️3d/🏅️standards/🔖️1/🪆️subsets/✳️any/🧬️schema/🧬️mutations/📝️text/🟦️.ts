/** 📝️ Text representation for `block.block3d.mutations`. */
export type Block3dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock3dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock3dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock3dMutationsTextGuardRefusal(at, why);
};

type blockBlock3dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock3dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock3dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock3dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock3dMutationsTextGuardReject(at, "value is not an object");
export const blockBlock3dMutationsTextGuardArray = (value: unknown, at: string, bounds: blockBlock3dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock3dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock3dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock3dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock3dMutationsTextGuardString = (value: unknown, at: string, bounds: blockBlock3dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock3dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock3dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock3dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock3dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock3dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock3dMutationsTextGuardReject(at, "value is not a boolean"));
export const blockBlock3dMutationsTextGuardNumber = (value: unknown, at: string, bounds: blockBlock3dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock3dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock3dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock3dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock3dMutationsTextGuardInteger = (value: unknown, at: string, bounds: blockBlock3dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock3dMutationsTextGuardNumber(value, at, bounds) : blockBlock3dMutationsTextGuardReject(at, "value is not an integer");
export const blockBlock3dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock3dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock3dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock3dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock3dMutationsText(value: unknown, at = "$"): Block3dMutationsText {
  return blockBlock3dMutationsTextGuardObject(value, `${at}`);
}
