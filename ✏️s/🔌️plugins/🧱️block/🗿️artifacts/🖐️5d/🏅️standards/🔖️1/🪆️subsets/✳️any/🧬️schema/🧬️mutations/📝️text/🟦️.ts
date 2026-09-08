/** 📝️ Text representation for `block.block5d.mutations`. */
export type Block5dMutationsText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dMutationsTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dMutationsTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dMutationsTextGuardRefusal(at, why);
};

type blockBlock5dMutationsTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dMutationsTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dMutationsTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dMutationsTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dMutationsTextGuardReject(at, "value is not an object");
export const blockBlock5dMutationsTextGuardArray = (value: unknown, at: string, bounds: blockBlock5dMutationsTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dMutationsTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dMutationsTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dMutationsTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dMutationsTextGuardString = (value: unknown, at: string, bounds: blockBlock5dMutationsTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dMutationsTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dMutationsTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dMutationsTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dMutationsTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dMutationsTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dMutationsTextGuardReject(at, "value is not a boolean"));
export const blockBlock5dMutationsTextGuardNumber = (value: unknown, at: string, bounds: blockBlock5dMutationsTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dMutationsTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dMutationsTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dMutationsTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dMutationsTextGuardInteger = (value: unknown, at: string, bounds: blockBlock5dMutationsTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dMutationsTextGuardNumber(value, at, bounds) : blockBlock5dMutationsTextGuardReject(at, "value is not an integer");
export const blockBlock5dMutationsTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dMutationsTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dMutationsTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dMutationsTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dMutationsText(value: unknown, at = "$"): Block5dMutationsText {
  return blockBlock5dMutationsTextGuardObject(value, `${at}`);
}
