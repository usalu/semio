/** 📝️ Text representation for `block.block2d.mutations`. */
export type Block2dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock2dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock2dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock2dMutationsTextGuardRefusal(at, why);
};

type blockBlock2dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock2dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock2dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock2dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock2dMutationsTextGuardReject(at, "value is not an object");
export const blockBlock2dMutationsTextGuardArray = (value: unknown, at: string, bounds: blockBlock2dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock2dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock2dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock2dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock2dMutationsTextGuardString = (value: unknown, at: string, bounds: blockBlock2dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock2dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock2dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock2dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock2dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock2dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock2dMutationsTextGuardReject(at, "value is not a boolean"));
export const blockBlock2dMutationsTextGuardNumber = (value: unknown, at: string, bounds: blockBlock2dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock2dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock2dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock2dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock2dMutationsTextGuardInteger = (value: unknown, at: string, bounds: blockBlock2dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock2dMutationsTextGuardNumber(value, at, bounds) : blockBlock2dMutationsTextGuardReject(at, "value is not an integer");
export const blockBlock2dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock2dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock2dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock2dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock2dMutationsText(value: unknown, at = "$"): Block2dMutationsText {
  return blockBlock2dMutationsTextGuardObject(value, `${at}`);
}
