/** 📝️ Text representation for `block.block5d.inference`. */
export type Block5dInferenceText = string;

//#region 🚪️Parsers
/** 🚪️ Refusal of one instance position, the shape every `parse<Export>` below rejects with. */
export class blockBlock5dInferenceTextGuardRefusal extends Error {
  constructor(readonly at: string, readonly why: string) {
    super(`${at}: ${why}`);
  }
}

const blockBlock5dInferenceTextGuardReject = (at: string, why: string): never => {
  throw new blockBlock5dInferenceTextGuardRefusal(at, why);
};

type blockBlock5dInferenceTextGuardTextBounds = { readonly minLength?: number; readonly maxLength?: number; readonly pattern?: string };
type blockBlock5dInferenceTextGuardRangeBounds = { readonly minimum?: number; readonly maximum?: number };
type blockBlock5dInferenceTextGuardSizeBounds = { readonly minItems?: number; readonly maxItems?: number };

export const blockBlock5dInferenceTextGuardObject = (value: unknown, at: string): Readonly<Record<string, unknown>> =>
  value !== null && typeof value === "object" && !Array.isArray(value) ? (value as Record<string, unknown>) : blockBlock5dInferenceTextGuardReject(at, "value is not an object");
export const blockBlock5dInferenceTextGuardArray = (value: unknown, at: string, bounds: blockBlock5dInferenceTextGuardSizeBounds = {}): readonly unknown[] => {
  if (!Array.isArray(value)) return blockBlock5dInferenceTextGuardReject(at, "value is not an array");
  if (bounds.minItems !== undefined && value.length < bounds.minItems) blockBlock5dInferenceTextGuardReject(at, `array has fewer than ${bounds.minItems} items`);
  if (bounds.maxItems !== undefined && value.length > bounds.maxItems) blockBlock5dInferenceTextGuardReject(at, `array has more than ${bounds.maxItems} items`);
  return value;
};
export const blockBlock5dInferenceTextGuardString = (value: unknown, at: string, bounds: blockBlock5dInferenceTextGuardTextBounds = {}): string => {
  if (typeof value !== "string") return blockBlock5dInferenceTextGuardReject(at, "value is not a string");
  const length = [...value].length;
  if (bounds.minLength !== undefined && length < bounds.minLength) blockBlock5dInferenceTextGuardReject(at, `string is shorter than ${bounds.minLength}`);
  if (bounds.maxLength !== undefined && length > bounds.maxLength) blockBlock5dInferenceTextGuardReject(at, `string is longer than ${bounds.maxLength}`);
  if (bounds.pattern !== undefined && !new RegExp(bounds.pattern, "u").test(value)) blockBlock5dInferenceTextGuardReject(at, `string does not match ${bounds.pattern}`);
  return value;
};
export const blockBlock5dInferenceTextGuardBoolean = (value: unknown, at: string): boolean => (typeof value === "boolean" ? value : blockBlock5dInferenceTextGuardReject(at, "value is not a boolean"));
export const blockBlock5dInferenceTextGuardNumber = (value: unknown, at: string, bounds: blockBlock5dInferenceTextGuardRangeBounds = {}): number => {
  if (typeof value !== "number" || !Number.isFinite(value)) return blockBlock5dInferenceTextGuardReject(at, "value is not a finite number");
  if (bounds.minimum !== undefined && value < bounds.minimum) blockBlock5dInferenceTextGuardReject(at, `number is below ${bounds.minimum}`);
  if (bounds.maximum !== undefined && value > bounds.maximum) blockBlock5dInferenceTextGuardReject(at, `number is above ${bounds.maximum}`);
  return value;
};
export const blockBlock5dInferenceTextGuardInteger = (value: unknown, at: string, bounds: blockBlock5dInferenceTextGuardRangeBounds = {}): number =>
  Number.isSafeInteger(value) ? blockBlock5dInferenceTextGuardNumber(value, at, bounds) : blockBlock5dInferenceTextGuardReject(at, "value is not an integer");
export const blockBlock5dInferenceTextGuardMember = <T extends string>(value: unknown, at: string, members: readonly T[]): T =>
  members.includes(value as T) ? (value as T) : blockBlock5dInferenceTextGuardReject(at, `value is not one of ${members.join(", ")}`);
export const blockBlock5dInferenceTextGuardConstant = <T extends string | number | boolean>(value: unknown, at: string, expected: T): T =>
  value === expected ? expected : blockBlock5dInferenceTextGuardReject(at, `value is not ${String(expected)}`);
//#endregion 🚪️Parsers

export function parseBlock5dInferenceText(value: unknown, at = "$"): Block5dInferenceText {
  return blockBlock5dInferenceTextGuardObject(value, `${at}`);
}
